use crate::derive::shared::named_fields::{
    impl_approx_size_named_fields, impl_from_named_fields, impl_to_named_fields, ToBufTokens,
};
use crate::derive::shared::unnamed_fields::{
    impl_approx_size_unnamed_fields, impl_from_unnamed_fields, impl_to_unnamed_fields,
};
use crate::derive::shared::FunctionBodies;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataStruct, Fields};

pub(super) fn impl_derive_struct(struct_data: DataStruct, struct_name: Ident) -> TokenStream {
    let DataStruct { fields, .. } = struct_data;

    let FunctionBodies {
        from_function_body,
        to_function_body,
        approx_size_function_body,
    } = match &fields {
        Fields::Named(named_fields) => {
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

    quote! {
        impl serbytes::prelude::SerBytes for #struct_name {
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
