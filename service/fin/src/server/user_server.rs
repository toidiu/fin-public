use crate::api;
use crate::data::{self, TickerBackend, UserBackend};
use crate::errors::{FinError, ResultFin};
use crate::portfolio::{self, Ticker, TickerId};
use rocket::http;
use rocket::request::Form;
use rocket::response::{status, Response};
use rocket::Request;
use rocket::{http::Method, State};
use rocket_contrib::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::io::Cursor;
use std::ops::Deref;
use std::sync::RwLock;

use lru_time_cache::LruCache;
use postgres::{Connection, TlsMode};

use super::{CACHE_MAX_COUNT, CACHE_TTL, DB_URI};

#[post("/login", data = "<form>")]
fn post_login<'r>(form: Json<api::LoginForm>) -> Response<'r> {
    // ) -> impl Responder + 'static {
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
    match db.get_login_user(&form.email, &form.password) {
        Ok(user_data) => {
            let cookie = http::Cookie::build("sess", user_data.id.to_string())
                .path("/")
                .http_only(true)
                .finish();
            let body = serde_json::to_string(&user_data).unwrap();
            Response::build()
                .status(http::Status::Accepted)
                .header(&cookie)
                .sized_body(Cursor::new(body))
                .finalize()
        }
        Err(err) => {
            Response::build()
                .status(http::Status::Unauthorized)
                .finalize()
        }
    }
}
