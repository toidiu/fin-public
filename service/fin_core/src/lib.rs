#![allow(unused)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;

// benchmark using criterion
#[macro_use]
pub mod std_ext;
pub mod algo;
pub mod portfolio;
