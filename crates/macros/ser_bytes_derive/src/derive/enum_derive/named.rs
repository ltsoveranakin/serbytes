use crate::derive::shared::FunctionBodies;
use crate::derive::shared::named_fields::{
    ToBufTokens, impl_approx_size_named_fields, impl_from_named_fields, impl_to_named_fields,
};
use proc_macro2::Ident;
use quote::quote;
use syn::FieldsNamed;

pub(super) fn derive_named(
    variant_ident: &Ident,
    index: u8,
    named_fields: &FieldsNamed,
) -> FunctionBodies<()> {
    let from_fields_body = impl_from_named_fields(named_fields);
    let ToBufTokens { destructure, body } = impl_to_named_fields(named_fields);
    let approx_size_body = impl_approx_size_named_fields(named_fields);

    let from_function_body = quote! {
        #index => {
            Ok(Self::#variant_ident {
                #from_fields_body
            })
        }
    };

    let to_function_body = quote! {
        Self::#variant_ident {
            #destructure
        } => {
            #index.to_buf(buf);
            #body
        }
    };

    let approx_size_function_body = quote! {
        Self::#variant_ident {
            #destructure
        } => {
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
