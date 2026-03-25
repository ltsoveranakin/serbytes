pub(crate) mod named_fields;
pub(crate) mod unnamed_fields;

pub(crate) struct FunctionBodies {
    pub(crate) from_function_body: proc_macro2::TokenStream,
    pub(crate) to_function_body: proc_macro2::TokenStream,
    pub(crate) approx_size_function_body: proc_macro2::TokenStream,
}
