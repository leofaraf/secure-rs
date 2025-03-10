mod params;

#[cfg(feature = "compression")]
mod compression {
    use std::fs::read_to_string;

    use super::params::IncludeSecureStringBrotliParams;
    use proc_macro::TokenStream;
    use proc_macro2::{Span, Ident};
    use syn::parse_macro_input;

    pub fn brotli(_item: TokenStream) -> TokenStream {
        let parsed = parse_macro_input!(_item as IncludeSecureStringBrotliParams);
    
        let span = Span::call_site();
        let function_name = Ident::new(&format!("get_{}", parsed.name), span);
    
        let span = Span::call_site();
        let mod_name = Ident::new(&format!("mod_{}", parsed.name), span);
    
        let value = read_to_string(parsed.path).expect("Cannot raed string from such path".into());
    
        let encoded_value = crate::backend::brotli::encode_brotli(&value.as_bytes());
    
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
    use std::fs::read_to_string;

    use super::params::IncludeSecureStringAesParams;
    use proc_macro::TokenStream;
    use proc_macro2::{Span, Ident};
    use syn::parse_macro_input;


    #[cfg(feature = "encryption")]
    pub fn aes(_item: TokenStream) -> TokenStream {
        let parsed = parse_macro_input!(_item as IncludeSecureStringAesParams);

        let span = Span::call_site();
        let function_name = Ident::new(&format!("get_{}", parsed.name), span);

        let span = Span::call_site();
        let mod_name = Ident::new(&format!("mod_{}", parsed.name), span);

        let value = read_to_string(parsed.path).expect("Cannot raed string from such path".into());

        let encoded_value = crate::backend::aes::encode_aes(
            &parsed.key,
            &value.as_bytes()
        );
        let key = parsed.key;

        quote::quote! {
            mod #mod_name {
                use base64::{prelude::BASE64_STANDARD, Engine};
                use byte_aes::Aes256Cryptor;
                
                pub fn #function_name() -> String {
                    let decoded_data = {
                        let value = BASE64_STANDARD.decode(#encoded_value.as_bytes()).unwrap();
                        let cryptor = Aes256Cryptor::try_from(#key).unwrap();
                        cryptor.decrypt(value).expect("Can't decrypt data")
                    };
                
                    String::from_utf8(decoded_data).unwrap()
                }
            }

            pub use #mod_name::#function_name;
        }.into()
    }
}

#[cfg(feature = "encryption")]
pub use encryption::aes;