use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Field, Token};

pub(crate) mod named_fields;
pub(crate) mod unnamed_fields;

pub(super) struct FunctionBodies<H> {
    pub(super) from_function_body: proc_macro2::TokenStream,
    pub(super) to_function_body: proc_macro2::TokenStream,
    pub(super) approx_size_function_body: proc_macro2::TokenStream,
    pub(super) size_hint_function_body: H,
}

pub(super) fn impl_size_hint(
    named_fields: &Punctuated<Field, Token![,]>,
) -> proc_macro2::TokenStream {
    let mut size_hint_body_tokens = Vec::new();

    for field in named_fields {
        let Field { ty, .. } = field;

        size_hint_body_tokens.push(quote! {
               <#ty as serbytes::prelude::SerBytes>::size_hint()
        });
    }

    let approx_size_body = quote! {
        #(#size_hint_body_tokens)+*
    };

    approx_size_body
}
