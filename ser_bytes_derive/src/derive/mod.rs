mod enum_derive;
mod shared;
mod struct_derive;

use crate::derive::enum_derive::impl_derive_enum;
use crate::derive::struct_derive::impl_derive_struct;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn ser_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let tokens = match derive_input.data {
        Data::Enum(enum_data) => impl_derive_enum(enum_data, derive_input.ident),
        Data::Struct(struct_data) => impl_derive_struct(struct_data, derive_input.ident),
        Data::Union(_) => {
            quote! {
                compile_error!("Unions not supported");
            }
        }
    };

    tokens.into()
}
