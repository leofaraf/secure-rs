# secure-rs
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/leofaraf/secure-rs/ci.yml?logo=github&label=CI%20tests)](https://github.com/leofaraf/secure-rs/actions)
[![Static Badge](https://img.shields.io/crates/v/secure)](https://crates.io/crates/secure)

| Proc-macro name | Args | Description | Example | Feature |
| ------------- | ------------- | ------------- | ------------- | ------------- |
| include_secure_str_brotli  | pathname, varname** | equivalent of `include_str!()`, but compress in c-t* via brotli | [here](examples/compression/src/main.rs) | `compression` |
| secure_str_brotli  | varname**, str value  | equivalent of `const NAME: &str`, but compress in c-t* via brotli | [here](examples/compression/src/main.rs) | `compression` |
| include_secure_bytes_brotli  | pathname, varname** | equivalent of `include_bytes!()`, but compress in c-t* via brotli | [here](examples/compression/src/main.rs) | `compression` |
| include_secure_bytes_brotli  | aes_key, pathname, varname** | equivalent of `include_str!()`, but compress in c-t* via aes | - | `encryption` |
| secure_str_aes | aes_key, varname**, str value  | equivalent of `const NAME: &str`, but encrypt in c-t* via aes | - | `encryption` |
| include_secure_bytes_brotli  | aes_key, pathname, varname** | equivalent of `include_bytes!()`, but compress in c-t* via aes | - | `encryption` |

c-t* — compile-time

varname** — each macro accept varname and as result generate function with name: `get_{varname}`

### Results of usage:
#### Non-secure string:
```rust
const API_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGQlFVujnjLZ2Ln+JjvcSu3cOiMB
JrKb3G49Ivg3Mvefp+D+UHN5OV2AcN1c0znkIzk/QgqxJ6VKaZtqjKMhZWG/0mta
niD36GTJTnUZ5MWQ2dVlItO6gtM/61uIf4FNZaLJG4CReoHkH61ffl7Fz7B9aroj
8+MhcMLInbWDQ6sJAgMBAAE=
-----END PUBLIC KEY-----";

fn main() {
    println!("{}", API_KEY)
}
```

binary:

![image](https://github.com/user-attachments/assets/6252202a-9e34-45c4-904e-fa6874a0df7a)

#### Secure string
```rust
use secure::secure_str_brotli;

secure_str_brotli!("api_key", "-----BEGIN PUBLIC KEY-----
MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGQlFVujnjLZ2Ln+JjvcSu3cOiMB
JrKb3G49Ivg3Mvefp+D+UHN5OV2AcN1c0znkIzk/QgqxJ6VKaZtqjKMhZWG/0mta
niD36GTJTnUZ5MWQ2dVlItO6gtM/61uIf4FNZaLJG4CReoHkH61ffl7Fz7B9aroj
8+MhcMLInbWDQ6sJAgMBAAE=
-----END PUBLIC KEY-----");

fn main() {
    println!("{}", get_api_key())
    // output:
    // -----BEGIN PUBLIC KEY-----
    // MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGQlFVujnjLZ2Ln+JjvcSu3cOiMB
    // JrKb3G49Ivg3Mvefp+D+UHN5OV2AcN1c0znkIzk/QgqxJ6VKaZtqjKMhZWG/0mta
    // niD36GTJTnUZ5MWQ2dVlItO6gtM/61uIf4FNZaLJG4CReoHkH61ffl7Fz7B9aroj
    // 8+MhcMLInbWDQ6sJAgMBAAE=
    // -----END PUBLIC KEY-----
}
```

binary:

![image](https://github.com/user-attachments/assets/0f7ebc82-660f-44ec-8366-8068faab71d6)

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
secure = { version = "0.1.10", features = ["compression", "encryption"] }
```
