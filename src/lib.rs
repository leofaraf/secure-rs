pub(crate) mod secure_str;
pub(crate) mod include_secure_str;

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn include_secure_str(_item: TokenStream) -> TokenStream {
    include_secure_str::handle(_item)
}

#[cfg(feature = "brotli")]
#[proc_macro]
pub fn secure_str_brotli(_item: TokenStream) -> TokenStream {
    secure_str::brotli(_item)
}