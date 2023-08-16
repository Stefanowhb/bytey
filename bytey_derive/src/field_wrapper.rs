use crate::symbols::*;
use syn::{
    Meta::{List, NameValue, Path},
    NestedMeta,
};

pub struct FieldWrapper<'a> {
    pub field: Option<&'a syn::Ident>,
    pub index: Option<syn::Index>,
}

impl quote::ToTokens for FieldWrapper<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(field) = self.field {
            field.to_tokens(tokens);
        } else if let Some(index) = &self.index {
            index.to_tokens(tokens);
        }
    }
}

pub fn is_skipped(field: &syn::Field) -> bool {
    for meta_item in field.attrs.iter().flat_map(get_bytey_meta_items) {
        match meta_item {
            NestedMeta::Meta(Path(word)) if word == SKIP => return true,
            NestedMeta::Meta(_) => {
                panic!("Unexpected field attribute found in bytey. Only skip is supported")
            }
            NestedMeta::Lit(_) => panic!("unexpected literal in bytey field attributes"),
        }
    }

    false
}

fn get_bytey_meta_items(attr: &syn::Attribute) -> Vec<syn::NestedMeta> {
    if attr.path != BYTEY {
        return Vec::new();
    }

    match attr.parse_meta() {
        Ok(List(meta)) => meta.nested.into_iter().collect(),
        Ok(other) => match other {
            Path(_) => panic!("expected #[bytey(...)]"),
            List(_) => panic!(
                "Error Structured named lists like derive(Copy, Clone) is not supported by bytey",
            ),
            NameValue(_) => panic!(
                "Error: Name Value paires like feature = \"nightly\" are not support by bytey",
            ),
        },
        Err(err) => {
            panic!("Error in bytey derive {} ", err)
        }
    }
}
