use crate::data;
use crate::settings::CONFIG;
use http::{self, Response, StatusCode};

use crate::errors::{self, FinError, ResultFin};
use chrono::prelude::*;

lazy_static! {
    static ref SECRET_KEY: Vec<u8> =
        Vec::from(CONFIG.app.paseto_token.as_bytes());
}

#[derive(Deserialize, Debug)]
struct Sess {
    user_id: i64,
}

pub fn parse_sess(sess: &str) -> ResultFin<i64> {
    let res_verified_token = paseto::tokens::validate_local_token(
        sess.to_string(),
        None,
        // TODO use session key to prevent replay attack.. timestamp helps
        // Some(String::from("key-id:gandalf0")),
        SECRET_KEY.to_vec(),
    )
    .map_err(|err| {
        error!("{}: {}", line!(), err);
        FinError::BadRequestErr
    });

    res_verified_token.and_then(|verified_token| {
        serde_json::from_value(verified_token)
            .map(|sess: Sess| sess.user_id)
            .map_err(|err| {
                error!("{}: {}", line!(), err);
                FinError::BadRequestErr
            })
    })
}

pub fn resp_with_auth(
    user_data: data::UserData,
    body: String,
    status: StatusCode,
) -> Result<Response<String>, warp::Rejection> {
    let curr = Utc::now();
    let expire = curr + chrono::Duration::minutes(15);

    let token = paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(SECRET_KEY.to_vec())
        .set_issued_at(None)
        .set_expiration(expire)
        .set_issuer(String::from("fin"))
        .set_audience(String::from("fin api"))
        .set_jti(String::from("session token"))
        .set_not_before(Utc::now())
        .set_subject(String::from("session"))
        .set_claim(String::from("user_id"), json!(user_data.id))
        // TODO use session key to prevent replay attack.. timestamp helps
        // .set_footer(String::from("key-id:gandalf1"))
        .build()
        .expect("Failed to construct paseto token w/ builder!");

    Ok(Response::builder()
        .status(status)
        .header(
            http::header::SET_COOKIE,
            format!("sess={};HttpOnly;path=/", token),
        )
        .body(body)
        .expect(&format!("{} error adding auth to resp", line!())))
}
