#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Fields, FieldsNamed,
    FieldsUnnamed, Ident, Index, Type, Variant,
};

#[proc_macro_derive(ByteBufferWrite)]
pub fn derive_byte_buffer_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    match input.data {
        Data::Struct(data) => parse_struct_write(ident, data),
        Data::Enum(data) => parse_enum_write(ident, data),
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

fn parse_enum_write(ident: Ident, data: DataEnum) -> TokenStream {
    let mut variants_native = Vec::<proc_macro2::TokenStream>::new();
    let mut variants_le = Vec::<proc_macro2::TokenStream>::new();
    let mut variants_be = Vec::<proc_macro2::TokenStream>::new();

    let mut id: u16 = 1;
    for variant in data.variants {
        let variant_ident = &variant.ident;

        match &variant.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                let mut field_idents = Vec::<Ident>::new();

                for field in named {
                    field_idents.push(field.ident.as_ref().unwrap().clone());
                }

                variants_native.push(quote! {
                    #ident::#variant_ident { #(#field_idents),* } => {
                        buffer.write(#id)?;
                        #(buffer.write(#field_idents)?;)*
                    }
                });

                variants_le.push(quote! {
                    #ident::#variant_ident { #(#field_idents),* } => {
                        buffer.write_le(#id)?;
                        #(buffer.write_le(#field_idents)?;)*
                    }
                });

                variants_be.push(quote! {
                    #ident::#variant_ident { #(#field_idents),* } => {
                        buffer.write_be(#id)?;
                        #(buffer.write_be(#field_idents)?;)*
                    }
                });
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let mut field_idents = Vec::<Ident>::new();
                let mut field_count = 0;

                for field in unnamed {
                    field_idents.push(Ident::new(
                        format!("val{}", field_count).as_str(),
                        variant.span(),
                    ));

                    field_count += 1;
                }

                variants_native.push(quote! {
                    #ident::#variant_ident ( #(#field_idents),* ) => {
                        buffer.write(#id)?;
                        #(buffer.write(#field_idents)?;)*
                    }
                });

                variants_le.push(quote! {
                    #ident::#variant_ident ( #(#field_idents),* ) => {
                        buffer.write_le(#id)?;
                        #(buffer.write_le(#field_idents)?;)*
                    }
                });

                variants_be.push(quote! {
                    #ident::#variant_ident ( #(#field_idents),* ) => {
                        buffer.write_be(#id)?;
                        #(buffer.write_be(#field_idents)?;)*
                    }
                });
            }
            Fields::Unit => {
                variants_native.push(quote! {
                    #ident::#variant_ident => {
                        buffer.write(#id)?;
                    }
                });

                variants_le.push(quote! {
                    #ident::#variant_ident => {
                        buffer.write_le(#id)?;
                    }
                });

                variants_be.push(quote! {
                    #ident::#variant_ident => {
                        buffer.write_be(#id)?;
                    }
                });
            }
        }

        id += 1;
    }

    let expanded = quote! {
        impl ::bytey_byte_buffer::byte_buffer_write::ByteBufferWrite for #ident {
            #[inline]
            fn write_to_buffer(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                match self {
                    #(#variants_native),*
                }

                Ok(())
            }

            #[inline]
            fn write_to_buffer_le(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                match self {
                    #(#variants_le),*
                }

                Ok(())
            }

            #[inline]
            fn write_to_buffer_be(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                match self {
                    #(#variants_be),*
                }

                Ok(())
            }
        }
    };

    expanded.into()
}

fn parse_struct_write(ident: Ident, data: DataStruct) -> TokenStream {
    match data.fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let mut fields = Vec::<Ident>::new();

            for field in named {
                fields.push(field.ident.unwrap());
            }

            generate_byte_buffer_write_impl(ident, fields)
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let mut count: usize = 0;
            let mut fields = Vec::<Index>::new();

            for field in unnamed {
                fields.push(Index::from(count));

                count += 1;
            }

            generate_byte_buffer_write_impl(ident, fields)
        }
        Fields::Unit => TokenStream::new(),
    }
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
        Data::Enum(data) => parse_enum_read(ident, data),
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

fn parse_enum_read(ident: Ident, data: DataEnum) -> TokenStream {
    let mut variants_native = Vec::<proc_macro2::TokenStream>::new();
    let mut variants_le = Vec::<proc_macro2::TokenStream>::new();
    let mut variants_be = Vec::<proc_macro2::TokenStream>::new();

    let mut id: u16 = 1;
    for variant in data.variants {
        let variant_ident = &variant.ident;

        match &variant.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                let mut field_idents = Vec::<Ident>::new();
                let mut field_tys = Vec::<Type>::new();

                for field in named {
                    field_idents.push(field.ident.as_ref().unwrap().clone());
                    field_tys.push(field.ty.clone());
                }

                variants_native.push(quote! {
                    #id => {
                        Ok(#ident::#variant_ident {
                            #(#field_idents: buffer.read::<#field_tys>()?),*
                        })
                    }
                });

                variants_le.push(quote! {
                    #id => {
                        Ok(#ident::#variant_ident {
                            #(#field_idents: buffer.read_le::<#field_tys>()?),*
                        })
                    }
                });

                variants_be.push(quote! {
                    #id => {
                        Ok(#ident::#variant_ident {
                            #(#field_idents: buffer.read_be::<#field_tys>()?),*
                        })
                    }
                });
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let mut field_tys = Vec::<Type>::new();

                for field in unnamed {
                    field_tys.push(field.ty.clone());
                }

                variants_native.push(quote! {
                    #id => {
                        Ok(#ident::#variant_ident (
                            #(buffer.read::<#field_tys>()?),*
                        ))
                    }
                });

                variants_le.push(quote! {
                    #id => {
                        Ok(#ident::#variant_ident (
                            #(buffer.read_le::<#field_tys>()?),*
                        ))
                    }
                });

                variants_be.push(quote! {
                    #id => {
                        Ok(#ident::#variant_ident (
                            #(buffer.read_be::<#field_tys>()?),*
                        ))
                    }
                });
            }
            Fields::Unit => {
                variants_native.push(quote! {
                    #id => Ok(#ident::#variant_ident)
                });

                variants_le.push(quote! {
                    #id => Ok(#ident::#variant_ident)
                });

                variants_be.push(quote! {
                    #id => Ok(#ident::#variant_ident)
                });
            }
        }

        id += 1;
    }

    let expanded = quote! {
        impl ::bytey_byte_buffer::byte_buffer_read::ByteBufferRead for #ident {
            #[inline]
            fn read_from_buffer(buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<Self> {
                match buffer.read::<u16>()? {
                    #(#variants_native),*,
                    id => Err(::bytey_byte_buffer::error::ByteBufferError::OtherError { error: ::std::format!("Invalid id: {}", id) })
                }
            }

            #[inline]
            fn read_from_buffer_le(buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<Self> {
                match buffer.read::<u16>()? {
                    #(#variants_le),*,
                    id => Err(::bytey_byte_buffer::error::ByteBufferError::OtherError { error: ::std::format!("Invalid id: {}", id) })
                }
            }

            #[inline]
            fn read_from_buffer_be(buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<Self> {
                match buffer.read::<u16>()? {
                    #(#variants_be),*,
                    id => Err(::bytey_byte_buffer::error::ByteBufferError::OtherError { error: ::std::format!("Invalid id: {}", id) })
                }
            }
        }
    };

    expanded.into()
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
