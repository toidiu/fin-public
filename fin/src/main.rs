#![feature(plugin)]
#![feature(proc_macro_gen)]
#![feature(nll)]
#![plugin(rocket_codegen)]
#![allow(unused)]
#![feature(custom_attribute)]

#[macro_use]
extern crate derivative;

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

macro_rules! matches(
    ($e:expr, $p:pat) => (
        match $e {
            $p => true,
            _ => false
        }
    )
);

mod logic {

    use crate::data;
    use crate::data::TickerDatabase;
    use crate::portfolio2;
    use crate::portfolio2::*;

    pub fn next_buy() {
        let db = data::DefaultTickerDatabase {};
        let port = portfolio2::get_data(db);

        let is_stock_greater = port.determine_action();
        match is_stock_greater {
            StockBondAction::BuyStock => (),

            StockBondAction::BuyBond => (),

            StockBondAction::BuyEither => (),
        };

        // calculate gTn%-aTn% for each ticker
        let diff = port.calculate_ticker_diff();

        println!("{}", serde_json::to_string_pretty(&diff).unwrap());

        // FIXME also check that stock % is met

        // filter if there is a Buy (difference is greater than deviation)
        let contains_buy = diff
            .iter()
            .filter(|x| matches!(x.action, portfolio2::TickerAction::Buy))
            .collect::<Vec<&TickerDiff>>()
            .is_empty();

        let action = if (contains_buy) {
            // if no Buy then buy cheapest first
            diff.into_iter().fold(TickerDiff::empty(), |x, y| {
                // db.get_ticker(&x.symbol).price;
                // db.get_ticker(&y.symbol).price;

                // FIXME compare price and get cheapest one
                // if (x.goal_minus_actual > y.goal_minus_actual) {
                x
                // } else {
                //     y
                // }
            })
        } else {
            // else buy the one with the largest difference
            diff.into_iter().fold(TickerDiff::empty(), |x, y| {
                if (x.goal_minus_actual > y.goal_minus_actual) {
                    x
                } else {
                    y
                }
            })
        };

        println!("{:#?}", action);
    }

}
