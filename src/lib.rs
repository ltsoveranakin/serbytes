extern crate core;

mod bytebuffer;

pub mod prelude;
mod ser_bytes_impl;

#[cfg(feature = "fs")]
mod fs;
mod ser_trait;
mod size;
