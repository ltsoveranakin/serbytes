use crate::derive::shared::{
    impl_from_named_fields, impl_from_unnamed_fields, impl_to_named_fields, impl_to_unnamed_fields,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataStruct, Fields};

pub(super) fn impl_derive_struct(struct_data: DataStruct, struct_name: Ident) -> TokenStream {
    let DataStruct { fields, .. } = struct_data;

    let (from_body, to_body) = match &fields {
        Fields::Named(named_fields) => {
            let from_body = impl_from_named_fields(named_fields);
            let (to_destructure, to_body) = impl_to_named_fields(named_fields);
            (
                quote! {
                    Ok(Self {
                        #from_body
                    })
                },
                quote! {
                    let Self {
                        #to_destructure
                    } = self;

                    #to_body
                },
            )
        }
        Fields::Unnamed(unnamed_fields) => {
            let from_body = impl_from_unnamed_fields(unnamed_fields);
            let (to_destructure, to_body) = impl_to_unnamed_fields(unnamed_fields);
            (
                quote! {
                    Ok(Self(#from_body))
                },
                quote! {
                     let Self(#to_destructure) = self;

                    #to_body
                },
            )
        }
        Fields::Unit => (
            quote! {
                Ok(Self)
            },
            TokenStream::new(),
        ),
    };

    quote! {
        impl serbytes::prelude::SerBytes for #struct_name {
            fn from_buf(buf: &mut serbytes::prelude::ReadByteBuffer) -> serbytes::prelude::Result<Self> {
                #from_body
            }

            fn to_buf(&self, buf: &mut serbytes::prelude::WriteByteBuffer) {
                #to_body
            }
        }
    }
}
