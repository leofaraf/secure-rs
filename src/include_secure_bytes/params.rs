use std::path::PathBuf;

use syn::{
    parse::{Parse, ParseStream},
    LitStr,
};

pub(super) struct IncludeSecureBytesBrotliParams {
    pub(super) path: PathBuf,
    pub(super) name: String,
}

impl Parse for IncludeSecureBytesBrotliParams {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let path_lit: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let name_lit: LitStr = input.parse()?;

        Ok(IncludeSecureBytesBrotliParams {
            path: PathBuf::from(path_lit.value()),
            name: name_lit.value(),
        })
    }
}

pub(super) struct IncludeSecureBytesAesParams {
    pub(super) path: PathBuf,
    pub(super) key: String,
    pub(super) name: String,
}

impl Parse for IncludeSecureBytesAesParams {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let path_lit: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let key_lit: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let name_lit: LitStr = input.parse()?;

        Ok(IncludeSecureBytesAesParams {
            key: key_lit.value(),
            path: PathBuf::from(path_lit.value()),
            name: name_lit.value(),
        })
    }
}