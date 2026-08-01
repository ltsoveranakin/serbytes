pub mod named;
pub mod unit;
pub mod unnamed;

use crate::derive::shared::FunctionBodies;
use crate::derive::struct_derive::named::derive_named;
use crate::derive::struct_derive::unit::derive_unit;
use crate::derive::struct_derive::unnamed::derive_unnamed;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataStruct, Fields, Generics};

pub(super) fn impl_derive_struct(
    struct_data: DataStruct,
    struct_name: Ident,
    generics: Generics,
) -> TokenStream {
    let DataStruct { fields, .. } = &struct_data;

    let FunctionBodies {
        from_function_body,
        to_function_body,
        approx_size_function_body,
        size_hint_function_body,
    } = match fields {
        Fields::Named(named_fields) => derive_named(&struct_name, named_fields),

        Fields::Unnamed(unnamed_fields) => derive_unnamed(&struct_name, unnamed_fields),

        Fields::Unit => derive_unit(&struct_name),
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics serbytes::prelude::SerBytes for #struct_name #ty_generics #where_clause{
            fn from_buf(buf: &mut bytebuffer::prelude::ReadByteBufferRefMut) -> bytebuffer::prelude::BBReadResult<Self> {
                let mut inner = || {
                    #from_function_body
                };

                bytebuffer::prelude::WithParent::with_parent(inner(), stringify!(#struct_name))
            }

            fn to_buf(&self, buf: &mut bytebuffer::prelude::WriteByteBufferOwned) {
                buf.reserve(Self::approx_size(self));
                #to_function_body
            }

            fn size_hint() -> usize
            where
                Self: Sized,
            {
                #size_hint_function_body
            }

            fn approx_size(&self) -> usize {
                #approx_size_function_body
            }
        }
    }
}

// For future :p

// fn verify_may_not_exist_attributes(named_fields: &FieldsNamed) -> syn::Result<()> {
//     let mut might_not_exist_declared = false;
//
//     for field in &named_fields.named {
//         if has_may_not_exist_attribute(&field.attrs) {
//             might_not_exist_declared = true;
//             if let Type::Path(type_path) = &field.ty {
//                 if !type_path
//                     .path
//                     .segments
//                     .last()
//                     .is_some_and(|segment| segment.ident == "Option")
//                 {
//                     return Err(syn::Error::new_spanned(
//                         field,
//                         "Fields with the #[may_not_exist] attribute must be of type Option",
//                     ));
//                 }
//             }
//         } else {
//             if might_not_exist_declared {
//                 return Err(syn::Error::new_spanned(
//                     field,
//                     "Fields without the #[may_not_exist] attribute cannot occur after fields with the #[may_not_exist] attribute",
//                 ));
//             }
//         }
//     }
//
//     Ok(())
// }
//
// fn has_may_not_exist_attribute(attributes: &[Attribute]) -> bool {
//     for attribute in attributes.iter() {
//         if attribute.path().is_ident("may_not_exist") {
//             return true;
//         }
//     }
//
//     false
// }
