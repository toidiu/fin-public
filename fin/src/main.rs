#![feature(plugin, custom_derive)]
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

use rocket::Request;
use rocket::{http::Method, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::ops::Deref;
use std::sync::RwLock;

use crate::{data::*, ticker::TickerSymbol};

#[macro_use]
mod std_ext;
mod api;
mod data;
mod portfolio;
mod ticker;

fn main() {
    let db = data::DefaultTickerDatabase {};
    let actual = db.get_actual();
    let mut port = portfolio::Portfolio::new(&db, &actual);
    let evolved = api::EvolvedState::new(port);

    start_server(evolved);
}

fn start_server(evolved: api::EvolvedState) {
    let (allowed_origins, failed_origins) =
        AllowedOrigins::some(&["http://localhost:1234"]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: false,
        ..Default::default()
    };

    rocket::ignite()
        .mount("/", routes![portfolio, buy])
        .catch(catchers![internal_error, not_found])
        .attach(options)
        .manage(RwLock::new(evolved))
        .launch();
}

type AppState<'a> = State<'a, RwLock<api::EvolvedState>>;

#[get("/portfolio")]
fn portfolio<'r>(state: AppState<'r>) -> String {
    let port_state = state.read().unwrap().init_state.get_state();
    serde_json::to_string(&port_state).unwrap()
}

#[derive(FromForm)]
struct AmountQuery {
    amount: f32,
}

#[get("/buy?<q_amount>")]
fn buy<'r>(
    state: AppState<'r>,
    q_amount: AmountQuery,
) -> String {
    println!("{}", q_amount.amount);
    let mut s = state.write().unwrap();

    // todo do based on buy_value and the desired value
    while (s.buy_value < q_amount.amount) {
        if let None = next_state(&mut s, q_amount.amount) {
            break;
        }
    }

    serde_json::to_string(&(*s)).unwrap()
}

fn next_state(
    s: &mut api::EvolvedState,
    buy_amount: f32,
) -> Option<portfolio::Action> {
    let db = data::DefaultTickerDatabase {};

    // get port from action actual
    let port = portfolio::Portfolio::new(&db, &s.evolved_actual);
    // get action
    let action = port.get_buy_next();

    // buying more would put us above the buy value
    if (s.buy_value + action.get_price() > buy_amount) {
        return None;
    }

    // get evolved state
    let evolved_port = port.evolve(&action);

    // update buy_value
    s.buy_value += action.get_price();
    // update action
    s.actions.push(action.clone());

    // update final state
    s.evolved_actual = evolved_port.get_actual_tickers();

    Some(action)
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}
