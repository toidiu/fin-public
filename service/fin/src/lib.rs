#![feature(plugin, custom_derive)]
#![feature(proc_macro_gen)]
#![feature(nll)]
#![plugin(rocket_codegen)]
#![allow(unused)]
#![feature(custom_attribute)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use crate::{data::*, ticker::TickerId};
use postgres::{Connection, TlsMode};

#[macro_use]
mod std_ext;
mod api;
mod data;
mod errors;
// mod iex;
mod models;
mod portfolio;
mod server2;
mod ticker;

pub fn bla() {
    server2::start_server();
}

pub fn test_db() {
    let database_url = "postgres://postgres@localhost:5432/test-fin";
    let conn = Connection::connect(database_url, TlsMode::None)
        .expect("unable to connect to postgres");

    let max_count = 100;
    let time_to_live = ::std::time::Duration::from_millis(100);
    let lru_cache = lru_time_cache::LruCache::<String, f32>::with_expiry_duration_and_capacity(
        time_to_live,
        max_count,
    );
    let db = data::PgTickerDatabase {
        conn: conn,
        lru: lru_cache,
    };

    info!("asdf");
    // TESTED manually
    db.get_user(&"toidiu".to_owned(), &"123456".to_owned())
        .unwrap();
    db.get_ticker_actual(&1, &1).unwrap();
    db.get_tickers_data(&vec![1, 2, 3]).unwrap();
    db.get_port_goal(&1).unwrap();
    db.get_ticker_goal(&1).unwrap();
}
