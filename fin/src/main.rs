#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(nll)]

#[macro_use]
extern crate serde_derive;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};


#[get("/")]
fn index() -> String {
    let ticker = models::Ticker {
        name: "vti".to_owned(),
        fee: 0.3,
        current_goal: 20,
        current_percent: 55
    };
    let ticker1 = models::Ticker {
        name: "voe".to_owned(),
        fee: 0.5,
        current_goal: 9,
        current_percent: 5
    };

    let ticker_list = vec![ticker, ticker1];
    serde_json::to_string(&ticker_list).unwrap()

    // "[{ \"name\": \"vti\", \"fee\": \".03\", \"current_goal\": 33, \"current_percent\": 33 }]"
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

mod models {

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Ticker {
        pub name: String,
        pub fee: f32,
        #[serde(rename = "currentGoal")]
        pub current_goal: u32,
        #[serde(rename = "currentPercent")]
        pub current_percent: u32,
    }

}
