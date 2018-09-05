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
    let ticker1 = Ticker {
        symbol: TickerSymbol("vti".to_owned()),
        fee: 0.3,
        price: 150.0,
        investment_kind: InvestmentKind::Stock,
        description: "des".to_owned(),
        // current_goal: 20,
        // current_percent: 55
    };
    let goal1 = TickerGoal {
        symbol: TickerSymbol("vti".to_owned()),
        goal_percent: 2.3,
        current_percent: 3.4,
        current_value: 22.0,
    };
    let ticker_goal1 = TickerStatus {
        ticker: ticker1,
        goal: goal1,
    };

    let ticker2 = Ticker {
        symbol: TickerSymbol("voe".to_owned()),
        fee: 0.5,
        price: 150.0,
        investment_kind: InvestmentKind::Bond,
        description: "dis".to_owned(),
        // current_goal: 9,
        // current_percent: 5
    };
    let goal2 = TickerGoal {
        symbol: TickerSymbol("voe".to_owned()),
        goal_percent: 2.3,
        current_percent: 3.4,
        current_value: 22.0,
    };
    let ticker_goal2 = TickerStatus {
        ticker: ticker2,
        goal: goal2,
    };

    let ticker_list = vec![ticker_goal1, ticker_goal2];
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
    use serde::ser::{Serialize, Serializer, SerializeStruct};
    use serde_json::Value;

    #[derive(Serialize, Deserialize, Debug, Default)]
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
        #[serde(rename = "stock")]
        Stock,
        #[serde(rename = "bond")]
        Bond,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TickerGoal {
        #[serde(skip)]
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
        #[serde(flatten)]
        pub ticker: Ticker,
        #[serde(flatten)]
        pub goal: TickerGoal,
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
