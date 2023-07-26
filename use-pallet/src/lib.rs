mod codegen;
mod ir;

use codegen::CodeGenerator;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use crate::ir::PalletConfiguration;

#[proc_macro_attribute]
pub fn use_pallet(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_code(attr.into(), item.into()).into()
}

fn generate_code(attr: TokenStream2, input: TokenStream2) -> TokenStream2 {
    let pallet_configuration = match PalletConfiguration::new(attr, input) {
        Ok(pallet) => pallet,
        Err(err) => return err.to_compile_error(),
    };

    let codegen = CodeGenerator::from(pallet_configuration);
    match codegen.generate_code() {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}
