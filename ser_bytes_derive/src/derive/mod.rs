mod enum_derive;
mod shared;
mod struct_derive;

use crate::derive::enum_derive::impl_derive_enum;
use crate::derive::struct_derive::impl_derive_struct;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input, parse_quote};

pub fn ser_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        data,
        ident,
        mut generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    for param in generics.type_params_mut() {
        param.bounds.push(parse_quote!(SerBytes));
    }

    let tokens = match data {
        Data::Struct(struct_data) => impl_derive_struct(struct_data, ident, generics),
        Data::Enum(enum_data) => impl_derive_enum(enum_data, ident, generics),
        Data::Union(_) => {
            quote! {
                compile_error!("Unions not supported");
            }
        }
    };

    tokens.into()
}
