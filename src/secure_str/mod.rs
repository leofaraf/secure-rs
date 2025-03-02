use std::io::Write;

use base64::{prelude::BASE64_STANDARD, Engine};
use params::SecureStringBrotliParams;
use proc_macro::TokenStream;
use proc_macro2::{Span, Ident};
use syn::parse_macro_input;

mod params;

pub fn brotli(_item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(_item as SecureStringBrotliParams);

    let span = Span::call_site();
    let function_name = Ident::new(&format!("get_{}", parsed.name), span);

    let span = Span::call_site();
    let mod_name = Ident::new(&format!("mod_{}", parsed.name), span);

    let encoded_value = encode_brotli(parsed.value);

    quote::quote! {
        mod #mod_name {
            use base64::{prelude::BASE64_STANDARD, Engine};
            use std::io::Write;
    
            pub fn #function_name() -> String {
                let mut decompressed_data = Vec::new();
                {
                    let data = BASE64_STANDARD.decode(#encoded_value.as_bytes()).unwrap_or_default();
                    let mut decompressor = brotli::DecompressorWriter::new(&mut decompressed_data, 4096);
                    decompressor.write_all(&data).unwrap();
                }
                String::from_utf8(decompressed_data).unwrap()
            }
        }

        pub use #mod_name::#function_name;
    }.into()
}

/// Accepts raw string, returns BASE64 string
fn encode_brotli(value: String) -> String {
    let mut decompressed_data = Vec::new();
    {
        let mut decompressor = brotli::CompressorWriter::new(&mut decompressed_data, 4096, 0, 0);
        decompressor.write_all(&value.as_bytes()).unwrap();
    }
    BASE64_STANDARD.encode(decompressed_data)
}