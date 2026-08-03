use crate::derive::shared::named_fields::ToBufTokens;
use crate::derive::shared::unnamed_fields::{
    impl_approx_size_unnamed_fields, impl_from_unnamed_fields, impl_to_unnamed_fields,
};
use crate::derive::shared::{FunctionBodies, impl_size_hint};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::FieldsUnnamed;

pub(super) fn derive_unnamed(
    struct_name: &Ident,
    unnamed_fields: &FieldsUnnamed,
) -> FunctionBodies<TokenStream> {
    let from_body = impl_from_unnamed_fields(unnamed_fields);
    let ToBufTokens { destructure, body } = impl_to_unnamed_fields(unnamed_fields);
    let approx_size_body = impl_approx_size_unnamed_fields(unnamed_fields);
    let size_hint_function_body = impl_size_hint(&unnamed_fields.unnamed);

    let not_empty = !unnamed_fields.unnamed.is_empty();

    let to_function_body = if not_empty {
        quote! {
            serbytes::prelude::WriteByteBufferOwned::reserve(buf, Self::approx_size(self));

            let #struct_name(#destructure) = self;

            #body
        }
    } else {
        // Empty
        quote! {}
    };

    let approx_size_function_body = if not_empty {
        quote! {
             let #struct_name(#destructure) = self;

            #approx_size_body
        }
    } else {
        quote! {
            0
        }
    };

    FunctionBodies {
        from_function_body: quote! {
            Ok(#struct_name(#from_body))
        },
        to_function_body,
        approx_size_function_body,
        size_hint_function_body,
    }
}
