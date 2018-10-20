#![allow(dead_code, unused)]

mod pg_data;

pub use self::pg_data::PgTickerDatabase;

use crate::errors::{FinError, ResultFinErr};
use crate::models;
use crate::{portfolio, ticker::*};
use std::collections::HashMap;

pub trait TickerDatabase {
    fn get_tickers(&mut self, ids: &Vec<i64>) -> HashMap<TickerId, Ticker>;
    fn get_goal(&self) -> HashMap<TickerId, portfolio::TickerGoal>;
    fn get_actual(&self) -> HashMap<TickerId, portfolio::TickerActual>;
}

pub trait NewDatabase {
    //========== (login) -> user
    fn get_user(
        &self,
        username: &String,
        pass: &String,
    ) -> ResultFinErr<models::UserData>;

    //========== -> Pa -> [Ta]
    fn get_ticker_actual(
        &self,
        user_id: &i64,
        port_g_id: &i64,
    ) -> ResultFinErr<Vec<models::TickerActualData>>;

    //========== -> Pg -> [Tg]
    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFinErr<models::PortGoalData>;

    fn get_ticker_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFinErr<Vec<models::TickerGoalData>>;

    //========== -> [T]
    fn get_tickers_data(
        &self,
        ids: &Vec<i64>,
    ) -> ResultFinErr<Vec<models::TickerData>>;

    //========== (buy) -> Actual -> Goal -> T
}
