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
extern crate log;
#[macro_use]
extern crate postgres_mapper_derive;

#[macro_use]
mod std_ext;
mod algo;
mod backend;
mod data;
mod errors;
mod portfolio;
mod ticker;

pub mod server;
