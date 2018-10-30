use crate::portfolio::{self, Ticker, TickerId};
use crate::std_ext::ExtIterator;
use lru_time_cache::LruCache;
use postgres::Connection;
use std::collections::HashMap;

use super::db_types;
use super::TickerDb;
use crate::errors::{FinError, ResultFinErr};
use postgres_mapper;
use postgres_mapper::FromPostgresRow;

pub struct PgTickerDatabase {
    pub conn: Connection,
    pub lru: LruCache<String, f32>,
}

impl TickerDb for PgTickerDatabase {
    //========== (login) -> user
    fn get_user(
        &self,
        username: &String,
        pass: &String,
    ) -> ResultFinErr<db_types::UserData> {
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
            .map(|row| db_types::UserData {
                id: row.get("id"),
                username: row.get(1),
            }).ok_or_else(|| FinError::DatabaseErr("no record".to_string()));

        ret
    }

    //========== -> Pa -> [Ta]
    fn get_ticker_actual(
        &self,
        user_id: &i64,
        port_g_id: &i64,
    ) -> ResultFinErr<Vec<db_types::TickerActualData>> {
        let stmt = "SELECT
            id,
            fk_user_id,
            fk_port_g_id,
            fk_tic_id,
            actual_shares,
            version,
            tsz
            FROM tic_actual
            WHERE fk_user_id = $1 AND fk_port_g_id = $2";
        let rows = &self
            .conn
            .query(stmt, &[user_id, port_g_id])
            .map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        let ret = rows
            .iter()
            .map(|row| {
                db_types::TickerActualData::from_postgres_row(row)
            }).collect::<Result<Vec<db_types::TickerActualData>, postgres_mapper::Error>>();

        ret.map_err(|err| FinError::DatabaseErr(err.to_string()))
    }

    fn update_tickers_actual(
        &self,
        init_tickers_actual: Vec<portfolio::TickerActual>,
        updated_tickers_actual: Vec<portfolio::TickerActual>,
    ) -> ResultFinErr<Vec<db_types::TickerActualData>> {
        let stmt_old = "INSERT INTO old_port_actual
            (fk_user_id, fk_port_g_id, version, port_a_data)
            VALUES ($1, $2, $3, $4)";
        let stmt_update_tic_a = "UPDATE tic_actual
            SET actual_shares = $4, version = $5, tsz = $6
            WHERE fk_user_id = $1, fk_port_g_id = $2, fk_tic_id = $3
            RETURNING tic_actual.id,
                tic_actual.fk_user_id,
                tic_actual.fk_port_g_id,
                tic_actual.fk_tic_id,
                tic_actual.actual_shares";

        let tic = updated_tickers_actual
            .first()
            .expect("expected non empty vec");
        let old_version = tic.version;
        let new_version = old_version + 1;
        let user_id = tic.user_id;
        let port_g_id = tic.port_goal_id;

        let trans = &self
            .conn
            .transaction()
            .map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        // set old
        let data = serde_json::to_value(init_tickers_actual).unwrap();
        trans.execute(stmt_old, &[&user_id, &port_g_id, &old_version, &data]);

        // set new and get updated
        updated_tickers_actual.iter().map(|updated_tic| {
            let rows = trans
                .query(
                    stmt_update_tic_a,
                    &[
                        // where clause
                        &user_id,
                        &port_g_id,
                        &updated_tic.id,
                        // updated values
                        &updated_tic.actual_shares,
                        &new_version,
                        &updated_tic.tsz,
                    ],
                ).map_err(|err| FinError::DatabaseErr(err.to_string()))?;

            let ret =
                rows.into_iter()
                    .map(|row| {
                        db_types::TickerActualData::from_postgres_row(row)
                    }).collect::<Result<
                        Vec<db_types::TickerActualData>,
                        postgres_mapper::Error,
                    >>();

            ret.map_err(|err| FinError::DatabaseErr(err.to_string()))
        });

        for updated_tic in updated_tickers_actual {
            trans.execute(
                stmt_update_tic_a,
                &[
                    // where clause
                    &user_id,
                    &port_g_id,
                    &updated_tic.id,
                    // updated values
                    &updated_tic.actual_shares,
                    &new_version,
                    &updated_tic.tsz,
                ],
            );
        }

        trans.set_commit();
        unimplemented!()
    }

    //========== -> Pg -> [Tg]
    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFinErr<db_types::PortGoalData> {
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
            .map(|row| db_types::PortGoalData {
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
    ) -> ResultFinErr<Vec<db_types::TickerGoalData>> {
        let rows = &self
            .conn
            .query(
                "SELECT fk_port_g_id, fk_tic_id, goal_per, ord FROM tic_goal
                WHERE fk_port_g_id = $1",
                &[port_g_id],
            ).map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        let ret = rows
            .iter()
            .map(|row| db_types::TickerGoalData {
                fk_port_g_id: row.get(0),
                fk_tic_id: row.get(1),
                goal_per: row.get(2),
                ord: row.get(3),
            }).collect::<Vec<db_types::TickerGoalData>>();

        Ok(ret)
    }

    //========== -> [T]
    fn get_tickers_by_ids(
        &self,
        ids: &Vec<i64>,
    ) -> ResultFinErr<Vec<db_types::TickerData>> {
        let rows = &self
            .conn
            .query(
                "SELECT id, symbol, fk_exchange, fee, kind FROM tickers
                WHERE id = ANY($1)",
                &[&ids],
            ).map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        let ret = rows
            .iter()
            .map(|row| {
                db_types::TickerData::from_postgres_row(row)
            }).collect::<Result<Vec<db_types::TickerData>, postgres_mapper::Error>>();

        ret.map_err(|err| FinError::DatabaseErr(err.to_string()))
    }

    //========== (buy) -> Actual -> Goal -> T
}

impl super::TickerBackend for PgTickerDatabase {
    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFinErr<db_types::PortGoalData> {
        TickerDb::get_port_goal(self, port_g_id)
    }

    fn get_tickers(&mut self, ids: &Vec<i64>) -> HashMap<TickerId, Ticker> {
        let res_tickers = self.get_tickers_by_ids(ids);
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

    fn get_goal(
        &self,
        port_g_id: &i64,
    ) -> HashMap<TickerId, portfolio::TickerGoal> {
        let tg = self.get_ticker_goal(port_g_id);
        let mut map = HashMap::new();
        if let Ok(g_tickers) = tg {
            for x in g_tickers {
                map.insert(tic_id!(x.fk_tic_id.clone()), x.to_tic_goal());
            }
        };

        map
    }

    fn get_actual(
        &self,
        user_id: &i64,
        port_g_id: &i64,
    ) -> ResultFinErr<HashMap<TickerId, portfolio::TickerActual>> {
        let ta = self.get_ticker_actual(user_id, port_g_id)?;
        let mut map = HashMap::new();

        for x in ta {
            map.insert(tic_id!(x.fk_tic_id.clone()), x.to_tic_actual());
        }

        Ok(map)
    }
}
