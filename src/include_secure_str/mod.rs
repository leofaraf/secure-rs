use std::fs::read_to_string;

use params::IncludeSecureStringBrotliParams;
use proc_macro::TokenStream;
use proc_macro2::{Span, Ident};
use syn::parse_macro_input;

mod params;

pub fn brotli(_item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(_item as IncludeSecureStringBrotliParams);

    let span = Span::call_site();
    let function_name = Ident::new(&format!("get_{}", parsed.name), span);

    let span = Span::call_site();
    let mod_name = Ident::new(&format!("mod_{}", parsed.name), span);

    let value = read_to_string(parsed.path).unwrap_or("Cannot raed string from such path".into());

    let encoded_value = crate::backend::brotli::encode_brotli(value);

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