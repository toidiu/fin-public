#![allow(unused)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate postgres_mapper_derive;

use crate::{data::*, portfolio::TickerId};
use postgres::{Connection, TlsMode};

#[macro_use]
mod std_ext;
mod api;
mod buy_next;
mod data;
mod errors;
mod portfolio;
mod ticker;

pub mod server;
