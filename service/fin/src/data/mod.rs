#![allow(dead_code, unused)]

mod db_types;
mod pg_data;

use crate::errors::{FinError, ResultFinErr};
use crate::portfolio::{self, Ticker, TickerId};
use std::collections::HashMap;

pub use self::pg_data::PgTickerDatabase;

pub trait TickerBackend {
    fn get_tickers(&mut self, ids: &Vec<i64>) -> HashMap<TickerId, Ticker>;

    fn get_goal(
        &self,
        port_g_id: &i64,
    ) -> HashMap<TickerId, portfolio::TickerGoal>;

    fn get_actual(
        &self,
        user_id: &i64,
        port_g_id: &i64,
    ) -> HashMap<TickerId, portfolio::TickerActual>;

    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFinErr<db_types::PortGoalData>;
}

trait TickerDb {
    //========== (login) -> user
    fn get_user(
        &self,
        username: &String,
        pass: &String,
    ) -> ResultFinErr<db_types::UserData>;

    //========== -> Pa -> [Ta]
    fn get_ticker_actual(
        &self,
        user_id: &i64,
        port_g_id: &i64,
    ) -> ResultFinErr<Vec<db_types::TickerActualData>>;

    //========== -> Ta -> [oldTa]
    fn update_tickers_actual(
        &self,
        init_tickers_actual: Vec<portfolio::TickerActual>,
        updated_tickers_actual: Vec<portfolio::TickerActual>,
    ) -> ResultFinErr<Vec<db_types::TickerActualData>>;

    //========== -> Pg -> [Tg]
    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFinErr<db_types::PortGoalData>;

    fn get_ticker_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFinErr<Vec<db_types::TickerGoalData>>;

    //========== -> [T]
    fn get_tickers_by_ids(
        &self,
        ids: &Vec<i64>,
    ) -> ResultFinErr<Vec<db_types::TickerData>>;

    //========== (buy) -> Actual -> Goal -> T
}
