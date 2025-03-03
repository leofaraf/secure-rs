use params::IncludeSecureStringBrotliParams;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod params;

pub fn brotli(_item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(_item as IncludeSecureStringBrotliParams);

    
    
    TokenStream::new()
}