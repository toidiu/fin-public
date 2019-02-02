#![allow(dead_code, unused)]

mod db_types;

use crate::backend;
use crate::errors::{FinError, ResultFin};
use crate::portfolio::{self, Ticker, TickerId};
use crate::std_ext::ExtIterator;
use postgres::Connection;
use std::collections::HashMap;

use postgres_mapper;
use postgres_mapper::FromPostgresRow;
use r2d2;

pub(crate) use self::db_types::*;

pub struct PgFinDb {
    pub conn: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>,
}

pub trait FinDb {
    //========== (login) -> user
    fn get_user(&self, email: &String) -> ResultFin<db_types::UserData>;

    fn get_user_with_pass(
        &self,
        email: &String,
    ) -> ResultFin<db_types::UserDataWithPass>;

    fn does_user_exist(&self, email: &String) -> ResultFin<bool>;

    fn create_user(
        &self,
        email: &String,
        password: &String,
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

    fn get_port_goals(&self) -> ResultFin<Vec<db_types::PortGoalData>>;

    //========== -> Pg -> [Tg]
    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<db_types::PortGoalData>;

    fn get_ticker_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<Vec<db_types::TickerGoalData>>;

    fn get_ticker_goal_detailed(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<Vec<db_types::TickerGoalDetailData>>;

    //========== -> [T]
    fn get_tickers_by_ids(
        &self,
        ids: &Vec<i64>,
    ) -> ResultFin<Vec<db_types::TickerData>>;

    //========== (buy) -> Actual -> Goal -> T
}

impl FinDb for PgFinDb {
    //========== (login) -> user
    fn get_user(&self, email: &String) -> ResultFin<db_types::UserData> {
        // table users
        let stmt = &format!(
            "SELECT {} FROM {} WHERE email = $1",
            &db_types::UserData::sql_fields(),
            &db_types::UserData::sql_table(),
        );

        let rows = &self
            .conn
            .query(stmt, &[email])
            // .unwrap();
            .map_err(|err| {
                error!("{}", err);
                FinError::DatabaseErr
            })?;

        let ret: ResultFin<db_types::UserData> = rows
            .iter()
            .next()
            .map(|row| {
                db_types::UserData::from_postgres_row(row).map_err(|err| {
                    error!("{}", err);
                    FinError::DatabaseErr
                })
            })
            .ok_or(FinError::DatabaseErr)?;

        ret
    }

    fn get_user_with_pass(
        &self,
        email: &String,
    ) -> ResultFin<db_types::UserDataWithPass> {
        // table users
        let stmt = &format!(
            "SELECT {} FROM {} WHERE email = $1",
            &db_types::UserDataWithPass::sql_fields(),
            &db_types::UserDataWithPass::sql_table(),
        );

        let rows = &self
            .conn
            .query(stmt, &[email])
            // .unwrap();
            .map_err(|err| {
                error!("{}", err);
                FinError::DatabaseErr
            })?;

        let ret: ResultFin<db_types::UserDataWithPass> = rows
            .iter()
            .next()
            .map(|row| {
                db_types::UserDataWithPass::from_postgres_row(row).map_err(
                    |err| {
                        error!("{}", err);
                        FinError::DatabaseErr
                    },
                )
            })
            .ok_or(FinError::DatabaseErr)?;

        ret
    }

    fn does_user_exist(&self, email: &String) -> ResultFin<bool> {
        // table users
        let stmt = &format!(
            "SELECT 1 from {} where email=$1",
            &db_types::UserData::sql_table(),
        );

        let rows = &self.conn.query(stmt, &[email]).map_err(|err| {
            error!("{} {}", err, line!());
            FinError::DatabaseErr
        })?;

        let ret = !rows.is_empty();
        Ok(ret)
    }

    fn create_user(
        &self,
        email: &String,
        password: &String,
    ) -> ResultFin<db_types::UserData> {
        let stmt = &format!(
            "INSERT INTO {}
            (email, password)
            VALUES ($1, $2) RETURNING *",
            &db_types::UserData::sql_table(),
        );

        let rows = &self
            .conn
            .query(stmt, &[email, password])
            // .unwrap();
            .map_err(|err| {
                error!("{} {}", err, line!());
                FinError::DatabaseErr
            })?;

        let ret: ResultFin<db_types::UserData> = rows
            .iter()
            .next()
            .map(|row| {
                db_types::UserData::from_postgres_row(row).map_err(|err| {
                    error!("{} {}", err, line!());
                    FinError::DatabaseErr
                })
            })
            .ok_or(FinError::DatabaseErr)?;

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
        let rows =
            &self
                .conn
                .query(stmt, &[user_id, port_g_id])
                .map_err(|err| {
                    error!("{}", err);
                    FinError::DatabaseErr
                })?;

        let ret = rows
            .iter()
            .map(|row| {
                let ret = db_types::TickerActualData {
                    id: row
                        .get_opt("id")
                        .ok_or_else(|| FinError::DatabaseErr)??,
                    fk_user_id: row
                        .get_opt("fk_user_id")
                        .ok_or(FinError::DatabaseErr)??,
                    fk_port_g_id: row
                        .get_opt("fk_port_g_id")
                        .ok_or_else(|| FinError::DatabaseErr)??,
                    fk_tic_id: row
                        .get_opt("fk_tic_id")
                        .ok_or_else(|| FinError::DatabaseErr)??,
                    actual_shares: row
                        .get_opt("actual_shares")
                        .ok_or_else(|| FinError::DatabaseErr)??,
                    version: row
                        .get_opt("version")
                        .ok_or_else(|| FinError::DatabaseErr)??,
                    tsz: row
                        .get_opt("tsz")
                        .ok_or_else(|| FinError::DatabaseErr)??,
                };
                Ok(ret)
            })
            .collect::<ResultFin<Vec<db_types::TickerActualData>>>();

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

        let tx = &self.conn.transaction().map_err(|err| {
            error!("{}", err);
            FinError::DatabaseErr
        })?;

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
                        )
                        .map_err(|err| {
                            error!("{}", err);
                            FinError::DatabaseErr
                        })?;

                    let ret: ResultFin<db_types::TickerActualData> = rows
                        .into_iter()
                        .next()
                        .map(|row| {
                            db_types::TickerActualData::from_postgres_row(row)
                                .map_err(|err| {
                                    error!("{}", err);
                                    FinError::DatabaseErr
                                })
                        })
                        .unwrap();

                    ret
                })
                .collect();

