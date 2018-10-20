use crate::std_ext::ExtIterator;
use crate::{portfolio, ticker::*};
use lru_time_cache::LruCache;
use postgres::Connection;
use std::collections::HashMap;

use super::NewDatabase;
use crate::errors::{FinError, ResultFinErr};
use crate::models;

pub struct PgTickerDatabase {
    pub conn: Connection,
    pub lru: LruCache<String, f32>,
}

impl NewDatabase for PgTickerDatabase {
    //========== (login) -> user
    fn get_user(
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

        ret
    }

    //========== -> Pa -> [Ta]
    fn get_ticker_actual(
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

        Ok(ret)
    }

    //========== -> Pg -> [Tg]
    fn get_port_goal(
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

        ret
    }

    fn get_ticker_goal(
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

        Ok(ret)
    }

    //========== -> [T]
    fn get_tickers_data(
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

        Ok(ret)
    }

    //========== (buy) -> Actual -> Goal -> T
}

impl super::TickerDatabase for PgTickerDatabase {
    fn get_tickers(&mut self, ids: &Vec<i64>) -> HashMap<TickerId, Ticker> {
        let res_tickers = self.get_tickers_data(ids);
        let mut tic_map = HashMap::new();
        let iex = iex_rust::Iex {};

        // get
        if let Ok(tickers) = res_tickers {
            let symbol_list: Vec<String> =
                tickers.iter().map(|x| x.symbol.clone()).collect();

            let mut p_map: HashMap<String, f32> = HashMap::new();
            debug!("symbol list pre filter: {:?}", symbol_list);
            let filtered_symb: Vec<String> = symbol_list
                .into_iter()
                .filter(|sym| {
                    // we peek so as not to reset the timestamp
                    let opt_exists = self.lru.peek(sym);
                    match opt_exists {
                        Some(price) => {
                            p_map.insert(sym.clone(), *price);
                            false
                        }
                        None => true,
                    }
                }).collect();
            debug!("symbol list after filtered: {:?}", filtered_symb);

            if (filtered_symb.len() > 0) {
                let filtered_p_map = iex.get_price(filtered_symb).unwrap();
                // add filtered_p_map to p_map and also lru
                for (s, p) in filtered_p_map {
                    p_map.insert(s.clone(), p.price);
                    self.lru.insert(s.clone(), p.price);
                }
            }

            for x in tickers {
                // if let Some(price) = p_map.get(&x.symbol) {
                let price = p_map.get(&x.symbol).expect("expected ticker price to be here either through iex or lru");
                tic_map.insert(tic_id!(x.id.clone()), x.to_ticker(*price));
                // }
            }
        };

        tic_map
    }

    fn get_goal(&self) -> HashMap<TickerId, portfolio::TickerGoal> {
        let tg = self.get_ticker_goal(&1);
        let mut map = HashMap::new();
        if let Ok(g_tickers) = tg {
            for x in g_tickers {
                map.insert(tic_id!(x.fk_tic_id.clone()), x.to_tic_goal());
            }
        };

        map
    }

    fn get_actual(&self) -> HashMap<TickerId, portfolio::TickerActual> {
        let ta = self.get_ticker_actual(&1, &1);
        let mut map = HashMap::new();
        if let Ok(a_tickers) = ta {
            for x in a_tickers {
                map.insert(tic_id!(x.fk_tic_id.clone()), x.to_tic_actual());
            }
        };

        map
    }
}
