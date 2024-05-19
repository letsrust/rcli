# rcli

Rust CLI tools.

## Dependencies

```bash
# add clap to cargo.toml
cargo add clap --features derive

# add anyhow to cargo.toml
cargo add anyhow

cargo add csv
cargo add serde --features derive
cargo add serde_json
```

## Run

```bash
# cargo run to test
cargo run -- csv -i assets/juventus.csv -o output.json --header -d ','
```

### Text encrypt / decrypt

```shell
# generate key
cargo run -- text generate --format chacha20 --output-path fixtures

# encrypt
cargo run -- text encrypt --key fixtures/chacha20.txt --input README.md

# decrypt
cargo run -- text decrypt --key fixtures/chacha20.txt --input fixtures/textencrypt.txt
```

### Jwt

```
# Key is hello
cargo run -- jwt sign --sub abc --exp 1d --aud aaa
cargo run -- jwt verify -t ...
```
