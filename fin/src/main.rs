#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(nll)]

#[macro_use]
extern crate serde_derive;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use crate::ticker::*;


#[get("/")]
fn index() -> String {
    let ticker = Ticker {
        symbol: TickerSymbol("vti".to_owned()),
        fee: 0.3,
        price: 150.0,
        investment_kind: InvestmentKind::Stock,
        description: "".to_owned(),
        // current_goal: 20,
        // current_percent: 55
    };
    let ticker1 = Ticker {
        symbol: TickerSymbol("voe".to_owned()),
        fee: 0.5,
        price: 150.0,
        investment_kind: InvestmentKind::Stock,
        description: "".to_owned(),
        // current_goal: 9,
        // current_percent: 5
    };

    let ticker_list = vec![ticker, ticker1];
    serde_json::to_string(&ticker_list).unwrap()
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

mod ticker {

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TickerSymbol(pub String);

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Ticker {
        pub symbol: TickerSymbol,
        pub fee: f32,
        pub price: f32,
        pub investment_kind: InvestmentKind,
        pub description: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum InvestmentKind {
        Stock,
        Bond,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TickerGoal {
        pub symbol: TickerSymbol,
        #[serde(rename = "goalPercent")]
        pub goal_percent: f32,
        #[serde(rename = "currentPercent")]
        pub current_percent: f32,
        #[serde(rename = "currentValue")]
        pub current_value: f32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TickerStatus {
        ticker: Ticker,
        goal: TickerGoal,
    }

}

mod portfolio {
    use crate::ticker::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Portfolio {
        pub name: String,
        pub started: u32,
        pub current_detail: PortfolioDetails,
        pub past_detail: Vec<PortfolioDetails>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PortfolioDetails {
        pub tickers: Vec<TickerStatus>,
        pub goal_stock_percent: f32,
        pub current_stock_percent: f32,
        pub deviation_percent: f32,
    }

}

mod action {
    use crate::ticker::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ActionType {
        Buy,
        Hold,
        Sell,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Action {
        symbol: TickerSymbol,
        action_type: ActionType,
        date: u32,
        quantity: u32,
    }
}
