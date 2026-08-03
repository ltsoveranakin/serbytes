use crate::derive::shared::FunctionBodies;
use crate::derive::shared::named_fields::ToBufTokens;
use crate::derive::shared::unnamed_fields::{
    impl_approx_size_unnamed_fields, impl_from_unnamed_fields, impl_to_unnamed_fields,
};
use proc_macro2::Ident;
use quote::quote;
use syn::FieldsUnnamed;

pub(super) fn derive_unnamed(
    variant_ident: &Ident,
    index: u8,
    unnamed_fields: &FieldsUnnamed,
) -> FunctionBodies<()> {
    let from_body = impl_from_unnamed_fields(unnamed_fields);
    let ToBufTokens { destructure, body } = impl_to_unnamed_fields(unnamed_fields);
    let approx_size_body = impl_approx_size_unnamed_fields(unnamed_fields);

    let from_function_body = quote! {
        #index => {
            Ok(Self::#variant_ident (
                #from_body
            ))
        }
    };

    let to_function_body = quote! {
        Self::#variant_ident (
            #destructure
        ) => {
            serbytes::prelude::to_buf(buf, &#index);
            #body
        }
    };

    let approx_size_function_body = quote! {
        Self::#variant_ident (
            #destructure
        ) => {
            #approx_size_body
        }
    };

    FunctionBodies {
        from_function_body,
        to_function_body,
        approx_size_function_body,
        size_hint_function_body: (),
    }
}
