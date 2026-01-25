use quote::quote;
use syn::{Field, FieldsNamed};

pub(crate) fn impl_from_named_fields(named_fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let mut from_body = Vec::new();

    for field in &named_fields.named {
        let Field { ident, .. } = field;
        from_body.push(quote! {
            #ident: serbytes::prelude::from_buf(buf)?
        });
    }

    quote! {
        #(#from_body),*
    }
}

pub(crate) fn impl_to_named_fields(
    named_fields: &FieldsNamed,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut to_destructure_body = Vec::new();
    let mut to_body = Vec::new();

    for field in &named_fields.named {
        let Field { ident, .. } = field;

        to_destructure_body.push(quote! {
            #ident
        });

        to_body.push(quote! {
            #ident.to_buf(buf);
        });
    }

    let destructure = quote! {
        #(#to_destructure_body),*
    };

    let body = quote! {
        #(#to_body)*
    };

    (destructure, body)
}

pub(crate) fn impl_approx_size_named_fields(
    named_fields: &FieldsNamed,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut approx_size_body_tokens = Vec::new();
    let mut approx_size_destructure_body_tokens = Vec::new();

    for field in &named_fields.named {
        let Field { ident, .. } = field;

        approx_size_body_tokens.push(quote! {
            #ident.approx_size()
        });

        approx_size_destructure_body_tokens.push(quote! {
            #ident
        });
    }

    let approx_size_body = quote! {
        #(#approx_size_body_tokens)+*
    };

    let approx_size_destructure_body = quote! {
        #(#approx_size_destructure_body_tokens),*
    };

    (approx_size_destructure_body, approx_size_body)
}
