use proc_macro2::TokenStream as TokenStream2;

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
