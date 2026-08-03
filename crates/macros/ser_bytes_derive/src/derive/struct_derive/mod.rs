pub mod named;
pub mod unit;
pub mod unnamed;

use crate::derive::shared::FunctionBodies;
use crate::derive::shared::define_with_vars::define_with_vars;
use crate::derive::struct_derive::named::derive_named;
use crate::derive::struct_derive::unit::derive_unit;
use crate::derive::struct_derive::unnamed::derive_unnamed;
use syn::{DataStruct, Fields, Generics};

pub(super) fn impl_derive_struct(
    struct_data: DataStruct,
    struct_name: proc_macro2::Ident,
    generics: Generics,
) -> proc_macro2::TokenStream {
    let bodies = get_function_bodies(&struct_name, &struct_data.fields);

    define_with_vars(generics, struct_name, bodies, struct_data.fields.is_empty())
}

fn get_function_bodies(
    struct_name: &proc_macro2::Ident,
    struct_fields: &Fields,
) -> FunctionBodies<proc_macro2::TokenStream> {
    let FunctionBodies {
        from_function_body,
        to_function_body,
        approx_size_function_body,
        size_hint_function_body,
    } = match struct_fields {
        Fields::Named(named_fields) => derive_named(&struct_name, &named_fields),
        Fields::Unnamed(unnamed_fields) => derive_unnamed(&struct_name, &unnamed_fields),
        Fields::Unit => derive_unit(&struct_name),
    };

    FunctionBodies {
        from_function_body,
        to_function_body,
        approx_size_function_body,
        size_hint_function_body,
    }
}
