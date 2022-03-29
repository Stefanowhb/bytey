pub struct StructSource<'a> {
    pub ident: &'a syn::Ident,
    pub fields: &'a syn::Fields,
    pub generics: &'a syn::Generics,
}

pub struct EnumSource<'a> {
    pub ident: &'a syn::Ident,
    pub variants: Vec<EnumVariant<'a>>,
    pub generics: &'a syn::Generics,
}

pub struct EnumVariant<'a> {
    pub ident: &'a syn::Ident,
    pub fields: &'a syn::Fields,
}

impl StructSource<'_> {
    pub fn from_input(input: &syn::DeriveInput) -> StructSource {
        let ident = &input.ident;
        let fields = match &input.data {
            syn::Data::Struct(data) => &data.fields,
            _ => panic!("StructSource::from_input called on a non-struct"),
        };
        let generics = &input.generics;

        StructSource {
            ident,
            fields,
            generics,
        }
    }
}

impl EnumSource<'_> {
    pub fn from_input(input: &syn::DeriveInput) -> EnumSource {
        let ident = &input.ident;
        let mut vars: Vec<EnumVariant> = Vec::new();
        let generics = &input.generics;

        match &input.data {
            syn::Data::Enum(syn::DataEnum { variants, .. }) => {
                for variant in variants {
                    let ident = &variant.ident;
                    let fields = &variant.fields;

                    vars.push(EnumVariant { ident, fields });
                }
            }
            _ => panic!("StructSource::from_input called on a non-struct"),
        }

        EnumSource {
            ident,
            variants: vars,
            generics,
        }
    }
}
