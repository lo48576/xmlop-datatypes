//! Basic datatypes for xmlop.
#![warn(missing_docs)]

#[cfg(feature = "nom-4")]
#[macro_use]
extern crate nom;
extern crate opaque_typedef;
#[macro_use]
extern crate opaque_typedef_macros;

pub mod strings;
