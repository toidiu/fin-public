use crate::api;
use rocket::request::Form;
use rocket::response::status;
use rocket::Request;
use rocket::{http::Method, State};
use rocket_contrib::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::ops::Deref;
use std::sync::RwLock;

use crate::data::{self, TickerBackend};
use crate::errors::{FinError, ResultFinErr};
use crate::portfolio::{self, Ticker, TickerId};
use lru_time_cache::LruCache;
use postgres::{Connection, TlsMode};

const CACHE_MAX_COUNT: usize = 100;
const CACHE_TTL: std::time::Duration = ::std::time::Duration::from_secs(10);
const DB_URI: &'static str = "postgres://postgres@localhost:5432/test-fin";

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
        .mount("/", routes![portfolio, get_buy_next, post_buy_next])
        .attach(options)
        .launch();
}

#[get("/portfolio?<query>")]
fn portfolio(query: api::PortfolioStateQuery) -> ResultFinErr<String> {
    let conn = Connection::connect(DB_URI, TlsMode::None)
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
    let actual = db.get_actual(&query.user_id, &query.goal_id)?;
    let goal_tickers = db.get_goal(&query.goal_id);
    let port_goal = db
        .get_port_goal(&query.goal_id)
        .unwrap()
        .to_port_goal(goal_tickers);
    let mut port = portfolio::Portfolio::new(&mut db, &actual, &port_goal);

    // get state
    let port_state = port.get_state();
    Ok(serde_json::to_string(&port_state).unwrap())
}

#[get("/buy?<query>")]
fn get_buy_next<'r>(query: api::BuyNextQuery) -> ResultFinErr<String> {
    let conn = Connection::connect(DB_URI, TlsMode::None)
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
    let actual = db.get_actual(&query.user_id, &query.goal_id)?;

    debug!("amount to buy for: {}", query.amount);
    let resp = portfolio::Portfolio::get_buy_next(
        &mut db,
        &actual,
        query.amount,
        &query.goal_id,
    );

    Ok(serde_json::to_string(&resp).unwrap())
}

#[post("/buy", data = "<form>")]
fn post_buy_next(
    form: Json<api::BuyNextForm>,
) -> ResultFinErr<status::Created<String>> {
    let conn = Connection::connect(DB_URI, TlsMode::None)
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
    let actual = db.get_actual(&form.user_id, &form.goal_id)?;

    let resp = portfolio::Portfolio::exec_buy_next(
        &mut db,
        &actual,
        form.amount,
        &form.goal_id,
    );

    Ok(status::Created("".to_owned(), None))
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}
