use crate::{api, data, portfolio};
use rocket::Request;
use rocket::{http::Method, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::ops::Deref;
use std::sync::RwLock;

use crate::{data::*, ticker::TickerId};
use lru_time_cache::LruCache;
use postgres::{Connection, TlsMode};

const CACHE_MAX_COUNT: usize = 100;
const CACHE_TTL: std::time::Duration = ::std::time::Duration::from_secs(10);

pub fn start_server() {
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
        .catch(catchers![internal_error, not_found])
        .mount("/", routes![portfolio, buy])
        .attach(options)
        .launch();
}

#[get("/portfolio")]
fn portfolio() -> String {
    let database_url = "postgres://postgres@localhost:5432/test-fin";
    let conn = Connection::connect(database_url, TlsMode::None)
        .expect("cannot connect to postgres");

    let lru_cache = LruCache::<String, f32>::with_expiry_duration_and_capacity(
        CACHE_TTL,
        CACHE_MAX_COUNT,
    );
    let mut db = data::PgTickerDatabase {
        conn: conn,
        lru: lru_cache,
    };

    // get port
    let actual = db.get_actual();
    let mut port = portfolio::Portfolio::new(&mut db, &actual);

    // get state
    let port_state = port.get_state();
    serde_json::to_string(&port_state).unwrap()
}

#[get("/buy?<q_amount>")]
fn buy<'r>(q_amount: api::AmountQuery) -> String {
    let database_url = "postgres://postgres@localhost:5432/test-fin";
    let conn = Connection::connect(database_url, TlsMode::None)
        .expect("cannot connect to postgres");

    let lru_cache = LruCache::<String, f32>::with_expiry_duration_and_capacity(
        CACHE_TTL,
        CACHE_MAX_COUNT,
    );
    let mut db = data::PgTickerDatabase {
        conn: conn,
        lru: lru_cache,
    };

    // get port
    let actual = db.get_actual();
    let mut port = portfolio::Portfolio::new(&mut db, &actual);

    debug!("{}", q_amount.amount);

    let mut e_state = api::EvolvedState::new(port);

    // todo do based on buy_value and the desired value
    while (e_state.buy_value < q_amount.amount) {
        if let None = next_state(&mut e_state, q_amount.amount, &mut db) {
            break;
        }
    }

    serde_json::to_string(&(e_state)).unwrap()
}

fn next_state(
    s: &mut api::EvolvedState,
    buy_amount: f32,
    db: &mut data::PgTickerDatabase,
) -> Option<portfolio::Action> {
    // get port from action actual
    let port = portfolio::Portfolio::new(db, &s.evolved_actual);
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
