extern crate proc_macro;
mod derive;

use crate::derive::ser_derive;

/// Derives the SerBytes trait for the given enum or struct.
///
/// For enum derives, it uses an 8 bit integer to serialize the ordinal
/// As such, the maximum amount of variants that an enum can contain is 2^8 (256)

#[proc_macro_derive(SerBytes)]
pub fn derive_ser_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ser_derive(input)
}
