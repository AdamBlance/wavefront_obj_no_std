//! Parsers for wavefront's `.obj` and `.mtl` file format for loading meshes.
#![no_std]
#![feature(error_in_core)]
#![crate_type = "lib"]
#![deny(missing_docs)]
#![deny(unreachable_pub)]

extern crate alloc;

pub use lex::ParseError;

mod lex;
mod util;

pub mod mtl;
pub mod obj;
