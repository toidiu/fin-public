#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(nll)]

#[macro_use]
extern crate serde_derive;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[get("/")]
fn index() -> String {
    use crate::data::*;

    let d = get_data();
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

mod data {
    use crate::portfolio::*;
    use crate::ticker::*;

    pub fn get_data() -> Portfolio {
        let tg_vti = TickerStatus {
            ticker: Ticker {
                symbol: TickerSymbol("vti".to_owned()),
                fee: 0.04,
                price: 150.0,
                kind: InvestmentKind::Stock,
                description: "des".to_owned(),
            },
            goal: TickerGoal {
                symbol: TickerSymbol("vti".to_owned()),
                goal_percent: 20.0,
                current_percent: 22.56,
                current_value: 300.0,
                current_quantity: 1,
            },
        };

        let tg_vtv = TickerStatus {
            ticker: Ticker {
                symbol: TickerSymbol("vtv".to_owned()),
                fee: 0.05,
                price: 111.0,
                kind: InvestmentKind::Stock,
                description: "des".to_owned(),
            },
            goal: TickerGoal {
                symbol: TickerSymbol("vtv".to_owned()),
                goal_percent: 6.0,
                current_percent: 8.35,
                current_value: 111.0,
                current_quantity: 1,
            },
        };

        let tg_voe = TickerStatus {
            ticker: Ticker {
                symbol: TickerSymbol("voe".to_owned()),
                fee: 0.07,
                price: 115.0,
                kind: InvestmentKind::Stock,
                description: "des".to_owned(),
            },
            goal: TickerGoal {
                symbol: TickerSymbol("voe".to_owned()),
                goal_percent: 4.0,
                current_percent: 8.65,
                current_value: 115.0,
                current_quantity: 1,
            },
        };

        let tg_vbr = TickerStatus {
            ticker: Ticker {
                symbol: TickerSymbol("vbr".to_owned()),
                fee: 0.07,
                price: 142.0,
                kind: InvestmentKind::Stock,
                description: "des".to_owned(),
            },
            goal: TickerGoal {
                symbol: TickerSymbol("vbr".to_owned()),
                goal_percent: 3.0,
                current_percent: 10.68,
                current_value: 142.0,
                current_quantity: 1,
            },
        };

        let tg_vea = TickerStatus {
            ticker: Ticker {
                symbol: TickerSymbol("vea".to_owned()),
                fee: 0.07,
                price: 43.0,
                kind: InvestmentKind::Stock,
                description: "des".to_owned(),
            },
            goal: TickerGoal {
                symbol: TickerSymbol("vea".to_owned()),
                goal_percent: 15.0,
                current_percent: 9.70,
                current_value: 129.0,
                current_quantity: 3,
            },
        };

        let tg_vwo = TickerStatus {
            ticker: Ticker {
                symbol: TickerSymbol("vwo".to_owned()),
                fee: 0.14,
                price: 43.0,
                kind: InvestmentKind::Stock,
                description: "des".to_owned(),
            },
            goal: TickerGoal {
                symbol: TickerSymbol("vwo".to_owned()),
                goal_percent: 10.0,
                current_percent: 6.47,
                current_value: 86.0,
                current_quantity: 2,
            },
        };

        let tg_vtip = TickerStatus {
            ticker: Ticker {
                symbol: TickerSymbol("vtip".to_owned()),
                fee: 0.06,
                price: 49.0,
                kind: InvestmentKind::Bond,
                description: "des".to_owned(),
            },
            goal: TickerGoal {
                symbol: TickerSymbol("vtip".to_owned()),
                goal_percent: 3.0,
                current_percent: 3.68,
                current_value: 49.0,
                current_quantity: 1,
            },
        };

        let tg_agg = TickerStatus {
            ticker: Ticker {
                symbol: TickerSymbol("agg".to_owned()),
                fee: 0.05,
                price: 106.0,
                kind: InvestmentKind::Bond,
                description: "des".to_owned(),
            },
            goal: TickerGoal {
                symbol: TickerSymbol("agg".to_owned()),
                goal_percent: 4.0,
                current_percent: 7.97,
                current_value: 106.0,
                current_quantity: 1,
            },
        };

        let tg_mub = TickerStatus {
            ticker: Ticker {
                symbol: TickerSymbol("mub".to_owned()),
                fee: 0.07,
                price: 109.0,
                kind: InvestmentKind::Bond,
                description: "des".to_owned(),
            },
            goal: TickerGoal {
                symbol: TickerSymbol("mub".to_owned()),
                goal_percent: 14.0,
                current_percent: 8.2,
                current_value: 109.0,
                current_quantity: 1,
            },
        };

        let tg_bndx = TickerStatus {
            ticker: Ticker {
                symbol: TickerSymbol("bndx".to_owned()),
                fee: 0.11,
                price: 54.0,
                kind: InvestmentKind::Bond,
                description: "des".to_owned(),
            },
            goal: TickerGoal {
                symbol: TickerSymbol("bndx".to_owned()),
                goal_percent: 12.0,
                current_percent: 8.12,
                current_value: 108.0,
                current_quantity: 2,
            },
        };

        let tg_vwob = TickerStatus {
            ticker: Ticker {
                symbol: TickerSymbol("vwob".to_owned()),
                fee: 0.32,
                price: 75.0,
                kind: InvestmentKind::Bond,
                description: "des".to_owned(),
            },
            goal: TickerGoal {
                symbol: TickerSymbol("vwob".to_owned()),
                goal_percent: 9.0,
                current_percent: 5.64,
                current_value: 75.0,
                current_quantity: 1,
            },
        };

        let port = Portfolio {
            name: "my portfolio".to_owned(),
            started: 123,
            current_detail: PortfolioDetails {
                stocks: vec![tg_vti, tg_vtv, tg_voe, tg_vbr, tg_vea, tg_vwo],
                bonds: vec![tg_vtip, tg_agg, tg_mub, tg_bndx, tg_vwob],
                goal_stock_percent: 58.0,
                current_stock_percent: 66.39,
                deviation_percent: 5.0,
            },
            past_detail: vec![],
        };

        port

        // vec![
        //     tg_vti, tg_vtv, tg_voe, tg_vbr, tg_vea, tg_vwo, tg_vtip, tg_agg, tg_mub, tg_bndx,
        //     tg_vwob,
        // ]
    }

}

mod ticker {

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct TickerSymbol(pub String);

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Ticker {
        pub symbol: TickerSymbol,
        pub fee: f32,
        pub price: f32,
        pub kind: InvestmentKind,
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
        #[serde(rename = "currentQuantity")]
        pub current_quantity: u32,
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
        pub stocks: Vec<TickerStatus>,
        pub bonds: Vec<TickerStatus>,
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
