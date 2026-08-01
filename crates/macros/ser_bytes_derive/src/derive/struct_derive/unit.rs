use crate::derive::shared::FunctionBodies;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub(super) fn derive_unit(struct_name: &Ident) -> FunctionBodies<TokenStream> {
    FunctionBodies {
        from_function_body: quote! {
            Ok(#struct_name)
        },
        to_function_body: TokenStream::new(),
        approx_size_function_body: quote! {
            0
        },
        size_hint_function_body: quote! {
            0
        },
    }
}
