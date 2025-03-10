pub(crate) mod secure_str;
pub(crate) mod include_secure_str;
pub(crate) mod include_secure_bytes;
pub(crate) mod backend;

extern crate proc_macro;
use proc_macro::TokenStream;

#[cfg(feature = "compression")]
#[proc_macro]
/// **pathname: &str, varname: &str**
/// 
/// equivalent of `include_str!()`, but compress in compile-time via brotli
/// 
/// pathname is path of located file
/// 
/// each macro accept varname and as result generate function with name: `get_{varname}`
pub fn include_secure_str_brotli(_item: TokenStream) -> TokenStream {
    include_secure_str::brotli(_item)
}

#[cfg(feature = "compression")]
#[proc_macro]
/// **varname: &str, value: &str**
/// 
/// equivalent of `include_bytes!()`, but compress in compile-time via brotli
/// 
/// value is result fo generated function
/// 
/// each macro accept varname and as result generate function with name: `get_{varname}`
pub fn secure_str_brotli(_item: TokenStream) -> TokenStream {
    secure_str::brotli(_item)
}

#[cfg(feature = "compression")]
#[proc_macro]
/// **pathname: &str, varname: &str**
/// 
/// equivalent of `include_bytes!()`, but compress in compile-time via brotli
/// 
/// pathname is path of located file
/// 
/// each macro accept varname and as result generate function with name: `get_{varname}`
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

#[cfg(feature = "encryption")]
#[proc_macro]
pub fn include_secure_bytes_aes(_item: TokenStream) -> TokenStream {
    include_secure_bytes::aes(_item)
}