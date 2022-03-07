#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed,
    Ident, Index, Path,
};

#[proc_macro_derive(ByteBufferWrite, attributes(skip))]
pub fn derive_byte_buffer_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    match input.data {
        Data::Struct(data) => parse_struct(ident, data),
        Data::Enum(data) => {
            ident
                .span()
                .unwrap()
                .error("Enums are currently not supported")
                .emit();

            TokenStream::new()
        }
        Data::Union(_) => {
            ident
                .span()
                .unwrap()
                .error("Unions are currently not supported")
                .emit();

            TokenStream::new()
        }
    }
}

fn parse_struct(ident: Ident, data: DataStruct) -> TokenStream {
    let mut expanded: TokenStream;

    match data.fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let mut fields = Vec::<Ident>::new();

            for field in named {
                if !skip_field(&field) {
                    fields.push(field.ident.unwrap());
                }
            }

            expanded = generate_impl(ident, fields);
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let mut count: usize = 0;
            let mut fields = Vec::<Index>::new();

            for field in unnamed {
                if !skip_field(&field) {
                    fields.push(Index::from(count));
                }

                count += 1;
            }

            expanded = generate_impl(ident, fields);
        }
        Fields::Unit => expanded = TokenStream::new(),
    }

    expanded
}

fn skip_field(field: &Field) -> bool {
    if !field.attrs.is_empty() {
        for attr in &field.attrs {
            if let Ok(Path(path)) = attr.parse_meta() {
                if path.segments.first().unwrap().ident.to_string() == "skip" {
                    return true;
                }
            }
        }
    }

    false
}

fn generate_impl<T: quote::ToTokens>(ident: Ident, tokens: Vec<T>) -> TokenStream {
    let expanded = quote! {
        impl ::bytey::ByteBufferWrite for #ident {
            #[inline]
            fn write_to_buffer(&self, buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<()> {
                #(
                    self.#tokens.write_to_buffer(buffer);
                )*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_le(&self, buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<()> {
                #(
                    self.#tokens.write_to_buffer_le(buffer);
                )*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_be(&self, buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<()> {
                #(
                    self.#tokens.write_to_buffer_be(buffer);
                )*

                Ok(())
            }
        }

        impl ::bytey::ByteBufferWrite for &#ident {
            #[inline]
            fn write_to_buffer(&self, buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<()> {
                #(
                    self.#tokens.write_to_buffer(buffer);
                )*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_le(&self, buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<()> {
                #(
                    self.#tokens.write_to_buffer_le(buffer);
                )*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_be(&self, buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<()> {
                #(
                    self.#tokens.write_to_buffer_be(buffer);
                )*

                Ok(())
            }
        }
    };

    expanded.into()
}
