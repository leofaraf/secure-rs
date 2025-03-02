# secure-rs
[![Crates.io](https://img.shields.io/crates/v/diesel.svg)](https://crates.io/crates/diesel)

### Usage:
```rust
use secure::secure_str_brotli;

secure_str_brotli!("brotli_value", "decompressed");

#[test]
fn secure_str_brotli() {
    assert_eq!(get_brotli_value(), "decompressed".to_string());
}
```

Add to dependencies
```toml
base64 = "0.22.1"
brotli = "7.0.0"
secure = "0.1.1"
```
