use crate::backend;
use crate::data;
use crate::errors::{ErrMessage, FinError};
use crate::global::{CONFIG, ROOT};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use http::StatusCode;

use warp::{Filter, Rejection};

mod api;
mod auth;
mod portfolio_server;
mod user_server;

pub use api::*;

lazy_static! {
    static ref CONNECTION: r2d2::Pool<PostgresConnectionManager> = {
        let manager = PostgresConnectionManager::new(
            CONFIG.database.url.to_string(),
            TlsMode::None,
        )
        .unwrap();
        r2d2::Pool::builder()
            .max_size(CONFIG.database.pool_size)
            .build(manager)
            .expect("Failed to create pool")
    };
    static ref LOGGER: slog::Logger =
        (*ROOT).clone().new(o!("mod" => "server"));
}

pub fn start_server() {
    println!("Listening on http://localhost:{}", CONFIG.app.port);
    info!(LOGGER, "Listening on http://localhost:{}", CONFIG.app.port);

    // HEADERS
    let with_cors = warp::cors()
        .allow_origin(CONFIG.app.cors_origin.as_str())
        .allow_credentials(true)
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]);

    let with_user_backend = {
        warp::any().map(|| match CONNECTION.get() {
            Ok(conn) => {
                Ok(backend::DefaultUserBackend::new(data::PgFinDb::new(
                    conn,
                    backend::UserBackend::get_logger_context((*LOGGER).clone()),
                )))
            }
            Err(err) => {
                error!(LOGGER, "{}: {}", line!(), err);
                Err(warp::reject::custom(FinError::DatabaseErr))
            }
        })
    };

    let with_portfolio_backend = {
        warp::any().map(|| match CONNECTION.get() {
            Ok(conn) => Ok(backend::DefaultPortfolioBackend::new(
                data::PgFinDb::new(
                    conn,
                    backend::PortfolioBackend::get_logger_context(
                        (*LOGGER).clone(),
                    ),
                ),
                (*LOGGER).clone(),
            )),
            Err(err) => {
                error!(LOGGER, "{}: {}", line!(), err);
                Err(warp::reject::custom(FinError::DatabaseErr))
            }
        })
    };

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

    // PORTFOLIO===============
    let portfolio_path = warp::path("portfolio");
    // GET -> portfolio/goal
    let get_port_g_list = warp::get2()
        .and(portfolio_path)
        .and(warp::path("goal"))
        .and(warp::path::end())
        .and(with_portfolio_backend)
        .and_then(portfolio_server::get_portfolio_g_list);
    // GET -> portfolio/actual/:id
    let get_port_a_by_id = warp::get2()
        .and(portfolio_path)
        .and(warp::path("actual"))
        .and(warp::path::param2::<i64>())
        .and(warp::path::end())
        .and(with_auth)
        .and(with_portfolio_backend)
        .and_then(portfolio_server::get_portfolio_a);
    // GET -> portfolio/actual
    let get_port_a_list = warp::get2()
        .and(portfolio_path)
        .and(warp::path("actual"))
        .and(warp::path::end())
        .and(with_auth)
        .and(with_portfolio_backend)
        .and_then(portfolio_server::get_port_a_list);
    // POST -> portfolio/actual
    let create_port_a = warp::post2()
        .and(portfolio_path)
        .and(warp::path("actual"))
        .and(warp::path::end())
        .and(with_auth)
        .and(warp::body::json())
        .and(with_portfolio_backend)
        .and_then(portfolio_server::create_port_a);
    // GET -> portfolio/actual/buy/?goal_id=1&amount=1
    let get_buy_next = warp::get2()
        .and(portfolio_path)
        .and(warp::path("actual"))
        .and(warp::path("buy"))
        .and(with_auth)
        .and(warp::query())
        .and(with_portfolio_backend)
        .and_then(portfolio_server::get_buy_next);
    // POST -> portfolio/actual/buy
    let post_buy_next = warp::post2()
        .and(portfolio_path)
        .and(warp::path("actual"))
        .and(warp::path("buy"))
        .and(warp::path::end())
        .and(with_auth)
        .and(warp::body::json())
        .and(with_portfolio_backend)
        .and_then(portfolio_server::post_buy_next);
    let port_api = get_port_g_list
        .or(get_port_a_by_id)
        .or(get_port_a_list)
        .or(create_port_a)
        .or(get_buy_next)
        .or(post_buy_next);

    // USERS===============
    let user_path = warp::path("users");
    let post_login = warp::post2()
        .and(user_path)
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(with_user_backend)
        .and_then(user_server::login);
    let post_logout = warp::post2()
        .and(user_path)
        .and(warp::path("logout"))
        .and_then(user_server::logout);
    let post_signup = warp::post2()
        .and(user_path)
        .and(warp::path("signup"))
        .and(warp::body::json())
        .and(with_user_backend)
        .and_then(user_server::signup);

    let user_api = post_login.or(post_logout).or(post_signup);

    // combine apis
    let api = port_api.or(user_api);

    let routes = api.recover(recover_error).with(with_cors);
    warp::serve(routes).run(([127, 0, 0, 1], CONFIG.app.port));
}

fn recover_error(err: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(&err) = err.find_cause::<FinError>() {
        let status_code = match err {
            FinError::NotLoggedIn => StatusCode::UNAUTHORIZED,
            FinError::BadRequestErr => StatusCode::BAD_REQUEST,
            FinError::NotFoundErr => StatusCode::NOT_FOUND,
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
