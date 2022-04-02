use crate::source::{EnumSource, StructSource};
use quote::quote;

pub fn expand_derive_byte_buffer_read(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
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
    let struct_ident = input.ident;
    let init_struct_native: proc_macro2::TokenStream;
    let init_struct_le: proc_macro2::TokenStream;
    let init_struct_be: proc_macro2::TokenStream;

    match input.fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            let mut field_reads_native: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut field_reads_le: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut field_reads_be: Vec<proc_macro2::TokenStream> = Vec::new();

            for field in named {
                let field_ident = field.ident.as_ref().unwrap();
                let field_ty = &field.ty;

                field_reads_native.push(quote! {
                    #field_ident: buffer.read::<#field_ty>()?
                });

                field_reads_le.push(quote! {
                    #field_ident: buffer.read_le::<#field_ty>()?
                });

                field_reads_be.push(quote! {
                    #field_ident: buffer.read_be::<#field_ty>()?
                });
            }

            init_struct_native = quote! {
                #struct_ident {
                    #(#field_reads_native),*
                }
            };

            init_struct_le = quote! {
                #struct_ident {
                    #(#field_reads_le),*
                }
            };

            init_struct_be = quote! {
                #struct_ident {
                    #(#field_reads_be),*
                }
            };
        }
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
            let mut field_reads_native: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut field_reads_le: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut field_reads_be: Vec<proc_macro2::TokenStream> = Vec::new();

            for field in unnamed {
                let field_ty = &field.ty;

                field_reads_native.push(quote! {
                    buffer.read::<#field_ty>()?
                });

                field_reads_le.push(quote! {
                    buffer.read_le::<#field_ty>()?
                });

                field_reads_be.push(quote! {
                    buffer.read_be::<#field_ty>()?
                });
            }

            init_struct_native = quote! {
                #struct_ident (
                    #(#field_reads_native),*
                )
            };

            init_struct_le = quote! {
                #struct_ident (
                    #(#field_reads_le),*
                )
            };

            init_struct_be = quote! {
                #struct_ident (
                    #(#field_reads_be),*
                )
            };
        }
        syn::Fields::Unit => {
            input
                .ident
                .span()
                .unwrap()
                .error("Unit structs are currently not supported")
                .emit();

            return proc_macro2::TokenStream::new();
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::bytey::ByteBufferRead for #struct_ident #ty_generics #where_clause {
            #[inline]
            fn read_from_buffer(buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<#struct_ident #ty_generics> {
                Ok(#init_struct_native)
            }

            #[inline]
            fn read_from_buffer_le(buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<#struct_ident #ty_generics> {
                Ok(#init_struct_le)
            }

            #[inline]
            fn read_from_buffer_be(buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<#struct_ident #ty_generics> {
                Ok(#init_struct_be)
            }
        }
    }
}

fn handle_enum(input: EnumSource) -> proc_macro2::TokenStream {
    let enum_ident = input.ident;
    let mut match_arms_native: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut match_arms_le: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut match_arms_be: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut id: u16 = 1;

    for variant in &input.variants {
        let variant_ident = variant.ident;

        match variant.fields {
            syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
                let mut field_idents: Vec<&syn::Ident> = Vec::new();
                let mut field_tys: Vec<&syn::Type> = Vec::new();

                for field in named {
                    field_idents.push(field.ident.as_ref().unwrap());
                    field_tys.push(&field.ty);
                }

                match_arms_native.push(quote! {
                    #id => {
                        Ok(#enum_ident::#variant_ident {
                            #( #field_idents: buffer.read::<#field_tys>()? ),*
                        })
                    }
                });

                match_arms_le.push(quote! {
                    #id => {
                        Ok(#enum_ident::#variant_ident {
                            #( #field_idents: buffer.read_le::<#field_tys>()? ),*
                        })
                    }
                });

                match_arms_be.push(quote! {
                    #id => {
                        Ok(#enum_ident::#variant_ident {
                            #( #field_idents: buffer.read_be::<#field_tys>()? ),*
                        })
                    }
                });
            }
            syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
                let mut field_tys: Vec<&syn::Type> = Vec::new();

                for field in unnamed {
                    field_tys.push(&field.ty);
                }

                match_arms_native.push(quote! {
                    #id => {
                        Ok(#enum_ident::#variant_ident (
                            #( buffer.read::<#field_tys>()? ),*
                        ))
                    }
                });

                match_arms_le.push(quote! {
                    #id => {
                        Ok(#enum_ident::#variant_ident (
                            #( buffer.read_le::<#field_tys>()? ),*
                        ))
                    }
                });

                match_arms_be.push(quote! {
                    #id => {
                        Ok(#enum_ident::#variant_ident (
                            #( buffer.read_be::<#field_tys>()? ),*
                        ))
                    }
                });
            }
            syn::Fields::Unit => {
                match_arms_native.push(quote! {
                    #id => Ok(#enum_ident::#variant_ident)
                });

                match_arms_le.push(quote! {
                    #id => Ok(#enum_ident::#variant_ident)
                });

                match_arms_be.push(quote! {
                    #id => Ok(#enum_ident::#variant_ident)
                });
            }
        }

        id += 1;
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::bytey::ByteBufferRead for #enum_ident #ty_generics #where_clause {
            #[inline]
            fn read_from_buffer(buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<#enum_ident #ty_generics> {
                match buffer.read::<u16>()? {
                    #(#match_arms_native,)*
                    id => Err(::bytey::ByteBufferError::OtherError { error: ::std::format!("Invalid id: {}", id) })
                }
            }

            #[inline]
            fn read_from_buffer_le(buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<#enum_ident #ty_generics> {
                match buffer.read::<u16>()? {
                    #(#match_arms_le,)*
                    id => Err(::bytey::ByteBufferError::OtherError { error: ::std::format!("Invalid id: {}", id) })
                }
            }

            #[inline]
            fn read_from_buffer_be(buffer: &mut ::bytey::ByteBuffer) -> ::bytey::Result<#enum_ident #ty_generics> {
                match buffer.read::<u16>()? {
                    #(#match_arms_be,)*
                    id => Err(::bytey::ByteBufferError::OtherError { error: ::std::format!("Invalid id: {}", id) })
                }
            }
        }
    }
}
