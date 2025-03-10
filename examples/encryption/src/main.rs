use secure::include_secure_str_aes;
use secure::secure_str_aes;
use secure::include_secure_bytes_aes;

include_secure_str_aes!("../../samples/loremipsum.txt", "Thisi$MyKeyT0Encryp!thislastTime", "loremipsum");
secure_str_aes!("Thisi$MyKeyT0Encryp!thislastTime", "api_key", "-----BEGIN PUBLIC KEY-----
MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGQlFVujnjLZ2Ln+JjvcSu3cOiMB
JrKb3G49Ivg3Mvefp+D+UHN5OV2AcN1c0znkIzk/QgqxJ6VKaZtqjKMhZWG/0mta
niD36GTJTnUZ5MWQ2dVlItO6gtM/61uIf4FNZaLJG4CReoHkH61ffl7Fz7B9aroj
8+MhcMLInbWDQ6sJAgMBAAE=
-----END PUBLIC KEY-----");
include_secure_bytes_aes!("../../samples/image.png", "Thisi$MyKeyT0Encryp!thislastTime", "bytes");

fn main() {
    println!("{}", get_loremipsum());
    // output:
    // Lorem ipsum odor amet, consectetuer adipiscing elit.
    // Lorem ipsum odor amet, consectetuer adipiscing elit.
    // Lorem ipsum odor amet, consectetuer adipiscing elit.
    // Lorem ipsum odor amet, consectetuer adipiscing elit.
    // Lorem ipsum odor amet, consectetuer adipiscing elit.
    // Lorem ipsum odor amet, consectetuer adipiscing elit.
    // Lorem ipsum odor amet, consectetuer adipiscing elit.
    // Lorem ipsum odor amet, consectetuer adipiscing elit.

    println!();

    println!("{}", get_api_key());
    // output:
    // -----BEGIN PUBLIC KEY-----
    // MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGQlFVujnjLZ2Ln+JjvcSu3cOiMB
    // JrKb3G49Ivg3Mvefp+D+UHN5OV2AcN1c0znkIzk/QgqxJ6VKaZtqjKMhZWG/0mta
    // niD36GTJTnUZ5MWQ2dVlItO6gtM/61uIf4FNZaLJG4CReoHkH61ffl7Fz7B9aroj
    // 8+MhcMLInbWDQ6sJAgMBAAE=
    // -----END PUBLIC KEY-----

    println!();

    println!("{}", get_bytes().len());
    // output: lenght of image
}