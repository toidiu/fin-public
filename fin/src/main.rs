#![feature(plugin)]
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

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use crate::data::*;

#[macro_use]
mod macros;
mod data;
mod portfolio1;
mod portfolio2;
mod ticker;

#[get("/")]
fn index() -> String {
    let d = portfolio1::get_data_portfolio1();
    serde_json::to_string(&d).unwrap()
}

#[get("/2")]
fn two() -> String {
    let db = data::DefaultTickerDatabase {};
    let d = portfolio2::get_data(&db);
    serde_json::to_string(&d).unwrap()
}

fn start_server() {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:1234"]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: false,
        ..Default::default()
    };

    rocket::ignite()
        .mount("/", routes![index, two])
        .attach(options)
        .launch();
}

fn main() {
    logic::next_buy();

    // start_server();
}

mod logic {

    use crate::data;
    use crate::data::TickerDatabase;
    use crate::portfolio2;
    use crate::portfolio2::*;

    pub fn next_buy() {
        let db = data::DefaultTickerDatabase {};
        let mut port = portfolio2::get_data(&db);

        // update meta data based on ticker price and percent
        port.update_portfolio();
        // println!("{}", serde_json::to_string_pretty(&port.meta.tickers_diff).unwrap());

        let buy_next = port.filter_based_on_stock_action();
        println!("{:#?}", buy_next);
        println!("{}", serde_json::to_string_pretty(&port).unwrap());
    }

}
