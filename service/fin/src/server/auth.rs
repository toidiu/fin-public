use crate::data;
use http::{self, Response, StatusCode};

pub fn parse_sess(sess: &str) -> Result<i64, ()> {
    sess.parse::<i64>().map_err(|err| ())
}

pub fn resp_with_auth(
    user_data: data::UserData,
    body: String,
    status: StatusCode,
) -> Result<Response<String>, warp::Rejection> {
    Ok(Response::builder()
        .status(status)
        .header(
            http::header::SET_COOKIE,
            format!("sess={};HttpOnly;path=/", user_data.id),
        )
        .body(body)
        .expect(&format!("{} error adding auth to resp", line!())))
}
