use crate::derive::shared::FunctionBodies;
use quote::quote;
use syn::Generics;

pub(crate) fn define_with_vars(
    generics: Generics,
    ty_name: proc_macro2::Ident,
    function_bodies: FunctionBodies<proc_macro2::TokenStream>,
    is_buf_unused: bool,
) -> proc_macro2::TokenStream {
    let FunctionBodies {
        from_function_body,
        to_function_body,
        approx_size_function_body,
        size_hint_function_body,
    } = function_bodies;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let buf_name = if is_buf_unused {
        quote! {
            _buf
        }
    } else {
        quote! {
            buf
        }
    };

    quote! {
        impl #impl_generics serbytes::prelude::SerBytes for #ty_name #ty_generics #where_clause {
            fn from_buf(#buf_name: &mut serbytes::prelude::ReadByteBufferRefMut) -> serbytes::prelude::BBReadResult<Self>
            where
                Self: Sized,
            {
                #from_function_body
            }

            fn to_buf(&self, #buf_name: &mut serbytes::prelude::WriteByteBufferOwned) {
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
