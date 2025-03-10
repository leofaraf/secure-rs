pub(crate) mod secure_str;
pub(crate) mod include_secure_str;
pub(crate) mod include_secure_bytes;
pub(crate) mod backend;

extern crate proc_macro;
use proc_macro::TokenStream;

#[cfg(feature = "compression")]
#[proc_macro]
pub fn include_secure_str_brotli(_item: TokenStream) -> TokenStream {
    include_secure_str::brotli(_item)
}

#[cfg(feature = "compression")]
#[proc_macro]
pub fn secure_str_brotli(_item: TokenStream) -> TokenStream {
    secure_str::brotli(_item)
}

#[cfg(feature = "compression")]
#[proc_macro]
pub fn include_secure_bytes_brotli(_item: TokenStream) -> TokenStream {
    include_secure_bytes::brotli(_item)
}

#[cfg(feature = "encryption")]
#[proc_macro]
pub fn include_secure_str_aes(_item: TokenStream) -> TokenStream {
    include_secure_str::aes(_item)
}

#[cfg(feature = "encryption")]
#[proc_macro]
pub fn secure_str_aes(_item: TokenStream) -> TokenStream {
    secure_str::aes(_item)
}