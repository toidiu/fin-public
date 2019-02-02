use super::CONNECTION;
use crate::api;
use crate::backend::{self, PortfolioBackend};
use crate::buy_next;
use crate::data;
use crate::errors::{FinError, ResultFin};
use crate::portfolio;
use crate::ticker::{Ticker, TickerId};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::RwLock;

use http::{self, Request, Response, StatusCode};

use super::auth;

pub fn get_portfolio_g_list(
    res_portfolio_backend: Result<
        impl backend::PortfolioBackend,
        warp::Rejection,
    >,
) -> Result<impl warp::Reply, warp::Rejection> {
    let port_backend = res_portfolio_backend?;
    let port_goal = port_backend.get_port_goals().map_err(|err| {
        error!("{}: {}", line!(), err);
        warp::reject::custom(err)
    })?;
    Ok(warp::reply::json(&port_goal))
}

pub fn get_portfolio_a(
    goal_id: i64,
    user_id: i64,
    res_portfolio_backend: Result<
        impl backend::PortfolioBackend,
        warp::Rejection,
    >,
) -> Result<impl warp::Reply, warp::Rejection> {
    let port_backend = res_portfolio_backend?;

    // get port
    let actual = port_backend
        .get_actual(&user_id, &goal_id)
        .map_err(|err| warp::reject::not_found())?;
    if (actual.is_empty()) {
        return Err(warp::reject::not_found());
    }

    let goal_tickers = port_backend.get_tic_goal(&goal_id);
    let port_goal = port_backend
        .get_port_goal(&goal_id)
        .map_err(|err| {
            error!("{}: {}", line!(), err);
            warp::reject::custom(err)
        })?
        .to_port_goal(goal_tickers);

    let keys = actual.keys().map(|x| x.0).collect();
    let tickers_map: HashMap<TickerId, Ticker> =
        port_backend.get_tickers(&keys);
    let mut port = portfolio::Portfolio::new(
        &port_backend,
        &actual,
        &tickers_map,
        &port_goal,
    );

    // get state
    let port_state = port.get_state();
    Ok(warp::reply::json(&port_state))
}

pub fn get_buy_next(
    user_id: i64,
    data: api::BuyNextQuery,
    res_portfolio_backend: Result<
        impl backend::PortfolioBackend,
        warp::Rejection,
    >,
) -> Result<impl warp::Reply, warp::Rejection> {
    let port_backend = res_portfolio_backend?;

    // get port
    let actual =
        port_backend
            .get_actual(&user_id, &data.goal_id)
            .map_err(|err| {
                error!("{}: {}", line!(), err);
                warp::reject::custom(err)
            })?;
    if (actual.is_empty()) {
        return Err(warp::reject::not_found());
    }

    debug!("amount to buy for: {}", data.amount);
    let resp = buy_next::BuyNext::get_buy_next(
        &port_backend,
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
    res_portfolio_backend: Result<
        impl backend::PortfolioBackend,
        warp::Rejection,
    >,
) -> Result<impl warp::Reply, warp::Rejection> {
    let port_backend = res_portfolio_backend?;
    // confirming that user has a portfolio
    let actual =
        port_backend
            .get_actual(&user_id, &data.goal_id)
            .map_err(|err| {
                error!("{}", err);
                warp::reject::custom(err)
            })?;
    if (actual.is_empty()) {
        return Err(warp::reject::not_found());
    }

    let port = portfolio::Portfolio::execute_action(
        &port_backend,
        &user_id,
        &data.goal_id,
        &data.actions,
    );
    let port = port
        .map_err(|err| warp::reject::custom(FinError::ServerErr))?
        .get_state();

    let reply = serde_json::to_string(&port).map_err(|err| {
        error!("{}: {}", line!(), err);
        warp::reject::custom(err)
    })?;
    Ok(warp::reply::with_status(
        reply,
        warp::http::StatusCode::CREATED,
    ))
}
