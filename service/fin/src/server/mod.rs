use crate::api;
use std::collections::BTreeSet;
use std::env;
use std::fmt::{self, Display};
use std::ops::Deref;
use std::sync::RwLock;

use crate::data::{self, TickerBackend, UserBackend};
use crate::errors::{ErrMessage, FinError, ResultFin};
use crate::portfolio::{self, Ticker, TickerId};
use postgres::Connection;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use http::header::HeaderValue;
use http::{Request, Response, StatusCode};

use warp::{http::Uri, Filter, Rejection, Reply};

mod auth;
mod portfolio_server;
mod user_server;

const DB_URI: &'static str = "postgres://postgres@localhost:5432/test-fin";

lazy_static! {
    static ref CONNECTION: r2d2::Pool<PostgresConnectionManager> = {
        let manager =
            PostgresConnectionManager::new(DB_URI, TlsMode::None).unwrap();
        r2d2::Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to create pool")
    };
}

pub fn start_server() {
    let port = 8000;
    println!("Listening on http://localhost:{}", port);

    // HEADERS
    let with_cors = warp::cors()
        .allow_origin("http://localhost:1234")
        .allow_credentials(true)
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]);
    let with_credentials =
        warp::reply::with::header("access-control-allow-credentials", "true");

    // AUTH
    let sess_cookie_name = "sess";
    let with_auth = warp::cookie::optional(&sess_cookie_name).and_then(
        |opt_sess: Option<String>| match opt_sess {
            Some(sess) => auth::parse_sess(&sess)
                .map_err(|err| warp::reject::custom(FinError::NotLoggedIn)),
            None => Err(warp::reject::custom(FinError::NotLoggedIn)),
        },
    );
    let with_opt_auth = warp::cookie::optional(&sess_cookie_name);

    // PORTFOLIO
    let portfolio_path = warp::path("portfolio");
    let get_port = warp::get2()
        .and(portfolio_path)
        .and(warp::path::param::<i64>())
        .and(with_auth)
        .and_then(portfolio_server::get_portfolio);
    let get_buy_next = warp::get2()
        .and(portfolio_path)
        .and(warp::path("buy"))
        .and(with_auth)
        .and(warp::query())
        .and_then(portfolio_server::get_buy_next);
    let post_buy_next = warp::post2()
        .and(portfolio_path)
        .and(warp::path("buy"))
        .and(with_auth)
        .and(warp::body::json())
        .and_then(portfolio_server::post_buy_next);
    let port_api = get_port.or(get_buy_next).or(post_buy_next);

    // USERS
    let user_path = warp::path("users");
    let post_login = warp::post2()
        .and(user_path)
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(user_server::login);

    let post_logout = warp::post2()
        .and(user_path)
        .and(warp::path("logout"))
        .and_then(user_server::logout);
    let user_api = post_login.or(post_logout);

    // OTHER
    let seth = warp::get2().and(warp::path("seth")).map(auth::f_seth);

    // combine apis
    let api = port_api.or(user_api).or(seth);

    let routes = api.recover(recover_error).with(with_cors);
    warp::serve(routes).run(([127, 0, 0, 1], port));
}

fn recover_error(err: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(&err) = err.find_cause::<FinError>() {
        let status_code = match err {
            FinError::NotLoggedIn => StatusCode::UNAUTHORIZED,
            FinError::BadRequestErr => StatusCode::BAD_REQUEST,
            FinError::DatabaseErr | FinError::ServerErr => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let json = warp::reply::json(&err.to_msg());
        Ok(warp::reply::with_status(json, status_code))
    } else {
        let status_code = StatusCode::NOT_FOUND;
        let json = warp::reply::json(&ErrMessage::new(
            status_code,
            "not found".to_string(),
        ));
        Ok(warp::reply::with_status(json, status_code))
    }
}
