mod derive;

extern crate proc_macro;

use crate::derive::ser_derive;

#[proc_macro_derive(SerBytes)]
pub fn derive_ser_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ser_derive(input)
}
