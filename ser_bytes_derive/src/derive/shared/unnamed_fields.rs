use proc_macro2::{Ident, Span};
use quote::quote;
use syn::FieldsUnnamed;

pub(crate) fn impl_from_unnamed_fields(unnamed_fields: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let mut from_body = Vec::new();

    for _ in &unnamed_fields.unnamed {
        from_body.push(quote! {
            serbytes::prelude::from_buf(buf)?
        });
    }

    let from = quote! {
        #(#from_body),*
    };

    from
}

pub(crate) fn impl_to_unnamed_fields(
    unnamed_fields: &FieldsUnnamed,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut to_destructure_body = Vec::new();
    let mut to_body = Vec::new();

    let mut i = 0;

    for _ in &unnamed_fields.unnamed {
        let destructure_var = Ident::new(&format!("field{}", i), Span::call_site());

        to_destructure_body.push(quote! {
            #destructure_var
        });

        to_body.push(quote! {
            #destructure_var.to_buf(buf);
        });
        i += 1;
    }

    let destructure_body = quote! {
        #(#to_destructure_body),*
    };

    let to = quote! {
        #(#to_body)*
    };

    (destructure_body, to)
}

pub(crate) fn impl_approx_size_unnamed_fields(
    unnamed_fields: &FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let mut approx_size_body_tokens = Vec::new();

    let mut i = 0;

    for _ in &unnamed_fields.unnamed {
        let destructure_var = Ident::new(&format!("field{}", i), Span::call_site());

        approx_size_body_tokens.push(quote! {
            #destructure_var.approx_size()
        });

        i += 1;
    }

    let approx_size_body = quote! {
        #(#approx_size_body_tokens)+*
    };

    approx_size_body
}
