#![allow(dead_code, unused)]

mod db_types;
mod pg_data;

use crate::errors::{FinError, ResultFin};
use crate::portfolio::{self, Ticker, TickerId};
use std::collections::HashMap;

pub use self::pg_data::PgTickerDatabase;

pub trait UserBackend {
    fn get_login_user(
        &self,
        email: &String,
        pass: &String,
    ) -> ResultFin<db_types::UserData>;
}

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
    ) -> ResultFin<HashMap<TickerId, portfolio::TickerActual>>;

    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<db_types::PortGoalData>;

    fn update_actual(
        &self,
        init_tickers_actual: &Vec<&portfolio::TickerActual>,
        updated_tickers_actual: &Vec<&portfolio::TickerActual>,
    ) -> ResultFin<Vec<portfolio::TickerActual>>;
}

trait TickerDb {
    //========== (login) -> user
    fn get_user(
        &self,
        email: &String,
        pass: &String,
    ) -> ResultFin<db_types::UserData>;

    //========== -> Pa -> [Ta]
    fn get_ticker_actual(
        &self,
        user_id: &i64,
        port_g_id: &i64,
    ) -> ResultFin<Vec<db_types::TickerActualData>>;

    //========== -> Ta -> [oldTa]
    fn update_tickers_actual(
        &self,
        init_tickers_actual: &Vec<&portfolio::TickerActual>,
        updated_tickers_actual: &Vec<&portfolio::TickerActual>,
    ) -> ResultFin<Vec<db_types::TickerActualData>>;

    //========== -> Pg -> [Tg]
    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<db_types::PortGoalData>;

    fn get_ticker_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<Vec<db_types::TickerGoalData>>;

    //========== -> [T]
    fn get_tickers_by_ids(
        &self,
        ids: &Vec<i64>,
    ) -> ResultFin<Vec<db_types::TickerData>>;

    //========== (buy) -> Actual -> Goal -> T
}
