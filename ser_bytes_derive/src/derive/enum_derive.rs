use crate::derive::shared::{
    impl_from_named_fields, impl_from_unnamed_fields, impl_to_named_fields, impl_to_unnamed_fields,
};
use proc_macro2::Ident;
use quote::quote;
use syn::{DataEnum, Fields, Variant};

pub(super) fn impl_derive_enum(enum_data: DataEnum, enum_name: Ident) -> proc_macro2::TokenStream {
    let mut from_buf_match_tokens = Vec::new();
    let mut to_buf_match_tokens = Vec::new();

    for (index, variant) in enum_data.variants.iter().enumerate() {
        let index = index as u8;
        let Variant { fields, ident, .. } = variant;

        let (from, to) = match fields {
            Fields::Named(named_fields) => {
                let from_fields_body = impl_from_named_fields(named_fields);
                let (to_fields_destructure, to_fields_body) = impl_to_named_fields(named_fields);
                (
                    quote! {
                        #index => {
                            Ok(Self::#ident {
                                #from_fields_body
                            })
                        }
                    },
                    quote! {
                        Self::#ident {
                            #to_fields_destructure
                        } => {
                            #index.to_buf(buf);
                            #to_fields_body
                        }
                    },
                )
            }
            Fields::Unnamed(unnamed_fields) => {
                let from_body = impl_from_unnamed_fields(unnamed_fields);
                let (destructure_body, to_body) = impl_to_unnamed_fields(unnamed_fields);

                let from = quote! {
                    #index => {
                        Ok(Self::#ident (
                            #from_body
                        ))
                    }
                };

                let to = quote! {
                    Self::#ident (
                        #destructure_body
                    ) => {
                        #index.to_buf(buf);
                        #to_body
                    }
                };

                (from, to)
            }
            Fields::Unit => impl_unit_fields(ident, index),
        };
        from_buf_match_tokens.push(from);
        to_buf_match_tokens.push(to);
    }

    quote! {
        impl serbytes::prelude::SerBytes for #enum_name {
            fn from_buf(buf: &mut serbytes::prelude::ByteBuffer) -> std::io::Result<Self> {
                let index = u8::from_buf(buf)?;
                match index {
                    #(#from_buf_match_tokens)*
                    _ => {
                        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
                    }
                }
            }

            fn to_buf(&self, buf: &mut serbytes::prelude::ByteBuffer) {
                match self {
                    #(#to_buf_match_tokens)*
                }
            }
        }
    }
}

pub(super) fn impl_unit_fields(
    variant_name: &Ident,
    index: u8,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let from = quote! {
        #index => {
            Ok(Self::#variant_name)
        }
    };

    let to = quote! {
        Self::#variant_name => {
            #index.to_buf(buf);
        }
    };

    (from, to)
}
