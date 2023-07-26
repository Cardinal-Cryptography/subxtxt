use proc_macro2::TokenStream as TokenStream2;

pub struct PalletConfiguration {}

impl PalletConfiguration {
    pub fn new(_attr: TokenStream2, input: TokenStream2) -> Result<Self, syn::Error> {
        Ok(Self {})
    }
}
