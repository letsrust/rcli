use anyhow::Result;
use hmac::{Hmac, Mac};
use jwt::{Error, SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn process_jwt_sign(sub: &str, aud: &str, exp: &str) -> Result<String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"hello")?;
    let mut claims = BTreeMap::new();
    claims.insert("sub", sub);
    claims.insert("aud", aud);
    claims.insert("exp", exp);
    let token_str = claims.sign_with_key(&key)?;
    // Ok(token_str.as_bytes().to_vec())
    Ok(token_str)
}

pub fn process_jwt_verify(token: String) -> Result<bool> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"hello")?;
    let claims: Result<BTreeMap<String, String>, Error> = token.verify_with_key(&key);
    // println!("{:?}", claims);
    match claims {
        Ok(c) => {
            println!("{:?}", c["sub"]);
            Ok(true)
        }
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_jwt_sign() -> Result<()> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"hello")?;
        let mut claims = BTreeMap::new();
        claims.insert("sub", "abc");
        claims.insert("aud", "aaa");
        claims.insert("exp", "1d");
        let token_str = claims.sign_with_key(&key)?;
        println!("{}", token_str);

        let token2 = process_jwt_sign("abc", "aaa", "1d")?;
        println!("{}", token2);

        let key: Hmac<Sha256> = Hmac::new_from_slice(b"hello")?;
        // let token_str = "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOls5Nyw5Nyw5N10sImV4cCI6WzQ5LDEwMF0sInN1YiI6Wzk3LDk4LDk5XX0.pOWz9LkNS4PzReXGBTJwEeoxQ-p_Q-xU3Yc2jpV1xgg";
        let claims: BTreeMap<String, String> = token_str.verify_with_key(&key)?;
        assert_eq!(claims["sub"], "abc");
        Ok(())
    }
}
