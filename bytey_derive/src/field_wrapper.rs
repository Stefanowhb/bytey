use crate::symbols::*;
use syn::punctuated::Punctuated;
use syn::{Meta, Token};

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
    let mut ret = false;

    for meta_item in field.attrs.iter().flat_map(get_bytey_meta_items) {
        match meta_item {
            Meta::Path(word) if word == SKIP => ret = true,
            Meta::Path(_) => {
                panic!("Unexpected field attribute found in bytey. Only skip is supported")
            }
            _ => {}
        }
    }

    ret
}

fn get_bytey_meta_items(attr: &syn::Attribute) -> Vec<syn::Meta> {
    if attr.path() != BYTEY {
        return Vec::new();
    }

    match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
        Ok(nested) => nested.into_iter().collect(),
        Err(err) => {
            panic!("error #[bytey(...)]: {} ", err);
        }
    }
}
