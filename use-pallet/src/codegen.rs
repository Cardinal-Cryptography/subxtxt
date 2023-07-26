use ir::PalletConfiguration;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::ir;

pub struct CodeGenerator {
    config: PalletConfiguration,
}

impl From<PalletConfiguration> for CodeGenerator {
    fn from(config: PalletConfiguration) -> Self {
        Self { config }
    }
}

impl CodeGenerator {
    pub fn generate_code(&self) -> Result<TokenStream2, syn::Error> {
        Ok(quote! {})
    }
}
