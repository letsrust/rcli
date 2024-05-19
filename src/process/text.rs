use crate::{process_genpass, TextSignFormat};
use anyhow::Result;
use chacha20poly1305::aead::generic_array::typenum::Unsigned;
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::aead::{Aead, AeadCore, KeyInit};
use chacha20poly1305::ChaCha20Poly1305;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::io::Read;

pub trait TextSigner {
    // signer could sign any input data.
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerifier {
    // verifier could verify any input data.
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes() == signature)
    }
}

impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (&signature[..64]).try_into()?;
        let sig = Signature::from_bytes(sig);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl Ed25519Signer {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(key);
        Self { key }
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec());
        map.insert("ed25519.pk", pk.to_bytes().to_vec());

        Ok(map)
    }
}

impl Ed25519Verifier {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

pub struct ChaCha20 {
    key: Vec<u8>,
}

impl ChaCha20 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        Ok(Self::new(key))
    }

    pub fn new(key: &[u8]) -> Self {
        Self { key: key.to_vec() }
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng).to_vec();
        let mut map = HashMap::new();
        map.insert("chacha20.txt", key);
        Ok(map)
    }

    fn encrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let key = GenericArray::from_slice(&self.key);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let mut obsf = cipher.encrypt(&nonce, buf.as_slice()).unwrap();
        obsf.splice(..0, nonce.iter().copied());
        Ok(obsf)
    }

    fn decrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let s = String::from_utf8_lossy(&buf);
        let s = s.trim();
        let buf = s.as_bytes().to_vec();

        let bytes = hex::decode(buf).unwrap();
        println!("{:?}", bytes);

        type NonceSize = <ChaCha20Poly1305 as AeadCore>::NonceSize;
        let key = GenericArray::from_slice(&self.key);
        let cipher = ChaCha20Poly1305::new(key);
        let (nonce, ciphertext) = bytes.split_at(NonceSize::to_usize());
        let nonce = GenericArray::from_slice(nonce);

        let plaintext = cipher.decrypt(nonce, ciphertext).unwrap();
        Ok(plaintext)
    }
}

pub fn process_text_sign(
    reader: &mut dyn Read,
    key: &[u8],
    format: TextSignFormat,
) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
        _ => Err(anyhow::anyhow!("Unsupported format to sign"))?,
    };

    signer.sign(reader)
}

pub fn process_text_verify(
    reader: &mut dyn Read,
    key: &[u8],
    signature: &[u8],
    format: TextSignFormat,
) -> Result<bool> {
    let verifier: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
        _ => Err(anyhow::anyhow!("Unsupported format to verify"))?,
    };

    verifier.verify(reader, signature)
}

pub fn process_text_keygen(format: TextSignFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
        TextSignFormat::ChaCha20 => ChaCha20::generate(),
    }
}

pub fn process_text_encrypt(reader: &mut dyn Read, key: &[u8]) -> Result<Vec<u8>> {
    let cc = ChaCha20::try_new(key)?;
    cc.encrypt(reader)
}

pub fn process_text_decrypt(reader: &mut dyn Read, key: &[u8]) -> Result<Vec<u8>> {
    let cc = ChaCha20::try_new(key)?;
    cc.decrypt(reader)
}
