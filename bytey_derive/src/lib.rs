#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident,
    Index, Type,
};

#[proc_macro_derive(ByteBufferWrite)]
pub fn derive_byte_buffer_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    match input.data {
        Data::Struct(data) => parse_struct_write(ident, data),
        Data::Enum(_) => {
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

fn parse_struct_write(ident: Ident, data: DataStruct) -> TokenStream {
    let mut expanded: TokenStream;

    match data.fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let mut fields = Vec::<Ident>::new();

            for field in named {
                fields.push(field.ident.unwrap());
            }

            expanded = generate_byte_buffer_write_impl(ident, fields);
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let mut count: usize = 0;
            let mut fields = Vec::<Index>::new();

            for field in unnamed {
                fields.push(Index::from(count));

                count += 1;
            }

            expanded = generate_byte_buffer_write_impl(ident, fields);
        }
        Fields::Unit => expanded = TokenStream::new(),
    }

    expanded
}

fn generate_byte_buffer_write_impl<T: quote::ToTokens>(
    ident: Ident,
    tokens: Vec<T>,
) -> TokenStream {
    let expanded = quote! {
        impl ::bytey_byte_buffer::byte_buffer_write::ByteBufferWrite for #ident {
            #[inline]
            fn write_to_buffer(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(
                    self.#tokens.write_to_buffer(buffer)?;
                )*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_le(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(
                    self.#tokens.write_to_buffer_le(buffer)?;
                )*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_be(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(
                    self.#tokens.write_to_buffer_be(buffer)?;
                )*

                Ok(())
            }
        }

        impl ::bytey_byte_buffer::byte_buffer_write::ByteBufferWrite for &#ident {
            #[inline]
            fn write_to_buffer(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(
                    self.#tokens.write_to_buffer(buffer)?;
                )*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_le(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(
                    self.#tokens.write_to_buffer_le(buffer)?;
                )*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_be(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(
                    self.#tokens.write_to_buffer_be(buffer)?;
                )*

                Ok(())
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(ByteBufferRead)]
pub fn derive_byte_buffer_read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    match input.data {
        Data::Struct(data) => parse_struct_read(ident, data),
        Data::Enum(_) => {
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

fn parse_struct_read(ident: Ident, data: DataStruct) -> TokenStream {
    match data.fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let mut fields = Vec::<Ident>::new();
            let mut tys = Vec::<Type>::new();

            for field in named {
                fields.push(field.ident.unwrap());
                tys.push(field.ty);
            }

            generate_byte_buffer_read_impl_named(ident, fields, tys)
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let mut count: usize = 0;
            let mut tys = Vec::<Type>::new();

            for field in unnamed {
                tys.push(field.ty);
            }

            generate_byte_buffer_read_impl_unnamed(ident, tys)
        }
        Fields::Unit => TokenStream::new(),
    }
}

fn generate_byte_buffer_read_impl_named(
    struct_ident: Ident,
    field_idents: Vec<Ident>,
    field_tys: Vec<Type>,
) -> TokenStream {
    let expanded = quote! {
        impl ::bytey_byte_buffer::byte_buffer_read::ByteBufferRead for #struct_ident {
            #[inline]
            fn read_from_buffer(buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<#struct_ident> {
                Ok(#struct_ident {
                    #(
                        #field_idents: buffer.read::<#field_tys>()?
                    ),*
                })
            }

            #[inline]
            fn read_from_buffer_le(buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<#struct_ident> {
                Ok(#struct_ident {
                    #(
                        #field_idents: buffer.read_le::<#field_tys>()?
                    ),*
                })
            }

            #[inline]
            fn read_from_buffer_be(buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<#struct_ident> {
                Ok(#struct_ident {
                    #(
                        #field_idents: buffer.read_be::<#field_tys>()?
                    ),*
                })
            }
        }
    };

    expanded.into()
}

fn generate_byte_buffer_read_impl_unnamed(
    struct_ident: Ident,
    field_tys: Vec<Type>,
) -> TokenStream {
    let expanded = quote! {
        impl ::bytey_byte_buffer::byte_buffer_read::ByteBufferRead for #struct_ident {
            #[inline]
            fn read_from_buffer(buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<#struct_ident> {
                Ok(#struct_ident (
                    #(
                        buffer.read::<#field_tys>()?
                    ),*
                ))
            }

            #[inline]
            fn read_from_buffer_le(buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<#struct_ident> {
                Ok(#struct_ident (
                    #(
                        buffer.read_le::<#field_tys>()?
                    ),*
                ))
            }

            #[inline]
            fn read_from_buffer_be(buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<#struct_ident> {
                Ok(#struct_ident (
                    #(
                        buffer.read_be::<#field_tys>()?
                    ),*
                ))
            }
        }
    };

    expanded.into()
}
