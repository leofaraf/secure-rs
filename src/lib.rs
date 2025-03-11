pub(crate) mod secure_str;
pub(crate) mod include_secure_str;
pub(crate) mod include_secure_bytes;
pub(crate) mod backend;

extern crate proc_macro;
use proc_macro::TokenStream;

#[cfg(feature = "compression")]
#[proc_macro]
/// equivalent of `include_str!()`, but compress in compile-time via brotli
/// 
/// **pathname: &str, varname: &str**
/// 
/// pathname is path of located file
/// 
/// each macro accept varname and as result generate function with name: `get_{varname}`
pub fn include_secure_str_brotli(_item: TokenStream) -> TokenStream {
    include_secure_str::brotli(_item)
}

#[cfg(feature = "compression")]
#[proc_macro]
/// equivalent of `include_bytes!()`, but compress in compile-time via brotli
/// 
/// **varname: &str, value: &str**
/// 
/// value is result fo generated function
/// 
/// each macro accept varname and as result generate function with name: `get_{varname}`
pub fn secure_str_brotli(_item: TokenStream) -> TokenStream {
    secure_str::brotli(_item)
}

#[cfg(feature = "compression")]
#[proc_macro]
/// equivalent of `include_bytes!()`, but compress in compile-time via brotli
/// 
/// **pathname: &str, varname: &str**
/// 
/// pathname is path of located file
/// 
/// each macro accept varname and as result generate function with name: `get_{varname}`
pub fn include_secure_bytes_brotli(_item: TokenStream) -> TokenStream {
    include_secure_bytes::brotli(_item)
}

#[cfg(feature = "encryption")]
#[proc_macro]
/// equivalent of `include_str!()`, but compress in compile-time via brotli
/// 
/// **pathname: &str, aes_key: &str, varname: &str**
/// 
/// pathname is path of located file
/// 
/// aes_key is string that should contain utf8 key, the number of bytes of the key shall be 32
/// 
/// each macro accept varname and as result generate function with name: `get_{varname}`
pub fn include_secure_str_aes(_item: TokenStream) -> TokenStream {
    include_secure_str::aes(_item)
}

#[cfg(feature = "encryption")]
#[proc_macro]
/// equivalent of `include_bytes!()`, but compress in compile-time via brotli
///
/// **aes_key: &str, varname: &str, value: &str**
/// 
/// value is result fo generated function
/// 
/// aes_key is string that should contain utf8 key, the number of bytes of the key shall be 32
/// 
/// each macro accept varname and as result generate function with name: `get_{varname}`
pub fn secure_str_aes(_item: TokenStream) -> TokenStream {
    secure_str::aes(_item)
}

#[cfg(feature = "encryption")]
#[proc_macro]
/// equivalent of `include_bytes!()`, but compress in compile-time via brotli
/// 
/// **pathname: &str, aes_key: &str, varname: &str**
/// 
/// pathname is path of located file
/// 
/// aes_key is string that should contain utf8 key, the number of bytes of the key shall be 32
/// 
/// each macro accept varname and as result generate function with name: `get_{varname}`
pub fn include_secure_bytes_aes(_item: TokenStream) -> TokenStream {
    include_secure_bytes::aes(_item)
}