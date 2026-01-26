use crate::derive::shared::named_fields::{
    impl_approx_size_named_fields, impl_from_named_fields, impl_to_named_fields, ToBufTokens,
};
use crate::derive::shared::unnamed_fields::{
    impl_approx_size_unnamed_fields, impl_from_unnamed_fields, impl_to_unnamed_fields,
};
use crate::derive::shared::FunctionBodies;
use proc_macro2::Ident;
use quote::quote;
use syn::{DataEnum, Fields, Variant};

pub(super) fn impl_derive_enum(enum_data: DataEnum, enum_name: Ident) -> proc_macro2::TokenStream {
    let mut from_buf_match_tokens = Vec::new();
    let mut to_buf_match_tokens = Vec::new();
    let mut approx_size_match_tokens = Vec::new();

    for (index, variant) in enum_data.variants.iter().enumerate() {
        let index = index as u8;
        let Variant { fields, ident, .. } = variant;

        let FunctionBodies {
            from_function_body,
            to_function_body,
            approx_size_function_body,
        } = match fields {
            Fields::Named(named_fields) => {
                let from_fields_body = impl_from_named_fields(named_fields);
                let ToBufTokens { destructure, body } = impl_to_named_fields(named_fields);
                let approx_size_body = impl_approx_size_named_fields(named_fields);

                let from_function_body = quote! {
                    #index => {
                        Ok(Self::#ident {
                            #from_fields_body
                        })
                    }
                };

                let to_function_body = quote! {
                    Self::#ident {
                        #destructure
                    } => {
                        #index.to_buf(buf);
                        #body
                    }
                };

                let approx_size_function_body = quote! {
                    Self::#ident {
                        #destructure
                    } => {
                        #index.approx_size() + #approx_size_body
                    }
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

                let from_function_body = quote! {
                    #index => {
                        Ok(Self::#ident (
                            #from_body
                        ))
                    }
                };

                let to_function_body = quote! {
                    Self::#ident (
                        #destructure
                    ) => {
                        #index.to_buf(buf);
                        #body
                    }
                };

                let approx_size_function_body = quote! {
                    Self::#ident (
                        #destructure
                    ) => {
                        #approx_size_body
                    }
                };

                FunctionBodies {
                    from_function_body,
                    to_function_body,
                    approx_size_function_body,
                }
            }
            Fields::Unit => impl_unit_fields(ident, index),
        };

        from_buf_match_tokens.push(from_function_body);
        to_buf_match_tokens.push(to_function_body);
        approx_size_match_tokens.push(approx_size_function_body);
    }

    quote! {
        impl serbytes::prelude::SerBytes for #enum_name {
            fn from_buf(buf: &mut serbytes::prelude::ReadByteBufferRefMut) -> serbytes::prelude::BBReadResult<Self> {
                let index = u8::from_buf(buf)?;
                match index {
                    #(#from_buf_match_tokens)*

                    _ => {
                        Err(serbytes::prelude::ReadError::new(format!("Error reading: enum index, invalid index read; got: {}", index)))
                    }
                }
            }

            fn to_buf(&self, buf: &mut serbytes::prelude::WriteByteBufferOwned) {
                match self {
                    #(#to_buf_match_tokens)*
                }
            }

            fn size_hint() -> usize
            where
                Self: Sized
            {
                u8::size_hint()
            }

            fn approx_size(&self) -> usize {
                match self {
                    #(#approx_size_match_tokens)*
                }
            }
        }
    }
}

pub(super) fn impl_unit_fields(variant_name: &Ident, index: u8) -> FunctionBodies {
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
    }
}
