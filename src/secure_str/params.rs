use syn::{
    parse::{Parse, ParseStream},
    LitStr,
};

pub(super) struct SecureStringBrotliParams {
    pub(super) name: String,
    pub(super) value: String,
}

impl Parse for SecureStringBrotliParams {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let name_lit: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let value_lit: LitStr = input.parse()?;

        Ok(SecureStringBrotliParams {
            name: name_lit.value(),
            value: value_lit.value(),
        })
    }
}

pub(super) struct SecureStringAesParams {
    pub(super) key: String,
    pub(super) name: String,
    pub(super) value: String,
}

impl Parse for SecureStringAesParams {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let key_lit: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let name_lit: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let value_lit: LitStr = input.parse()?;

        Ok(SecureStringAesParams {
            key: key_lit.value(),
            name: name_lit.value(),
            value: value_lit.value(),
        })
    }
}