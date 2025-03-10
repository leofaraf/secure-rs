mod params;

#[cfg(feature = "compression")]
mod compression {
    use super::params::SecureStringBrotliParams;
    use proc_macro::TokenStream;
    use proc_macro2::{Span, Ident};
    use syn::parse_macro_input;


    #[cfg(feature = "compression")]
    pub fn brotli(_item: TokenStream) -> TokenStream {
        let parsed = parse_macro_input!(_item as SecureStringBrotliParams);

        let span = Span::call_site();
        let function_name = Ident::new(&format!("get_{}", parsed.name), span);

        let span = Span::call_site();
        let mod_name = Ident::new(&format!("mod_{}", parsed.name), span);

        let encoded_value = crate::backend::brotli::encode_brotli(&parsed.value.as_bytes());

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
}

#[cfg(feature = "compression")]
pub use compression::brotli;

#[cfg(feature = "encryption")]
mod encryption {
    use super::params::SecureStringAesParams;
    use proc_macro::TokenStream;
    use proc_macro2::{Span, Ident};
    use syn::parse_macro_input;


    #[cfg(feature = "encryption")]
    pub fn aes(_item: TokenStream) -> TokenStream {
        use base64::{prelude::BASE64_STANDARD, Engine};

        let parsed = parse_macro_input!(_item as SecureStringAesParams);

        let span = Span::call_site();
        let function_name = Ident::new(&format!("get_{}", parsed.name), span);

        let span = Span::call_site();
        let mod_name = Ident::new(&format!("mod_{}", parsed.name), span);

        let encoded_value = crate::backend::aes::encode_aes(
            &BASE64_STANDARD.decode(parsed.key).expect("Can't decode AES key"),
            &parsed.value.as_bytes()
        );

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
}

#[cfg(feature = "encryption")]
pub use encryption::aes;
