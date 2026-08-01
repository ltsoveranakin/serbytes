use crate::derive::shared::named_fields::{
    ToBufTokens, impl_approx_size_named_fields, impl_from_named_fields, impl_to_named_fields,
};
use crate::derive::shared::{FunctionBodies, impl_size_hint};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::FieldsNamed;

pub(super) fn derive_named(
    struct_name: &Ident,
    named_fields: &FieldsNamed,
) -> FunctionBodies<TokenStream> {
    let from_body = impl_from_named_fields(named_fields);
    let ToBufTokens { destructure, body } = impl_to_named_fields(named_fields);
    let approx_size_body = impl_approx_size_named_fields(named_fields);
    let size_hint_function_body = impl_size_hint(&named_fields.named);

    let from_function_body = quote! {
        Ok(#struct_name {
            #from_body
        })
    };

    let to_function_body = quote! {
        let #struct_name {
            #destructure
        } = self;

        #body
    };

    let approx_size_function_body = quote! {
        let #struct_name {
            #destructure
        } = self;

        #approx_size_body
    };

    FunctionBodies {
        from_function_body,
        to_function_body,
        approx_size_function_body,
        size_hint_function_body,
    }
}
