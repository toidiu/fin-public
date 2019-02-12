use super::CONNECTION;
use crate::algo;
use crate::backend::{self, PortfolioBackend};
use crate::data;
use crate::errors::{FinError, ResultFin};
use crate::portfolio;
use crate::server;
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

pub fn get_port_a_list(
    user_id: i64,
    res_portfolio_backend: Result<
        impl backend::PortfolioBackend,
        warp::Rejection,
    >,
) -> Result<impl warp::Reply, warp::Rejection> {
    let port_backend = res_portfolio_backend?;
    let port_actual = port_backend
        .get_port_actual_list_by_user_id(&user_id)
        .map_err(|err| {
            error!("{}: {}", line!(), err);
            warp::reject::custom(err)
        })?;
    Ok(warp::reply::json(&port_actual))
}

pub fn get_portfolio_a(
    actual_id: i64,
    user_id: i64,
    res_portfolio_backend: Result<
        impl backend::PortfolioBackend,
        warp::Rejection,
    >,
) -> Result<impl warp::Reply, warp::Rejection> {
    let port_backend = res_portfolio_backend?;

    // actual info
    let actual_tickers = port_backend
        .get_actual_tickers(&user_id, &actual_id)
        .map_err(|err| warp::reject::not_found())?;
    let port_actual = port_backend
        .get_port_actual(&actual_id, &actual_tickers)
        .map_err(|err| {
            error!("{}: {}", line!(), err);
            warp::reject::not_found()
        })?;

    // ticker info
    let keys = actual_tickers.keys().map(|x| x.0).collect();
    let tickers_map: HashMap<TickerId, Ticker> =
        port_backend.get_tickers(&keys);

    // goal info
    let goal_tickers = port_backend.get_tic_goal(&port_actual.fk_port_g_id);
    let port_goal = port_backend
        .get_port_goal(
            &port_actual.fk_port_g_id,
            &goal_tickers,
            &tickers_map,
            &port_actual.stock_percent,
        )
        .map_err(|err| {
            error!("{}: {}", line!(), err);
            warp::reject::custom(err)
        })?;

    let mut port =
        portfolio::PortfolioState::new(&port_actual, &port_goal, &tickers_map);

    // get state
    let resp: server::PortfolioStateResp = port.into();
    Ok(warp::reply::json(&resp))
}

pub fn create_port_a(
    user_id: i64,
    data: server::NewPortActualData,
    res_portfolio_backend: Result<
        impl backend::PortfolioBackend,
        warp::Rejection,
    >,
) -> Result<impl warp::Reply, warp::Rejection> {
    let port_backend = res_portfolio_backend?;
    let resp = port_backend
        .create_port_a(&user_id, &data.goal_id, &data.stock_percent)
        .map_err(|err| {
            error!("{}: {}", line!(), err);
            warp::reject::custom(FinError::ServerErr)
        })?;

    let reply = serde_json::to_string(&resp).map_err(|err| {
        error!("{}: {}", line!(), err);
        warp::reject::custom(err)
    })?;
    Ok(warp::reply::with_status(
        reply,
        warp::http::StatusCode::CREATED,
    ))
}

pub fn get_buy_next(
    user_id: i64,
    data: server::BuyNextQuery,
    res_portfolio_backend: Result<
        impl backend::PortfolioBackend,
        warp::Rejection,
    >,
) -> Result<impl warp::Reply, warp::Rejection> {
    let port_backend = res_portfolio_backend?;

    debug!("amount to buy for: {}", data.amount);
    let resp = port_backend
        .get_buy_next(
            &user_id,
            &data.goal_port_id,
            &data.actual_port_id,
            data.amount,
        )
        .map_err(|err| {
            error!("{}: {}", line!(), err);
            warp::reject::custom(err)
        })?;
    let resp = server::BuyNextResp::from_data(resp, data.amount);
    Ok(warp::reply::json(&resp))
}

pub(super) fn post_buy_next(
    user_id: i64,
    data: server::BuyNextData,
    res_portfolio_backend: Result<
        impl backend::PortfolioBackend,
        warp::Rejection,
    >,
) -> Result<impl warp::Reply, warp::Rejection> {
    let port_backend = res_portfolio_backend?;

    let port = port_backend.execute_actions(
        &user_id,
        &data.goal_id,
        &data.port_a_id,
        &data.actions,
    );

    let resp: server::PortfolioStateResp = port
        .map_err(|err| {
            error!("{}: {}", line!(), err);
            warp::reject::custom(FinError::ServerErr)
        })?
        .into();

    let reply = serde_json::to_string(&resp).map_err(|err| {
        error!("{}: {}", line!(), err);
        warp::reject::custom(err)
    })?;
    Ok(warp::reply::with_status(
        reply,
        warp::http::StatusCode::CREATED,
    ))
}
