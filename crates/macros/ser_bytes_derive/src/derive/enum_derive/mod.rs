mod named;
mod unit;
mod unnamed;

use crate::derive::enum_derive::named::derive_named;
use crate::derive::enum_derive::unit::derive_unit;
use crate::derive::enum_derive::unnamed::derive_unnamed;
use crate::derive::shared::FunctionBodies;

use crate::derive::shared::define_with_vars::define_with_vars;
use quote::quote;
use syn::{DataEnum, Fields, Generics, Variant};

pub(super) fn impl_derive_enum(
    enum_data: DataEnum,
    enum_name: proc_macro2::Ident,
    generics: Generics,
) -> proc_macro2::TokenStream {
    let bodies = get_function_bodies(&enum_name, enum_data);

    define_with_vars(generics, enum_name, bodies, false)
}

fn get_function_bodies(
    enum_name: &proc_macro2::Ident,
    enum_data: DataEnum,
) -> FunctionBodies<proc_macro2::TokenStream> {
    let mut from_buf_match_tokens = Vec::new();
    let mut to_buf_match_tokens = Vec::new();
    let mut approx_size_match_tokens = Vec::new();

    for (index, variant) in enum_data.variants.iter().enumerate() {
        assert!(
            index < 128,
            "Cannot have more than 127 variants for the enum"
        );

        let index = index as u8;
        let Variant {
            fields,
            ident: variant_name,
            ..
        } = variant;

        let FunctionBodies {
            from_function_body,
            to_function_body,
            approx_size_function_body,
            ..
        } = match fields {
            Fields::Named(named_fields) => derive_named(variant_name, index, named_fields),
            Fields::Unnamed(unnamed_fields) => derive_unnamed(variant_name, index, unnamed_fields),
            Fields::Unit => derive_unit(variant_name, index),
        };

        from_buf_match_tokens.push(from_function_body);
        to_buf_match_tokens.push(to_function_body);
        approx_size_match_tokens.push(approx_size_function_body);
    }

    let u8_size_hint = quote! {
        <u8 as serbytes::prelude::SerBytes>::size_hint()
    };

    let bodies = if !enum_data.variants.is_empty() {
        let approx_size_function_body = quote! {
            let content_size = match self {
                #(#approx_size_match_tokens)*
            };

            #u8_size_hint + content_size
        };

        let to_function_body = quote! {
            serbytes::prelude::WriteByteBufferOwned::reserve(buf, Self::approx_size(self));

            match self {
                #(#to_buf_match_tokens)*
            }
        };

        let max_bound = enum_data.variants.len() as u8 - 1;

        let from_function_body = quote! {
            let mut inner = || {
                let index = serbytes::prelude::WithParent::with_parent(<u8 as serbytes::prelude::SerBytes>::from_buf(buf), "Enum index")?;

                match index {
                    #(#from_buf_match_tokens)*

                    _ => {
                        Err(
                            serbytes::prelude::ReadError::new(
                                bytebuffer::prelude::SpecificError::EnumOrdinalOutOfBounds {
                                    max_bound: #max_bound,
                                    got: index
                                },
                                stringify!(#enum_name),
                                None
                            )
                        )
                    }
                }
            };

            serbytes::prelude::WithParent::with_parent(inner(), stringify!(#enum_name))
        };

        FunctionBodies {
            from_function_body,
            approx_size_function_body,
            to_function_body,
            size_hint_function_body: (),
        }
    } else {
        let approx_size_function_body = quote! {
            #u8_size_hint
        };

        let to_function_body = quote! {
            serbytes::prelude::to_buf::<u8>(buf, &0);
        };

        let from_function_body = quote! {
            let mut inner = || {
                let _index = serbytes::prelude::WithParent::with_parent(serbytes::prelude::from_buf::<u8>(buf), "Enum index")?;

                Err(
                    serbytes::prelude::ReadError::new(
                        bytebuffer::prelude::SpecificError::InvalidEnum,
                        stringify!(#enum_name),
                        None
                    )
                )
            };

            serbytes::prelude::WithParent::with_parent(inner(), stringify!(#enum_name))
        };

        FunctionBodies {
            from_function_body,
            approx_size_function_body,
            to_function_body,
            size_hint_function_body: (),
        }
    };

    FunctionBodies {
        from_function_body: bodies.from_function_body,
        to_function_body: bodies.to_function_body,
        approx_size_function_body: bodies.approx_size_function_body,
        size_hint_function_body: quote! {
            #u8_size_hint
        },
    }
}
