use crate::std_ext::ExtIterator;
use crate::{portfolio, ticker::*};
use postgres::Connection;
use std::collections::HashMap;

use crate::errors::{FinError, ResultFinErr};
use crate::models;

pub struct PgTickerDatabase {
    pub conn: Connection,
}

impl PgTickerDatabase {
    //========== (login) -> user
    pub fn get_user(
        &self,
        username: &String,
        pass: &String,
    ) -> ResultFinErr<models::UserData> {
        let rows = &self
            .conn
            .query(
                "SELECT id, username FROM users
                WHERE username = $1 AND password = $2",
                &[username, pass],
            ).map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        let ret = rows
            .iter()
            .next()
            .map(|row| models::UserData {
                id: row.get(0),
                username: row.get(1),
            }).ok_or_else(|| FinError::DatabaseErr("no record".to_string()));

        debug!("{:#?}", ret);

        ret
    }

    //========== -> Pa -> [Ta]
    pub fn get_ticker_actual(
        &self,
        user_id: &i64,
        port_g_id: &i64,
    ) -> ResultFinErr<Vec<models::TickerActualData>> {
        let rows = &self
            .conn
            .query(
                "SELECT id, fk_user_id, fk_port_g_id, fk_tic_id, actual_shares FROM tic_actual
                WHERE fk_user_id = $1 AND fk_port_g_id = $2",
                &[user_id, port_g_id],
            ).map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        let ret = rows
            .iter()
            .map(|row| models::TickerActualData {
                id: row.get(0),
                fk_user_id: row.get(1),
                fk_port_g_id: row.get(2),
                fk_tic_id: row.get(3),
                actual_shares: row.get(4),
            }).collect::<Vec<models::TickerActualData>>();

        debug!("{:#?}", ret);
        Ok(ret)
    }

    //========== -> Pg -> [Tg]
    pub fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFinErr<models::PortGoalData> {
        let rows = &self
            .conn
            .query(
                "SELECT id, stock_per, deviation, name, description FROM port_goal
                WHERE id = $1",
                &[port_g_id],
            ).map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        let ret = rows
            .iter()
            .next()
            .map(|row| models::PortGoalData {
                id: row.get(0),
                stock_per: row.get(1),
                deviation: row.get(2),
                name: row.get(3),
                description: row.get(4),
            }).ok_or_else(|| FinError::DatabaseErr("no record".to_string()));

        debug!("{:#?}", ret);

        ret
    }

    pub fn get_ticker_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFinErr<Vec<models::TickerGoalData>> {
        let rows = &self
            .conn
            .query(
                "SELECT fk_port_g_id, fk_tic_id, goal_per, ord FROM tic_goal
                WHERE fk_port_g_id = $1",
                &[port_g_id],
            ).map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        let ret = rows
            .iter()
            .map(|row| models::TickerGoalData {
                fk_port_g_id: row.get(0),
                fk_tic_id: row.get(1),
                goal_per: row.get(2),
                ord: row.get(3),
            }).collect::<Vec<models::TickerGoalData>>();

        debug!("{:#?}", ret);
        Ok(ret)
    }

    //========== -> [T]
    pub fn get_tickers_data(
        &self,
        ids: &Vec<i64>,
    ) -> ResultFinErr<Vec<models::TickerData>> {
        let rows = &self
            .conn
            .query(
                "SELECT id, symbol, fk_exchange, fee, kind FROM tickers
                WHERE id = ANY($1)",
                &[&ids],
            ).map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        let ret = rows
            .iter()
            .map(|row| models::TickerData {
                id: row.get(0),
                symbol: row.get(1),
                fk_exchange: row.get(2),
                fee: row.get(3),
                kind: row.get(4),
            }).collect::<Vec<models::TickerData>>();

        debug!("{:#?}", ret);
        Ok(ret)
    }

    //========== (buy) -> Actual -> Goal -> T
}

impl super::TickerDatabase for PgTickerDatabase {
    fn get_tickers(&self) -> HashMap<TickerSymbol, Ticker> {
        unimplemented!();
    }

    fn get_goal(&self) -> HashMap<TickerSymbol, portfolio::TickerGoal> {
        unimplemented!();
    }

    fn get_actual(&self) -> HashMap<TickerSymbol, portfolio::TickerActual> {
        unimplemented!();
    }
}
