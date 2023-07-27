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
        let pallet_name = &self.config.pallet_name;

        Ok(quote! {
            use ::subxtxt::anyhow;
            use ::subxtxt::async_trait;
            use ::subxtxt::parity_scale_codec;

            use ::subxtxt::pallets::#pallet_name::*;

            // ::subxtxt::pallets::#pallet_name
            ::subxtxt::pallet_api_impl!(());
        })
    }
}
