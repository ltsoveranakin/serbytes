mod named;
mod unit;
mod unnamed;

use crate::derive::enum_derive::named::derive_named;
use crate::derive::enum_derive::unit::derive_unit;
use crate::derive::enum_derive::unnamed::derive_unnamed;
use crate::derive::shared::FunctionBodies;
use proc_macro2::Ident;
use quote::quote;
use syn::{DataEnum, Fields, Generics, Variant};

pub(super) fn impl_derive_enum(
    enum_data: DataEnum,
    enum_name: Ident,
    generics: Generics,
) -> proc_macro2::TokenStream {
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

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics serbytes::prelude::SerBytes for #enum_name #ty_generics #where_clause{
            fn from_buf(buf: &mut bytebuffer::prelude::ReadByteBufferRefMut) -> bytebuffer::prelude::BBReadResult<Self> {
                let mut inner = || {
                    let index = bytebuffer::prelude::WithParent::with_parent(u8::from_buf(buf), "Enum index")?;

                    match index {
                        #(#from_buf_match_tokens)*

                        _ => {
                            Err(bytebuffer::prelude::ReadError::new(bytebuffer::prelude::SpecificError::Other("Enum index out of bounds".into()), stringify!(#enum_name), None))
                        }
                    }
                };

                bytebuffer::prelude::WithParent::with_parent(inner(), stringify!(#enum_name))
            }

            fn to_buf(&self, buf: &mut bytebuffer::prelude::WriteByteBufferOwned) {
                buf.reserve(Self::approx_size(self));

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
                let content_size = match self {
                    #(#approx_size_match_tokens)*
                };

                u8::size_hint() + content_size
            }
        }
    }
}
