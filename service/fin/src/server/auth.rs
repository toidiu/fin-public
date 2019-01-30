pub fn parse_sess(sess: &str) -> Result<i64, ()> {
    sess.parse::<i64>().map_err(|err| ())
}

// TEMP==============
use http::{Request, Response, StatusCode};
pub fn f_seth() -> impl warp::Reply {
    let response = Response::builder()
        .status(200)
        .header(http::header::SET_COOKIE, "sess=1;HttpOnly")
        .body("set header".to_string())
        .unwrap();
    response
}

fn f_geth() -> impl warp::Reply {
    let response = Response::builder()
        .status(200)
        .header(http::header::SET_COOKIE, "sess=1;HttpOnly")
        .body("set header".to_string())
        .unwrap();
    response
}
