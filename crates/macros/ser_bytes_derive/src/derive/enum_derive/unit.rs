use crate::derive::shared::FunctionBodies;
use proc_macro2::Ident;
use quote::quote;

pub(super) fn derive_unit(variant_name: &Ident, index: u8) -> FunctionBodies<()> {
    let from_function_body = quote! {
        #index => {
            Ok(Self::#variant_name)
        }
    };

    let to_function_body = quote! {
        Self::#variant_name => {
            #index.to_buf(buf);
        }
    };

    let approx_size_function_body = quote! {
        Self::#variant_name => {
            0
        }
    };

    FunctionBodies {
        from_function_body,
        to_function_body,
        approx_size_function_body,
        size_hint_function_body: (),
    }
}
