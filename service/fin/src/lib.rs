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
extern crate paseto;

#[macro_use]
mod std_ext;
mod errors;
mod global;
mod ticker;

// has deps
mod algo;
mod backend;
mod data;
mod portfolio;

pub mod server;
