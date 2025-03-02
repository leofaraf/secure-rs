mod methods;

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn include_secure_str(_item: TokenStream) -> TokenStream {
    methods::include_secure_str::handle(_item)
}

#[proc_macro]
pub fn secure_str(_item: TokenStream) -> TokenStream {
    methods::secure_str::handle(_item)
}
