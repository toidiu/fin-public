use crate::portfolio::{self, Ticker, TickerId};
use crate::std_ext::ExtIterator;
use lru_time_cache::LruCache;
use postgres::Connection;
use std::collections::HashMap;

use super::db_types;
use super::TickerDb;
use crate::errors::{FinError, ResultFin};
use postgres_mapper;
use postgres_mapper::FromPostgresRow;

pub struct PgTickerDatabase {
    pub conn: Connection,
    pub lru: LruCache<String, f32>,
}

impl super::TickerBackend for PgTickerDatabase {
    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<db_types::PortGoalData> {
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
    ) -> ResultFin<HashMap<TickerId, portfolio::TickerActual>> {
        let ta = self.get_ticker_actual(user_id, port_g_id)?;
        let mut map = HashMap::new();

        for x in ta {
            map.insert(tic_id!(x.fk_tic_id.clone()), x.to_tic_actual());
        }

        Ok(map)
    }

    fn update_actual(
        &self,
        init_tickers_actual: &Vec<&portfolio::TickerActual>,
        updated_tickers_actual: &Vec<&portfolio::TickerActual>,
    ) -> ResultFin<Vec<portfolio::TickerActual>> {
        self.update_tickers_actual(init_tickers_actual, updated_tickers_actual)
            .map(|res| res.into_iter().map(|x| x.to_tic_actual()).collect())
    }
}

impl TickerDb for PgTickerDatabase {
    //========== (login) -> user
    fn get_user(
        &self,
        username: &String,
        pass: &String,
    ) -> ResultFin<db_types::UserData> {
        // table users
        let stmt = &format!(
            "SELECT {} FROM {} WHERE username = $1 AND password = $2",
            &db_types::UserData::sql_fields(),
            &db_types::UserData::sql_table(),
        );

        let rows = &self.conn.query(stmt, &[username, pass]).unwrap();

        let ret: ResultFin<db_types::UserData> = rows
            .iter()
            .next()
            .map(|row| {
                db_types::UserData::from_postgres_row(row)
                    .map_err(|err| FinError::DatabaseErr(err.to_string()))
            }).unwrap();

        ret
    }

    //========== -> Pa -> [Ta]
    fn get_ticker_actual(
        &self,
        user_id: &i64,
        port_g_id: &i64,
    ) -> ResultFin<Vec<db_types::TickerActualData>> {
        let stmt = "SELECT
            tic_actual.id,
            tic_actual.fk_user_id,
            tic_actual.fk_port_g_id,
            tic_actual.fk_tic_id,
            tic_actual.actual_shares,
            tic_actual.version,
            tic_actual.tsz
            FROM tic_actual
            WHERE fk_user_id = $1 AND fk_port_g_id = $2";
        let rows = &self
            .conn
            .query(stmt, &[user_id, port_g_id])
            .map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        let ret = rows
            .iter()
            .map(|row| {
                let ret = db_types::TickerActualData {
                    id: row.get_opt("id").ok_or_else(|| {
                        FinError::DatabaseErr("asdf".to_string())
                    })??,
                    // id: row.get_opt("id").unwrap(),
                    fk_user_id: row
                        .get_opt("fk_user_id")
                        .ok_or(FinError::DatabaseErr("asdf".to_string()))??,
                    fk_port_g_id: row.get_opt("fk_port_g_id").ok_or_else(
                        || FinError::DatabaseErr("asdf".to_string()),
                    )??,
                    fk_tic_id: row.get_opt("fk_tic_id").ok_or_else(
                        || FinError::DatabaseErr("asdf".to_string()),
                    )??,
                    actual_shares: row.get_opt("actual_shares").ok_or_else(
                        || FinError::DatabaseErr("asdf".to_string()),
                    )??,
                    version: row.get_opt("version").ok_or_else(|| {
                        FinError::DatabaseErr("asdf".to_string())
                    })??,
                    tsz: row.get_opt("tsz").ok_or_else(|| {
                        FinError::DatabaseErr("asdf".to_string())
                    })??,
                };
                Ok(ret)
            }).collect::<ResultFin<Vec<db_types::TickerActualData>>>();

        ret
    }

    fn update_tickers_actual(
        &self,
        init_tickers_actual: &Vec<&portfolio::TickerActual>,
        updated_tickers_actual: &Vec<&portfolio::TickerActual>,
    ) -> ResultFin<Vec<db_types::TickerActualData>> {
        let stmt_old = "INSERT INTO old_port_actual
            (fk_user_id, fk_port_g_id, version, port_a_data)
            VALUES ($1, $2, $3, $4)";
        // table tic_actual
        let stmt_update_tic_a = &format!(
            "UPDATE {}
            SET actual_shares = $4, version = $5, tsz = $6
            WHERE fk_user_id = $1 AND fk_port_g_id = $2 AND fk_tic_id = $3
            RETURNING {}",
            &db_types::TickerActualData::sql_table(),
            &db_types::TickerActualData::sql_fields()
        );

        let tic = updated_tickers_actual
            .first()
            .expect("expected non empty vec");
        let old_version = tic.version;
        let new_version = old_version + 1;
        let user_id = tic.user_id;
        let port_g_id = tic.port_goal_id;

        let tx = &self
            .conn
            .transaction()
            .map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        // set old
        let data = serde_json::to_value(init_tickers_actual).unwrap();
        tx.execute(stmt_old, &[&user_id, &port_g_id, &old_version, &data]);

        // set new and get updated
        let d: Vec<ResultFin<db_types::TickerActualData>> =
            updated_tickers_actual
                .iter()
                .map(|updated_tic| {
                    let rows = tx
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
                        ).map_err(|err| {
                            FinError::DatabaseErr(err.to_string())
                        })?;

                    let ret: ResultFin<
                        db_types::TickerActualData,
                    > = rows
                        .into_iter()
                        .next()
                        .map(|row| {
                            db_types::TickerActualData::from_postgres_row(row)
                                .map_err(|err| {
                                    FinError::DatabaseErr(err.to_string())
                                })
                        }).unwrap();

                    ret
                }).collect();

        tx.set_commit();
        d.into_iter().collect()
    }

    //========== -> Pg -> [Tg]
    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<db_types::PortGoalData> {
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
    ) -> ResultFin<Vec<db_types::TickerGoalData>> {
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
    ) -> ResultFin<Vec<db_types::TickerData>> {
        let stmt = &format!(
            "SELECT {} FROM {} WHERE id = ANY($1)",
            &db_types::TickerData::sql_fields(),
            &db_types::TickerData::sql_table()
        );

        let rows = &self
            .conn
            .query(stmt, &[ids])
            .map_err(|err| FinError::DatabaseErr(err.to_string()))?;

        rows
            .iter()
            .map(|row| {
                db_types::TickerData::from_postgres_row(row)
            }).collect::<Result<Vec<db_types::TickerData>, postgres_mapper::Error>>()
            .map_err(|err| FinError::DatabaseErr(err.to_string()))
    }

    //========== (buy) -> Actual -> Goal -> T
}
