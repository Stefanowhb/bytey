use crate::field_wrapper::FieldWrapper;
use crate::source::{EnumSource, StructSource};
use quote::quote;
use syn::spanned::Spanned;

pub fn expand_derive_byte_buffer_write(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match &input.data {
        syn::Data::Struct(_) => {
            let source = StructSource::from_input(input);

            handle_struct(source)
        }
        syn::Data::Enum(_) => {
            let source = EnumSource::from_input(input);

            handle_enum(source)
        }
        syn::Data::Union(_) => {
            input
                .ident
                .span()
                .unwrap()
                .error("Unions are currently not supported")
                .emit();

            proc_macro2::TokenStream::new()
        }
    }
}

fn handle_struct(input: StructSource) -> proc_macro2::TokenStream {
    let mut fields: Vec<FieldWrapper> = Vec::new();

    match input.fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            for field in named {
                fields.push(FieldWrapper {
                    field: Some(field.ident.as_ref().unwrap()),
                    index: None,
                });
            }
        }
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
            for (count, _) in unnamed.into_iter().enumerate() {
                fields.push(FieldWrapper {
                    field: None,
                    index: Some(syn::Index::from(count)),
                });
            }
        }
        syn::Fields::Unit => return proc_macro2::TokenStream::new(),
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident = input.ident;

    quote! {
        impl #impl_generics ::bytey_byte_buffer::byte_buffer_write::ByteBufferWrite for #ident #ty_generics #where_clause {
            #[inline]
            fn write_to_buffer(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(self.#fields.write_to_buffer(buffer)?;)*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_le(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(self.#fields.write_to_buffer_le(buffer)?;)*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_be(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(self.#fields.write_to_buffer_be(buffer)?;)*

                Ok(())
            }
        }

        impl #impl_generics ::bytey_byte_buffer::byte_buffer_write::ByteBufferWrite for &#ident #ty_generics #where_clause {
            #[inline]
            fn write_to_buffer(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(self.#fields.write_to_buffer(buffer)?;)*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_le(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(self.#fields.write_to_buffer_le(buffer)?;)*

                Ok(())
            }

            #[inline]
            fn write_to_buffer_be(&self, buffer: &mut ::bytey_byte_buffer::byte_buffer::ByteBuffer) -> ::bytey_byte_buffer::error::Result<()> {
                #(self.#fields.write_to_buffer_be(buffer)?;)*

                Ok(())
            }
        }
    }
}

fn handle_enum(input: EnumSource) -> proc_macro2::TokenStream {
    let enum_ident = input.ident;
    let mut variants_native: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut variants_le: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut variants_be: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut id: u16 = 1;

    for variant in &input.variants {
        let mut field_idents: Vec<syn::Ident> = Vec::new();
        let variant_ident = variant.ident;

        let variant_match_case = match variant.fields {
            syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
                for field in named {
                    field_idents.push(field.ident.as_ref().unwrap().clone());
                }

                quote! { #enum_ident::#variant_ident { #(#field_idents),* } }
            }
            syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
                for (count, field) in unnamed.into_iter().enumerate() {
                    field_idents.push(syn::Ident::new(
                        format!("val{}", count).as_str(),
                        field.span(),
                    ));
                }

                quote! { #enum_ident::#variant_ident ( #(#field_idents),* ) }
            }
            syn::Fields::Unit => quote! {#enum_ident::#variant_ident},
        };

        variants_native.push(quote! {
            #variant_match_case => {
                #id.write_to_buffer(buffer)?;
                #(#field_idents.write_to_buffer(buffer)?;)*
            }
        });

        variants_le.push(quote! {
            #variant_match_case => {
                #id.write_to_buffer_le(buffer)?;
                #(#field_idents.write_to_buffer_le(buffer)?;)*
            }
        });

        variants_be.push(quote! {
            #variant_match_case => {
                #id.write_to_buffer_be(buffer)?;
                #(#field_idents.write_to_buffer_be(buffer)?;)*
            }
        });

        id += 1;
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::bytey_byte_buffer::byte_buffer_write::ByteBufferWrite for #enum_ident #ty_generics #where_clause {
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

        impl #impl_generics ::bytey_byte_buffer::byte_buffer_write::ByteBufferWrite for &#enum_ident #ty_generics #where_clause {
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
    }
}