        tx.set_commit();
        d.into_iter().collect()
    }

    //========== -> Pg -> [Tg]
    fn get_port_goals(&self) -> ResultFin<Vec<db_types::PortGoalData>> {
        let rows = &self
            .conn
            .query(
                "SELECT id, stock_per, deviation, name, description FROM port_goal",
                &[]

            )
            .map_err(|err| {
                error!("{}", err);
                FinError::DatabaseErr
            })?;

        let ret = rows
            .iter()
            .map(|row| db_types::PortGoalData {
                id: row.get(0),
                stock_per: row.get(1),
                deviation: row.get(2),
                name: row.get(3),
                description: row.get(4),
            })
            .collect();
        Ok(ret)
    }

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
            )
            .map_err(|err| {
                error!("{}", err);
                FinError::DatabaseErr
            })?;

        let ret = rows
            .iter()
            .next()
            .map(|row| db_types::PortGoalData {
                id: row.get(0),
                stock_per: row.get(1),
                deviation: row.get(2),
                name: row.get(3),
                description: row.get(4),
            })
            .ok_or_else(|| FinError::DatabaseErr);

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
            )
            .map_err(|err| {
                error!("{}", err);
                FinError::DatabaseErr
            })?;

        let ret = rows
            .iter()
            .map(|row| db_types::TickerGoalData {
                fk_port_g_id: row.get(0),
                fk_tic_id: row.get(1),
                goal_per: row.get(2),
                ord: row.get(3),
            })
            .collect::<Vec<db_types::TickerGoalData>>();

        Ok(ret)
    }

    fn get_ticker_goal_detailed(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<Vec<db_types::TickerGoalDetailData>> {
        let stmt = &format!(
            "SELECT tg.fk_port_g_id, tg.fk_tic_id, tg.goal_per, tg.ord, ti.symbol from tic_goal tg
            INNER JOIN {} ti on
            (tg.fk_tic_id = ti.id AND tg.fk_port_g_id = $1);",
            &db_types::TickerData::sql_table()
        );

        let rows = &self.conn.query(stmt, &[port_g_id]).map_err(|err| {
            error!("{}", err);
            FinError::DatabaseErr
        })?;

        rows.iter()
            .map(|row| db_types::TickerGoalDetailData::from_postgres_row(row))
            .collect::<Result<Vec<db_types::TickerGoalDetailData>, postgres_mapper::Error>>()
            .map_err(|err| {
                error!("{}", err);
                FinError::DatabaseErr
            })

        //         let rows = &self
        //             .conn
        //             .query(
        //                 "SELECT fk_port_g_id, fk_tic_id, goal_per, ord FROM tic_goal
        //                 WHERE fk_port_g_id = $1",
        //                 &[port_g_id],
        //             )
        //             .map_err(|err| {
        //                 error!("{}", err);
        //                 FinError::DatabaseErr
        //             })?;

        //         let ret = rows
        //             .iter()
        //             .map(|row| db_types::TickerGoalDetailData {
        //                 fk_port_g_id: row.get(0),
        //                 fk_tic_id: row.get(1),
        //                 goal_per: row.get(2),
        //                 ord: row.get(3),
        //                 symbol: row.get(4),
        //             })
        //             .collect::<Vec<db_types::TickerGoalDetailData>>();

        //         Ok(ret)
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

        let rows = &self.conn.query(stmt, &[ids]).map_err(|err| {
            error!("{}", err);
            FinError::DatabaseErr
        })?;

        rows.iter()
            .map(|row| db_types::TickerData::from_postgres_row(row))
            .collect::<Result<Vec<db_types::TickerData>, postgres_mapper::Error>>()
            .map_err(|err| {
                error!("{}", err);
                FinError::DatabaseErr
            })
    }

    //========== (buy) -> Actual -> Goal -> T
}
