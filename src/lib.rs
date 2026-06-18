extern crate core;

pub mod prelude;

mod bytebuffer;
#[cfg(feature = "fs")]
mod fs;
mod mapped;
mod ser_bytes_impl;
mod ser_trait;
mod size;
