use super::{CONNECTION, DB_URI};
use crate::api;
use crate::data::{self, TickerBackend, UserBackend};
use crate::errors::{FinError, ResultFin};
use crate::portfolio::{self, Ticker, TickerId};
use std::io::Cursor;
use std::ops::Deref;
use std::sync::RwLock;

use http::{self, Request, Response, StatusCode};
use lru_time_cache::LruCache;
use postgres::{Connection, TlsMode};

pub fn login(
    data: api::LoginForm,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = CONNECTION.get().map_err(|err| {
        error!("{}", err);
        warp::reject::custom(FinError::DatabaseErr)
    })?;
    let mut db = data::PgTickerDatabase { conn: conn };

    // res_e_p.and_then(|(email, password)| {
    match db.get_login_user(&data.email, &data.password) {
        Ok(user_data) => {
            let response = Response::builder()
                .status(StatusCode::ACCEPTED)
                .header(
                    http::header::SET_COOKIE,
                    format!(
                        "sess={};HttpOnly;path=/",
                        user_data.id.to_string()
                    ),
                )
                .body("logged in".to_string())
                .unwrap();
            Ok(response)
        }
        Err(err) => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("user not found".to_string())
                .unwrap();
            Ok(response)
        }
    }
    // }
    // )
}

// document.cookie = 'foo=; expires=Thu, 01 Jan 1970 00:00:00 UTC;'
pub fn logout() -> Result<impl warp::Reply, warp::Rejection> {
    let response = Response::builder()
        .status(StatusCode::ACCEPTED)
        .header(
            http::header::SET_COOKIE,
            "sess=;HttpOnly;path=/;expires=Thu, 01 Jan 1970 00:00:00 UTC",
        )
        .body("logged in".to_string())
        .unwrap();
    Ok(response)
}

// #[get("/logout")]
// fn get_logout<'r>() -> Response<'r> {
//     let cookie = http::Cookie::build("sess", "")
//         .path("/")
//         .max_age(time::Duration::zero())
//         .expires(time::now() - time::Duration::days(100))
//         .http_only(true)
//         .finish();
//     Response::build()
//         .status(http::Status::Unauthorized)
//         .header(&cookie)
//         .finalize()
// }
