#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(nll)]
#![allow(unused)]

#[macro_use]
extern crate serde_derive;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod data;
mod portfolio2;
mod ticker;

#[get("/")]
fn index() -> String {
    use crate::data::*;

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
        .mount("/", routes![index])
        .attach(options)
        .launch();
}

mod portfolio1 {
    use crate::ticker::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Portfolio {
        pub name: String,
        pub current_detail: PortfolioDetails,
        pub past_detail: Vec<PortfolioDetails>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PortfolioDetails {
        pub stocks: Vec<TickerStatus>,
        pub bonds: Vec<TickerStatus>,
        pub goal_stock_percent: f32,

        // calculated
        pub total_value: f32,
        // calculated
        pub current_stock_percent: f32,
        pub deviation_percent: f32,
    }

    impl PortfolioDetails {
        pub fn new(
            stocks: Vec<TickerStatus>,
            bonds: Vec<TickerStatus>,
            goal_stock_percent: f32,
            deviation_percent: f32,
        ) -> Self {
            PortfolioDetails {
                stocks: stocks,
                bonds: bonds,
                goal_stock_percent: goal_stock_percent,
                current_stock_percent: 0.0,
                total_value: 0.0,
                deviation_percent: deviation_percent,
            }
            // calculate total value before others
            .set_total_value()
            .set_stock_percent()
        }

        pub fn get_stock_value(&self) -> f32 {
            let s: f32 = self.stocks.iter().map(|s| s.goal.current_value).sum();
            s.into()
        }

        pub fn get_bond_value(&self) -> f32 {
            let b: f32 = self.bonds.iter().map(|s| s.goal.current_value).sum();
            b.into()
        }

        fn set_stock_percent(mut self) -> Self {
            let stocks = self.get_stock_value();
            self.current_stock_percent = (stocks / self.total_value) * 100.0;
            self
        }

        fn set_total_value(mut self) -> Self {
            let s: f32 = self.stocks.iter().map(|s| s.goal.current_value).sum();
            let b: f32 = self.bonds.iter().map(|s| s.goal.current_value).sum();

            self.total_value = (s + b).into();
            self
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TickerGoal {
        // #[serde(skip)]
        // pub symbol: TickerSymbol,
        #[serde(rename = "goalPercent")]
        pub goal_percent: f32,

        // TODO calculate this
        #[serde(rename = "currentPercent")]
        pub current_percent: f32,
        #[serde(rename = "currentValue")]
        pub current_value: f32,

        // either keep this here or maybe have another table that records
        // the transactions we have made. we can then calculate what the
        // current shares are based on that table. (table could be
        // expensive)
        #[serde(rename = "currentShares")]
        pub current_shares: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TickerStatus {
        #[serde(flatten)]
        pub ticker: Ticker,
        #[serde(flatten)]
        pub goal: TickerGoal,
    }

}

// mod action {
//     use crate::ticker::*;

//     #[derive(Serialize, Deserialize, Debug)]
//     pub enum ActionType {
//         Buy,
//         Hold,
//         Sell,
//     }

//     #[derive(Serialize, Deserialize, Debug)]
//     pub struct Action {
//         symbol: TickerSymbol,
//         action_type: ActionType,
//         date: u32,
//         quantity: u32,
//     }
// }
