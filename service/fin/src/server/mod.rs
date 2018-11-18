use crate::api;
use rocket::request::Form;
use rocket::response::status;
use rocket::Request;
use rocket::{http::Method, State};
use rocket_contrib::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::ops::Deref;
use std::sync::RwLock;

use crate::data::{self, TickerBackend, UserBackend};
use crate::errors::{FinError, ResultFin};
use crate::portfolio::{self, Ticker, TickerId};
use lru_time_cache::LruCache;
use postgres::{Connection, TlsMode};

mod portfolio_server;
mod user_server;

const CACHE_MAX_COUNT: usize = 100;
const CACHE_TTL: std::time::Duration = ::std::time::Duration::from_secs(10);
const DB_URI: &'static str = "postgres://postgres@localhost:5432/test-fin";

pub fn start_server() {
    let (allowed_origins, failed_origins) =
        // AllowedOrigins::all();
        AllowedOrigins::some(&["http://localhost:1234"]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        // allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            // .into_iter()
            // .map(From::from)
            // .collect(),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .catch(catchers![internal_error, not_found])
        .mount(
            "/portfolio",
            routes![
                portfolio_server::portfolio,
                portfolio_server::get_buy_next,
                portfolio_server::post_buy_next
            ],
        )
        .mount("/users", routes![user_server::post_login])
        .attach(options)
        .launch();
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}
