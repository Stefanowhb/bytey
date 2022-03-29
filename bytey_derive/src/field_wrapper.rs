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
