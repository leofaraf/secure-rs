use std::{env, path::PathBuf};

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

        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let resolved_path = PathBuf::from(manifest_dir).join(path_lit.value());

        Ok(IncludeSecureBytesBrotliParams {
            path: resolved_path,
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

        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let resolved_path = PathBuf::from(manifest_dir).join(path_lit.value());

        Ok(IncludeSecureBytesAesParams {
            key: key_lit.value(),
            path: resolved_path,
            name: name_lit.value(),
        })
    }
}