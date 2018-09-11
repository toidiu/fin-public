#![feature(plugin)]
#![feature(proc_macro_gen)]
#![feature(nll)]
#![plugin(rocket_codegen)]
#![allow(unused)]

#[macro_use]
extern crate serde_derive;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use crate::data::*;

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
    let d = portfolio2::get_data(db);
    serde_json::to_string(&d).unwrap()
}

fn main() {
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

mod logic {

    use crate::data;
    use crate::portfolio2;
    use crate::portfolio2::*;

    fn calculate() {
        let db = data::DefaultTickerDatabase {};
        let port = portfolio2::get_data(db);

        let is_stock_greater = port.determine_action();
        match is_stock_greater {
            StockBondAction::BuyStock => (),

            StockBondAction::BuyBond => (),

            StockBondAction::BuyEither => (),
        };

        // calculate gTn%-aTn% for each ticker
        // buy the one with the largest difference
        // if difference for all is within q% then buy cheapest first
    }

}
