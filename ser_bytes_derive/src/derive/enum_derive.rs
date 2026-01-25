use crate::derive::shared::named_fields::{
    impl_approx_size_named_fields, impl_from_named_fields, impl_to_named_fields,
};
use crate::derive::shared::unnamed_fields::{
    impl_approx_size_unnamed_fields, impl_from_unnamed_fields, impl_to_unnamed_fields,
};
use proc_macro2::Ident;
use quote::quote;
use syn::{DataEnum, Fields, Variant};

pub(super) fn impl_derive_enum(enum_data: DataEnum, enum_name: Ident) -> proc_macro2::TokenStream {
    let mut from_buf_match_tokens = Vec::new();
    let mut to_buf_match_tokens = Vec::new();
    let mut approx_size_match_tokens = Vec::new();

    for (index, variant) in enum_data.variants.iter().enumerate() {
        let index = index as u8;
        let Variant { fields, ident, .. } = variant;

        let (from, to, approx_size_body) = match fields {
            Fields::Named(named_fields) => {
                let from_fields_body = impl_from_named_fields(named_fields);
                let (to_fields_destructure, to_fields_body) = impl_to_named_fields(named_fields);
                let (approx_size_destructure, approx_size_body) =
                    impl_approx_size_named_fields(named_fields);

                let from = quote! {
                    #index => {
                        Ok(Self::#ident {
                            #from_fields_body
                        })
                    }
                };

                let to = quote! {
                    Self::#ident {
                        #to_fields_destructure
                    } => {
                        #index.to_buf(buf);
                        #to_fields_body
                    }
                };

                let approx_size = quote! {
                    Self::#ident {
                        #approx_size_destructure
                    } => {
                        #index.approx_size() + #approx_size_body
                    }
                };

                (from, to, approx_size)
            }
            Fields::Unnamed(unnamed_fields) => {
                let from_body = impl_from_unnamed_fields(unnamed_fields);
                let (destructure_body, to_body) = impl_to_unnamed_fields(unnamed_fields);
                let approx_size_body = impl_approx_size_unnamed_fields(unnamed_fields);

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

                let approx_size = quote! {
                    Self::#ident (
                        #destructure_body
                    ) => {
                        #approx_size_body
                    }
                };

                (from, to, approx_size)
            }
            Fields::Unit => impl_unit_fields(ident, index),
        };

        from_buf_match_tokens.push(from);
        to_buf_match_tokens.push(to);
        approx_size_match_tokens.push(approx_size_body);
    }

    quote! {
        impl serbytes::prelude::SerBytes for #enum_name {
            fn from_buf(buf: &mut serbytes::prelude::ReadByteBuffer) -> serbytes::prelude::BBReadResult<Self> {
                let index = u8::from_buf(buf)?;
                match index {
                    #(#from_buf_match_tokens)*

                    _ => {
                        Err(serbytes::prelude::ReadError::new(format!("Error reading: enum index, invalid index read; got: {}", index)))
                    }
                }
            }

            fn to_buf(&self, buf: &mut serbytes::prelude::WriteByteBuffer) {
                match self {
                    #(#to_buf_match_tokens)*
                }
            }

            fn size_hint() -> usize
            where
                Self: Sized
            {
                u8::size_hint()
            }

            fn approx_size(&self) -> usize {
                match self {
                    #(#approx_size_match_tokens)*
                }
            }
        }
    }
}

pub(super) fn impl_unit_fields(
    variant_name: &Ident,
    index: u8,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
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

    let approx_size = quote! {
        Self::#variant_name => {
            0
        }
    };

    (from, to, approx_size)
}
