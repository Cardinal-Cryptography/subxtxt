use proc_macro2::TokenStream as TokenStream2;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PalletConfiguration {
    pallet_name: syn::Ident,
    type_declarations: Vec<syn::ItemType>,
}

impl PalletConfiguration {
    pub fn new(_attr: TokenStream2, input: TokenStream2) -> Result<Self, syn::Error> {
        let item_mod = syn::parse2::<syn::ItemMod>(input)?;

        let type_declarations = item_mod
            .content
            .unwrap_or_default()
            .1
            .iter()
            .filter_map(|item| match item {
                syn::Item::Type(item_type) => Some(item_type.clone()),
                _ => None,
            })
            .collect();

        Ok(Self {
            pallet_name: item_mod.ident,
            type_declarations,
        })
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
    use quote::quote;

    use crate::ir::PalletConfiguration;

    fn check_creation(input: TokenStream2, expected: Result<PalletConfiguration, &'static str>) {
        assert_eq!(
            PalletConfiguration::new(Default::default(), input).map_err(|err| err.to_string()),
            expected.map_err(ToString::to_string),
        );
    }

    #[test]
    fn non_module_item_fails() {
        check_creation(quote! { fn pallet() {} }, Err("expected `mod`"));
        check_creation(quote! { type pallet = (); }, Err("expected `mod`"));
        check_creation(quote! { struct Pallet; }, Err("expected `mod`"));
        check_creation(quote! { enum Pallet {} }, Err("expected `mod`"));
    }

    #[test]
    fn empty_config_works() {
        check_creation(
            quote! { mod pallet {} },
            Ok(PalletConfiguration {
                pallet_name: Ident::new("pallet", Span::call_site()),
                type_declarations: vec![],
            }),
        )
    }

    #[test]
    fn type_declarations_works() {
        check_creation(
            quote! {
                mod pallet_with_primitive_types {
                    type A = u32;
                    type B = u64;
                }
            },
            Ok(PalletConfiguration {
                pallet_name: Ident::new("pallet_with_primitive_types", Span::call_site()),
                type_declarations: vec![
                    syn::parse_quote! { type A = u32; },
                    syn::parse_quote! { type B = u64; },
                ],
            }),
        )
    }
}
