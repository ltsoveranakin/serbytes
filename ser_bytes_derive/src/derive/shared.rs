use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Field, FieldsNamed, FieldsUnnamed};

pub(super) fn impl_from_named_fields(named_fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let mut from_body = Vec::new();

    for field in &named_fields.named {
        let Field { ident, .. } = field;
        from_body.push(quote! {
            #ident: serdata::prelude::from_buf(buf)?
        });
    }

    quote! {
        #(#from_body),*
    }
}

pub(super) fn impl_to_named_fields(
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

pub(super) fn impl_from_unnamed_fields(named_fields: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let mut from_body = Vec::new();

    for _ in &named_fields.unnamed {
        from_body.push(quote! {
            serdata::prelude::from_buf(buf)?
        });
    }

    let from = quote! {
        #(#from_body),*
    };

    from
}

pub(super) fn impl_to_unnamed_fields(
    named_fields: &FieldsUnnamed,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut to_destructure_body = Vec::new();
    let mut to_body = Vec::new();

    let mut i = 0;

    for _ in &named_fields.unnamed {
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
