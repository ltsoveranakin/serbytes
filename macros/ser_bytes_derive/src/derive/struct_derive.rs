use crate::derive::shared::FunctionBodies;
use crate::derive::shared::named_fields::{
    ToBufTokens, impl_approx_size_named_fields, impl_from_named_fields, impl_to_named_fields,
};
use crate::derive::shared::unnamed_fields::{
    impl_approx_size_unnamed_fields, impl_from_unnamed_fields, impl_to_unnamed_fields,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataStruct, Fields, Generics};

pub(super) fn impl_derive_struct(
    struct_data: DataStruct,
    struct_name: Ident,
    generics: Generics,
) -> TokenStream {
    let DataStruct { fields, .. } = struct_data;

    let FunctionBodies {
        from_function_body,
        to_function_body,
        approx_size_function_body,
    } = match &fields {
        Fields::Named(named_fields) => {
            // if let Err(e) = verify_may_not_exist_attributes(named_fields) {
            //     return e.into_compile_error().into();
            // };

            let from_body = impl_from_named_fields(named_fields);
            let ToBufTokens { destructure, body } = impl_to_named_fields(named_fields);
            let approx_size_body = impl_approx_size_named_fields(named_fields);

            let from_function_body = quote! {
                Ok(Self {
                    #from_body
                })
            };

            let to_function_body = quote! {
                let Self {
                    #destructure
                } = self;

                #body
            };

            let approx_size_function_body = quote! {
                let Self {
                    #destructure
                } = self;

                #approx_size_body
            };

            FunctionBodies {
                from_function_body,
                to_function_body,
                approx_size_function_body,
            }
        }
        Fields::Unnamed(unnamed_fields) => {
            let from_body = impl_from_unnamed_fields(unnamed_fields);
            let ToBufTokens { destructure, body } = impl_to_unnamed_fields(unnamed_fields);
            let approx_size_body = impl_approx_size_unnamed_fields(unnamed_fields);

            FunctionBodies {
                from_function_body: quote! {
                    Ok(Self(#from_body))
                },
                to_function_body: quote! {
                    let Self(#destructure) = self;

                    #body
                },
                approx_size_function_body: quote! {
                     let Self(#destructure) = self;

                    #approx_size_body
                },
            }
        }
        Fields::Unit => FunctionBodies {
            from_function_body: quote! {
                Ok(Self)
            },
            to_function_body: TokenStream::new(),
            approx_size_function_body: quote! {
                0
            },
        },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics serbytes::prelude::SerBytes for #struct_name #ty_generics #where_clause{
            fn from_buf(buf: &mut serbytes::prelude::ReadByteBufferRefMut) -> serbytes::prelude::BBReadResult<Self> {
                #from_function_body
            }

            fn to_buf(&self, buf: &mut serbytes::prelude::WriteByteBufferOwned) {
                #to_function_body
            }

            fn size_hint() -> usize
            where
                Self: Sized,
            {
                0
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
