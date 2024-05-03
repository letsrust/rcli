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
