use std::path::PathBuf;

use syn::{
    parse::{Parse, ParseStream},
    LitStr,
};

pub(super) struct IncludeSecureStringBrotliParams {
    pub(super) path: PathBuf,
    pub(super) name: String,
}

impl Parse for IncludeSecureStringBrotliParams {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let path_lit: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let name_lit: LitStr = input.parse()?;

        Ok(IncludeSecureStringBrotliParams {
            path: PathBuf::from(path_lit.value()),
            name: name_lit.value(),
        })
    }
}