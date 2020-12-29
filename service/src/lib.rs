#![allow(unused)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate slog;

#[macro_use]
extern crate postgres_mapper_derive;

#[macro_use]
extern crate fin_core;

mod global;

// benchmark using criterion
pub mod backend;

// has deps
mod data;
pub mod server;
