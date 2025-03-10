mod params;

#[cfg(feature = "compression")]
mod compression {
    use std::{fs::{read_to_string, File}, io::Read};

    use super::params::IncludeSecureBytesBrotliParams;
    use proc_macro::TokenStream;
    use proc_macro2::{Span, Ident};
    use syn::parse_macro_input;

    pub fn brotli(_item: TokenStream) -> TokenStream {
        let parsed = parse_macro_input!(_item as IncludeSecureBytesBrotliParams);
    
        let span = Span::call_site();
        let function_name = Ident::new(&format!("get_{}", parsed.name), span);
    
        let span = Span::call_site();
        let mod_name = Ident::new(&format!("mod_{}", parsed.name), span);
    
        let value = {
            let mut buf = vec![];
            let mut file = File::open(parsed.path).expect("Cannot open file");
            file.read_to_end(&mut buf).expect("Cannot read file");
            buf
        };
    
        let encoded_value = crate::backend::brotli::encode_brotli(&value);
    
        quote::quote! {
            mod #mod_name {
                use base64::{prelude::BASE64_STANDARD, Engine};
                use std::io::Write;
        
                pub fn #function_name() -> Vec<u8> {
                    let mut decompressed_data = Vec::new();
                    {
                        let data = BASE64_STANDARD.decode(#encoded_value.as_bytes()).unwrap_or_default();
                        let mut decompressor = brotli::DecompressorWriter::new(&mut decompressed_data, 4096);
                        decompressor.write_all(&data).unwrap();
                    }
                    decompressed_data
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

    use super::params::IncludeSecureBytesAesParams;
    use proc_macro::TokenStream;
    use proc_macro2::{Span, Ident};
    use syn::parse_macro_input;


    #[cfg(feature = "encryption")]
    pub fn aes(_item: TokenStream) -> TokenStream {
        use std::{fs::File, io::Read};

        let parsed = parse_macro_input!(_item as IncludeSecureBytesAesParams);

        let span = Span::call_site();
        let function_name = Ident::new(&format!("get_{}", parsed.name), span);

        let span = Span::call_site();
        let mod_name = Ident::new(&format!("mod_{}", parsed.name), span);

        let value = {
            let mut buf = vec![];
            let mut file = File::open(parsed.path).expect("Cannot open file");
            file.read_to_end(&mut buf).expect("Cannot read file");
            buf
        };
        
        let encoded_value = crate::backend::aes::encode_aes(
            &parsed.key,
            &value
        );
        let key = parsed.key;

        quote::quote! {
            mod #mod_name {
                use base64::{prelude::BASE64_STANDARD, Engine};
                use byte_aes::Aes256Cryptor;
                
                pub fn #function_name() -> Vec<u8> {
                    let decoded_data = {
                        let value = BASE64_STANDARD.decode(#encoded_value.as_bytes()).unwrap();
                        let cryptor = Aes256Cryptor::try_from(#key).unwrap();
                        cryptor.decrypt(value).expect("Can't decrypt data")
                    };
                
                    decoded_data
                }
            }

            pub use #mod_name::#function_name;
        }.into()
    }
}

#[cfg(feature = "encryption")]
pub use encryption::aes;