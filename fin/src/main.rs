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

        // println!("{}", serde_json::to_string_pretty(&diff).unwrap());

        let empty_diff = EMPTY_TICKER_DIFF.clone();

        // stock or bond
        match port.meta.stock_action {
            StockBondAction::BuyStock => (),
            FIXME filter out stock or bond or none
            FIXME filter out buy or if none then filter out cheapest
            StockBondAction::BuyBond => (),
            StockBondAction::BuyEither => (),
        };

        // let action = if (port.meta.contains_buy) {
        //     // if no Buy then buy cheapest first
        //     port.meta.ticker_diffs.into_iter().fold(empty_diff, |x, y| {
        //         if (x.symbol == EMPTY_TICKER_DIFF.symbol) {
        //             y
        //         } else if (db.get_ticker(&x.symbol).price <= db.get_ticker(&y.symbol).price) {
        //             x
        //         } else {
        //             y
        //         }
        //     })
        // } else {
        //     // else buy the one with the largest difference
        //     port.meta.ticker_diffs.into_iter().fold(empty_diff, |x, y| {
        //         if (x.goal_minus_actual > y.goal_minus_actual) {
        //             x
        //         } else {
        //             y
        //         }
        //     })
        // };

        // println!("{:#?}", action);
    }

}
