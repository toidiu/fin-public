use super::CONNECTION;
use crate::api;
use crate::buy_next;
use crate::data::{self, TickerBackend, UserBackend};
use crate::errors::{FinError, ResultFin};
use crate::portfolio;
use crate::ticker::{Ticker, TickerId};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::RwLock;

use http::{self, Request, Response, StatusCode};

use super::auth;

pub fn get_portfolio(
    goal_id: i64,
    user_id: i64,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = CONNECTION.get().map_err(|err| {
        error!("{}", err);
        warp::reject::custom(FinError::DatabaseErr)
    })?;
    let mut db = data::PgTickerDatabase { conn: conn };

    // get port
    let actual = db
        .get_actual(&user_id, &goal_id)
        .map_err(|err| warp::reject::not_found())?;
    let goal_tickers = db.get_goal(&goal_id);
    let port_goal = db
        .get_port_goal(&goal_id)
        .unwrap()
        .to_port_goal(goal_tickers);

    let keys = actual.keys().map(|x| x.0).collect();
    let tickers_map: HashMap<TickerId, Ticker> = db.get_tickers(&keys);
    let mut port =
        portfolio::Portfolio::new(&mut db, &actual, &tickers_map, &port_goal);

    // get state
    let port_state = port.get_state();
    Ok(warp::reply::json(&port_state))
}

pub fn get_buy_next(
    user_id: i64,
    data: api::BuyNextQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = CONNECTION.get().map_err(|err| {
        error!("{}", err);
        warp::reject::custom(FinError::DatabaseErr)
    })?;

    let mut db = data::PgTickerDatabase { conn: conn };

    // get port
    let actual = db.get_actual(&user_id, &data.goal_id).map_err(|err| {
        error!("{}", err);
        warp::reject::custom(err)
    })?;

    debug!("amount to buy for: {}", data.amount);
    let resp = buy_next::BuyNext::get_buy_next(
        &mut db,
        &actual,
        data.amount,
        &data.goal_id,
    );
    let resp = api::BuyNextResp::from_data(resp, data.amount);
    Ok(warp::reply::json(&resp))
}

pub(super) fn post_buy_next(
    user_id: i64,
    data: api::BuyNextData,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = CONNECTION.get().map_err(|err| {
        error!("{}", err);
        warp::reject::custom(FinError::DatabaseErr)
    })?;
    let mut db = data::PgTickerDatabase { conn: conn };

    let port = portfolio::Portfolio::execute_action(
        &mut db,
        &user_id,
        &data.goal_id,
        &data.actions,
    );
    let port = port
        .map_err(|err| warp::reject::custom(FinError::ServerErr))?
        .get_state();

    let reply = serde_json::to_string(&port).unwrap();
    Ok(warp::reply::with_status(
        reply,
        warp::http::StatusCode::CREATED,
    ))
}
