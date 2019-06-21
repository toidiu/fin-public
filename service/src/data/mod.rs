mod db_types;
mod test_helper;

use crate::backend;
use crate::errors::{FinError, ResultFin};
use crate::portfolio;
use crate::server;
use crate::ticker::{InvestmentKind, Ticker, TickerId, TickerSymbol};
use chrono::prelude::*;
use postgres::Connection;
use std::collections::HashMap;

use postgres_mapper;
use postgres_mapper::FromPostgresRow;
use r2d2;

pub(crate) use self::db_types::*;

pub struct PgFinDb {
    pub conn: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>,
    logger: slog::Logger,
}

impl PgFinDb {
    pub fn new(
        conn: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>,
        logger: slog::Logger,
    ) -> Self {
        PgFinDb {
            conn: conn,
            logger: logger.new(o!("mod" => "data")),
        }
    }
}

pub trait FinDb {
    //========== USER
    fn get_user(&self, email: &str) -> ResultFin<db_types::UserData>;

    fn get_user_with_pass(
        &self,
        email: &str,
    ) -> ResultFin<db_types::UserDataWithPass>;

    fn does_user_exist(&self, email: &str) -> ResultFin<bool>;

    fn create_user(
        &self,
        email: &str,
        password: &str,
    ) -> ResultFin<db_types::UserData>;

    //========== ACTUAL
    fn get_port_actual_list_by_user_id(
        &self,
        user_id: &server::UserId,
    ) -> ResultFin<Vec<db_types::ActualPortDetailData>>;

    fn get_port_actual(
        &self,
        port_a_id: &i64,
    ) -> ResultFin<db_types::ActualPortData>;

    fn get_actual_tickers(
        &self,
        port_g_id: &i64,
        port_a_id: &i64,
    ) -> ResultFin<Vec<db_types::ActualTickerData>>;

    fn update_tickers_actual(
        &self,
        user_id: &server::UserId,
        current_port_version: &i32,
        updated_tsz: &DateTime<Utc>,
        init_tickers_actual: &Vec<&portfolio::TickerActual>,
        updated_tickers_actual: &Vec<&portfolio::TickerActual>,
        init_port: &serde_json::Value,
        new_port: &serde_json::Value,
        actions: &serde_json::Value,
    ) -> ResultFin<db_types::ActualFullData>;

    fn create_portfolio_actual(
        &self,
        user_id: &server::UserId,
        port_g_id: &i64,
        stock_percent: &f32,
    ) -> ResultFin<db_types::ActualPortData>;

    //========== GOAL
    fn get_port_goals(&self) -> ResultFin<Vec<db_types::GoalPortData>>;

    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<db_types::GoalPortData>;

    fn get_ticker_goal_by_id(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<Vec<db_types::GoalTickerData>>;

    fn get_ticker_goal_detailed(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<Vec<db_types::GoalTickerDetailData>>;

    //========== TICKERS
    fn get_tickers_by_ids(
        &self,
        ids: &Vec<i64>,
    ) -> ResultFin<Vec<db_types::TickerData>>;
}

impl FinDb for PgFinDb {
    fn get_user(&self, email: &str) -> ResultFin<db_types::UserData> {
        // table users
        let stmt = &format!(
            "SELECT {} FROM {} WHERE email = $1",
            &db_types::UserData::sql_fields(),
            &db_types::UserData::sql_table(),
        );

        let rows = &self.conn.query(stmt, &[&email]).map_err(|err| {
            error!(self.logger, "{}: {}", line!(), err);
            lineError!(self.logger, err);
            FinError::DatabaseErr
        })?;

        let ret: ResultFin<db_types::UserData> = rows
            .iter()
            .next()
            .map(|row| {
                db_types::UserData::from_postgres_row(row).map_err(|err| {
                    error!(self.logger, "{}: {}", line!(), err);
                    FinError::DatabaseErr
                })
            })
            .ok_or(FinError::DatabaseErr)?;

        ret
    }

    fn get_user_with_pass(
        &self,
        email: &str,
    ) -> ResultFin<db_types::UserDataWithPass> {
        // table users
        let stmt = &format!(
            "SELECT {} FROM {} WHERE email = $1",
            &db_types::UserDataWithPass::sql_fields(),
            &db_types::UserDataWithPass::sql_table(),
        );

        let rows = &self.conn.query(stmt, &[&email]).map_err(|err| {
            error!(self.logger, "{}: {}", line!(), err);
            FinError::DatabaseErr
        })?;

        let ret: ResultFin<db_types::UserDataWithPass> = rows
            .iter()
            .next()
            .map(|row| {
                db_types::UserDataWithPass::from_postgres_row(row).map_err(
                    |err| {
                        error!(self.logger, "{}: {}", line!(), err);
                        FinError::DatabaseErr
                    },
                )
            })
            .ok_or(FinError::DatabaseErr)?;

        ret
    }

    fn does_user_exist(&self, email: &str) -> ResultFin<bool> {
        // table users
        let stmt = &format!(
            "SELECT 1 from {} where email=$1",
            &db_types::UserData::sql_table(),
        );

        let rows = &self.conn.query(stmt, &[&email]).map_err(|err| {
            error!(self.logger, "{} {}", err, line!());
            FinError::DatabaseErr
        })?;

        let ret = !rows.is_empty();
        Ok(ret)
    }

    fn create_user(
        &self,
        email: &str,
        password: &str,
    ) -> ResultFin<db_types::UserData> {
        let stmt = &format!(
            "INSERT INTO {}
            (email, password)
            VALUES ($1, $2) RETURNING *",
            &db_types::UserData::sql_table(),
        );

        let rows =
            &self.conn.query(stmt, &[&email, &password]).map_err(|err| {
                error!(self.logger, "{} {}", err, line!());
                FinError::DatabaseErr
            })?;

        let ret: ResultFin<db_types::UserData> = rows
            .iter()
            .next()
            .map(|row| {
                db_types::UserData::from_postgres_row(row).map_err(|err| {
                    error!(self.logger, "{} {}", err, line!());
                    FinError::DatabaseErr
                })
            })
            .ok_or(FinError::DatabaseErr)?;

        ret
    }

    fn get_port_actual_list_by_user_id(
        &self,
        user_id: &server::UserId,
    ) -> ResultFin<Vec<db_types::ActualPortDetailData>> {
        let stmt = &format!(
            "SELECT ap.id, ap.fk_user_id, ap.fk_port_g_id, ap.stock_percent,
            ap.deviation, ap.version, ap.last_updated, gp.name, gp.description
            FROM {} ap JOIN {} gp on
            (ap.fk_port_g_id = gp.id AND ap.fk_user_id = $1)",
            &db_types::ActualPortData::sql_table(),
            &db_types::GoalPortData::sql_table(),
        );
        let rows = &self.conn.query(stmt, &[user_id.get_user_id()]).map_err(
            |err| {
                error!(self.logger, "{}: {}", line!(), err);
                FinError::DatabaseErr
            },
        )?;

        let ret =
            rows.iter()
                .map(|row| {
                    db_types::ActualPortDetailData::from_postgres_row(row)
                        .map_err(|err| {
                            lineError!(
                                self.logger,
                                format!("{}. user_id: {:?}", err, &user_id)
                            );
                            FinError::DatabaseErr
                        })
                })
                .collect::<ResultFin<Vec<db_types::ActualPortDetailData>>>();

        ret
    }

    fn get_port_actual(
        &self,
        port_a_id: &i64,
    ) -> ResultFin<db_types::ActualPortData> {
        let stmt = &format!(
            "SELECT {} FROM {} WHERE id = $1",
            &db_types::ActualPortData::sql_fields(),
            &db_types::ActualPortData::sql_table(),
        );
        let rows = &self.conn.query(stmt, &[port_a_id]).map_err(|err| {
            error!(self.logger, "{}: {}", line!(), err);
            FinError::DatabaseErr
        })?;

        let ret = rows
            .iter()
            .next()
            .map(|row| {
                db_types::ActualPortData::from_postgres_row(row).map_err(
                    |err| {
                        error!(self.logger, "{}: {}", line!(), err);
                        FinError::DatabaseErr
                    },
                )
            })
            .ok_or(FinError::DatabaseErr)?;

        ret
    }

    fn get_actual_tickers(
        &self,
        port_g_id: &i64,
        port_a_id: &i64,
    ) -> ResultFin<Vec<db_types::ActualTickerData>> {
        let stmt = &format!(
            "SELECT {} FROM {}
            WHERE fk_port_g_id = $1 AND fk_port_a_id = $2",
            &db_types::ActualTickerData::sql_fields(),
            &db_types::ActualTickerData::sql_table()
        );
        let rows =
            &self
                .conn
                .query(stmt, &[port_g_id, port_a_id])
                .map_err(|err| {
                    error!(self.logger, "{}: {}", line!(), err);
                    FinError::DatabaseErr
                })?;

        let ret = rows
            .iter()
            .map(|row| {
                db_types::ActualTickerData::from_postgres_row(row).map_err(
                    |err| {
                        error!(self.logger, "{}: {}", line!(), err);
                        FinError::DatabaseErr
                    },
                )
            })
            .collect::<ResultFin<Vec<db_types::ActualTickerData>>>();

        ret
    }

    fn update_tickers_actual(
        &self,
        user_id: &server::UserId,
        current_port_version: &i32,
        updated_tsz: &DateTime<Utc>,
        init_tickers_actual: &Vec<&portfolio::TickerActual>,
        updated_tickers_actual: &Vec<&portfolio::TickerActual>,
        init_port_data: &serde_json::Value,
        new_port_data: &serde_json::Value,
        actions_data: &serde_json::Value,
    ) -> ResultFin<db_types::ActualFullData> {
        if (updated_tickers_actual.is_empty()) {
            error!(self.logger, "unable to update tickers because updated_tickers_actual is empty");
            return Err(FinError::BadRequestErr);
        }
        let tic = updated_tickers_actual
            .first()
            .expect(&format!("{} expected non empty vec", line!()));
        let new_version = current_port_version + 1;
        let port_g_id = tic.port_goal_id;
        let port_a_id = tic.port_actual_id;

        let tx = &self.conn.transaction().map_err(|err| {
            error!(self.logger, "{}: {}", line!(), err);
            FinError::DatabaseErr
        })?;

        // ===================
        // save old data
        // ===================
        let stmt_old_backup = format!(
            "INSERT INTO {}
            (fk_port_a_id, version, init_port_a_data, new_port_a_data,
            actions_data, port_action)
            VALUES ($1, $2, $3, $4, $5, $6);",
            &db_types::OldActualPortData::sql_table(),
        );
        tx.execute(
            &stmt_old_backup,
            &[
                &port_a_id,
                &current_port_version,
                &init_port_data,
                &new_port_data,
                &actions_data,
                &DomainPortAction::new("TICKER".to_string()),
            ],
        )
        .map_err(|err| {
            error!(self.logger, "{}: {}", line!(), err);
            FinError::DatabaseErr
        })?;

        // ===================
        // save actual port
        // ===================
        let stmt_update_port_a = &format!(
            "UPDATE {}
            SET version = $3, last_updated = $4
            WHERE id = $1 AND version = $2
            RETURNING {}",
            &db_types::ActualPortData::sql_table(),
            &db_types::ActualPortData::sql_fields()
        );
        // update port
        let rows = tx
            .query(
                stmt_update_port_a,
                &[
                    // where clause
                    &port_a_id,
                    &current_port_version,
                    // updated values
                    &new_version,
                    &updated_tsz,
                ],
            )
            .map_err(|err| {
                error!(self.logger, "{}: {}", line!(), err);
                FinError::DatabaseErr
            })?;

        let updated_port: ResultFin<db_types::ActualPortData> = rows
            .iter()
            .next()
            .map(|row| {
                db_types::ActualPortData::from_postgres_row(row).map_err(
                    |err| {
                        error!(self.logger, "{}: {}", line!(), err);
                        FinError::DatabaseErr
                    },
                )
            })
            .ok_or(FinError::DatabaseErr)?;

        // ===================
        // save actual tickers
        // ===================
        let stmt_update_tic_a = &format!(
            "UPDATE {}
            SET actual_shares = $4
            WHERE fk_port_g_id = $1 AND fk_port_a_id = $2 AND fk_tic_id = $3
            RETURNING {}",
            &db_types::ActualTickerData::sql_table(),
            &db_types::ActualTickerData::sql_fields()
        );

        // update tic and get updated
        let updated_tic: ResultFin<Vec<db_types::ActualTickerData>> =
            updated_tickers_actual
                .iter()
                .map(|updated_tic| {
                    let rows = tx
                        .query(
                            stmt_update_tic_a,
                            &[
                                // We could use `id = updated_tic.id` but the more
                                // complicated port_g, port_a and tic_id are used
                                // to avoid potentially updating the wrong ticker.
                                // Change behavior if this become a performance
                                // bottle neck.
                                // select clause
                                &port_g_id,
                                &port_a_id,
                                &updated_tic.get_ticker_id(),
                                // updated values
                                &updated_tic.actual_shares,
                            ],
                        )
                        .map_err(|err| {
                            error!(self.logger, "{}: {}", line!(), err);
                            FinError::DatabaseErr
                        })?;

                    let ret: ResultFin<db_types::ActualTickerData> = rows
                        .into_iter()
                        .next()
                        .map(|row| {
                            db_types::ActualTickerData::from_postgres_row(row)
                                .map_err(|err| {
                                    error!(self.logger, "{}: {}", line!(), err);
                                    FinError::DatabaseErr
                                })
                        })
                        .ok_or(FinError::DatabaseErr)?;

                    ret
                })
                .collect();

        tx.set_commit();
        // updated_tic
        updated_tic.and_then(|t| {
            updated_port.map(|p| ActualFullData { port: p, tics: t })
        })
    }

    /// We will create ActualPortData and also ActualTickerData
    /// for each corresponding GoalTickerData.
    fn create_portfolio_actual(
        &self,
        user_id: &server::UserId,
        port_g_id: &i64,
        stock_percent: &f32,
    ) -> ResultFin<db_types::ActualPortData> {
        // get tickers_goal. this can be before the transaction
        let goal_tickers = self.get_ticker_goal_by_id(port_g_id)?;

        let tx = &self.conn.transaction().map_err(|err| {
            error!(self.logger, "{}: {}", line!(), err);
            FinError::DatabaseErr
        })?;

        // insert port actual
        let stmt_port_a = &format!(
            "INSERT INTO {}
            (fk_user_id, fk_port_g_id, stock_percent, deviation, version, last_updated)
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            &db_types::ActualPortData::sql_table(),
        );

        let default_version = 1;
        let rows = tx
            .query(
                stmt_port_a,
                &[
                    user_id.get_user_id(),
                    port_g_id,
                    stock_percent,
                    &portfolio::PERCENT_DEVIATION,
                    &default_version,
                    &Utc::now(),
                ],
            )
            .map_err(|err| {
                error!(self.logger, "{}: {}", line!(), err);
                FinError::DatabaseErr
            })?;

        let new_actual_port: db_types::ActualPortData = rows
            .iter()
            .next()
            .map(|row| {
                db_types::ActualPortData::from_postgres_row(row).map_err(
                    |err| {
                        error!(self.logger, "{} {}", err, line!());
                        FinError::DatabaseErr
                    },
                )
            })
            .ok_or(FinError::DatabaseErr)??;

        let stmt_tic_a = format!(
            "INSERT INTO {}
            (fk_port_g_id, fk_port_a_id, fk_tic_id, actual_shares)
            VALUES ($1, $2, $3, $4);",
            &db_types::ActualTickerData::sql_table(),
        );
        for gt in goal_tickers {
            // insert ActualTickerData
            tx.execute(
                &stmt_tic_a,
                &[&port_g_id, &new_actual_port.id, &gt.fk_tic_id, &0.0],
            )?;
        }

        tx.set_commit();
        Ok(new_actual_port)
    }

    fn get_port_goals(&self) -> ResultFin<Vec<db_types::GoalPortData>> {
        let stmt = &format!(
            "SELECT {} FROM {}",
            &db_types::GoalPortData::sql_fields(),
            &db_types::GoalPortData::sql_table(),
        );
        let rows = &self.conn.query(stmt, &[]).map_err(|err| {
            error!(self.logger, "{}: {}", line!(), err);
            FinError::DatabaseErr
        })?;

        let ret = rows
            .iter()
            .map(|row| db_types::GoalPortData::from_postgres_row(row))
            .collect::<Result<Vec<db_types::GoalPortData>, postgres_mapper::Error>>()
            .map_err(|err| {
                error!(self.logger, "{}: {}", line!(), err);
                FinError::DatabaseErr
            });
        ret
    }

    fn get_port_goal(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<db_types::GoalPortData> {
        let stmt = &format!(
            "SELECT {} FROM {} WHERE id = $1",
            &db_types::GoalPortData::sql_fields(),
            &db_types::GoalPortData::sql_table(),
        );
        let rows = &self.conn.query(stmt, &[port_g_id]).map_err(|err| {
            error!(self.logger, "{}: {}", line!(), err);
            FinError::DatabaseErr
        })?;

        let ret = rows
            .iter()
            .next()
            .map(|row| {
                db_types::GoalPortData::from_postgres_row(row).map_err(|err| {
                    error!(self.logger, "{}: {}", line!(), err);
                    FinError::DatabaseErr
                })
            })
            .ok_or(FinError::DatabaseErr)?;

        ret
    }

    fn get_ticker_goal_by_id(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<Vec<db_types::GoalTickerData>> {
        let stmt = &format!(
            "SELECT {} FROM {} WHERE fk_port_g_id = $1",
            &db_types::GoalTickerData::sql_fields(),
            &db_types::GoalTickerData::sql_table(),
        );
        let rows = &self.conn.query(stmt, &[port_g_id]).map_err(|err| {
            error!(self.logger, "{}: {}", line!(), err);
            FinError::DatabaseErr
        })?;

        let ret = rows
            .iter()
            .map(|row| db_types::GoalTickerData::from_postgres_row(row))
            .collect::<Result<Vec<db_types::GoalTickerData>, postgres_mapper::Error>>()
            .map_err(|err| {
                error!(self.logger, "{}: {}", line!(), err);
                FinError::DatabaseErr
            });

        ret
    }

    fn get_ticker_goal_detailed(
        &self,
        port_g_id: &i64,
    ) -> ResultFin<Vec<db_types::GoalTickerDetailData>> {
        let stmt = &format!(
            "SELECT gt.fk_port_g_id, gt.fk_tic_id, gt.tic_goal_per, gt.ord, ti.symbol from goal_tic gt
            INNER JOIN {} ti on
            (gt.fk_tic_id = ti.id AND gt.fk_port_g_id = $1);",
            &db_types::TickerData::sql_table()
        );

        let rows = &self.conn.query(stmt, &[port_g_id]).map_err(|err| {
            error!(self.logger, "{}: {}", line!(), err);
            FinError::DatabaseErr
        })?;

        rows.iter()
            .map(|row| db_types::GoalTickerDetailData::from_postgres_row(row))
            .collect::<Result<Vec<db_types::GoalTickerDetailData>, postgres_mapper::Error>>()
            .map_err(|err| {
                error!(self.logger, "{}: {}", line!(), err);
                FinError::DatabaseErr
            })
    }

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
            error!(self.logger, "{}: {}", line!(), err);
            FinError::DatabaseErr
        })?;

        rows.iter()
            .map(|row| db_types::TickerData::from_postgres_row(row))
            .collect::<Result<Vec<db_types::TickerData>, postgres_mapper::Error>>()
            .map_err(|err| {
                error!(self.logger, "{}: {}", line!(), err);
                FinError::DatabaseErr
            })
    }
}

#[cfg(test)]
mod tests {

    use super::test_helper::TestHelper;
    use super::*;

    #[test]
    fn test_get_user() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.get_user("apoorv@toidiu.com");
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap().email, "apoorv@toidiu.com");
        })
    }

    #[test]
    fn test_get_user_with_pass() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.get_user_with_pass("apoorv@toidiu.com");
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap().email, "apoorv@toidiu.com");
        })
    }

    #[test]
    fn test_does_user_exist() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.does_user_exist("apoorv@toidiu.com");
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap(), true);

            let res = db.does_user_exist("not@toidiu.com");
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap(), false);
        })
    }

    #[test]
    fn test_create_user() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.create_user("1@toidiu.com", "123456");
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap().email, "1@toidiu.com");

            let res = db.get_user_with_pass("1@toidiu.com");
            assert_eq!(res.is_ok(), true);
            assert_eq!(res.unwrap().password, "123456");
        });
    }

    #[test]
    fn test_get_port_actual_list_by_user_id() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.get_port_actual_list_by_user_id(&user_id!(1));

            assert_eq!(res.is_ok(), true);
            let r = &res.unwrap();
            assert_eq!(&r.len(), &2);
            assert_eq!(&r.get(1).unwrap().fk_user_id, &1);
            assert_eq!(&r.get(1).unwrap().stock_percent, &58.0);
            assert_eq!(&r.get(1).unwrap().name, "Value Portfolio");
            assert_eq!(&r.get(0).unwrap().stock_percent, &90.0);
            assert_eq!(&r.get(0).unwrap().name, "Value Portfolio");
        })
    }

    #[test]
    fn test_get_port_actual_list_by_user_id_sql_join_stmt() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.get_port_actual_list_by_user_id(&user_id!(2));

            assert_eq!(res.is_ok(), true);
            let r = &res.unwrap();
            assert_eq!(&r.len(), &1);
            assert_eq!(&r.get(0).unwrap().fk_user_id, &2);
            assert_eq!(&r.get(0).unwrap().stock_percent, &50.0);
            assert_eq!(&r.get(0).unwrap().name, "Value Portfolio");
        })
    }

    #[test]
    fn test_get_port_actual() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.get_port_actual(&1);

            assert_eq!(res.is_ok(), true);
            let r = &res.unwrap();
            assert_eq!(&r.id, &1);
            assert_eq!(&r.fk_user_id, &1);
        })
    }

    #[test]
    fn test_get_ticker_actual() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.get_actual_tickers(&1, &1);

            assert_eq!(res.is_ok(), true);
            assert!(!res.unwrap().is_empty());

            let res = db.get_actual_tickers(&12, &1000);
            assert_eq!(res.is_ok(), true);
            assert!(res.unwrap().is_empty());
        })
    }

    #[test]
    fn test_update_tickers_actual() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);

            let old_time =
                "2014-11-28T12:00:09Z".parse::<DateTime<Utc>>().unwrap();
            let new_time =
                "2015-11-28T12:00:09Z".parse::<DateTime<Utc>>().unwrap();
            let new_shares = 100.0;
            let curr_version = 1;
            let new_version = curr_version + 1;

            let init_tic = portfolio::TickerActual::new(1, 1, 1, 1, 0.0);

            let updated_tic =
                portfolio::TickerActual::new(1, 1, 1, 1, new_shares);

            let res = db.update_tickers_actual(
                &user_id!(1),
                &curr_version,
                &new_time,
                &vec![&init_tic],
                &vec![&updated_tic],
                &json!("init_port_data"),
                &json!("new_port_data"),
                &json!("actions_data"),
            );

            assert_eq!(res.is_ok(), true);
            let res_unwrap = &res.unwrap();
            assert_eq!(
                &res_unwrap.tics.get(0).unwrap().actual_shares,
                &new_shares
            );
            assert_eq!(&res_unwrap.port.version, &new_version);
        })
    }

    #[test]
    fn test_update_tickers_actual_id_2() {
        TestHelper::run_test_opt_teardown(true, |db_name| {
            let db = TestHelper::get_test_db(db_name);

            let user_id = &user_id!(1);
            let port_g_id = 1;
            let port_a_id = 2;
            let curr_version = 1;
            let new_version = curr_version + 1;

            let old_time =
                "2014-11-28T12:00:09Z".parse::<DateTime<Utc>>().unwrap();
            let new_time =
                "2015-11-28T12:00:09Z".parse::<DateTime<Utc>>().unwrap();

            let new_shares = 100.0;
            let tic_unique_id = 12;
            let ticker_id = 1;
            let init_tic = portfolio::TickerActual::new(
                tic_unique_id,
                port_g_id,
                port_a_id,
                ticker_id,
                0.0,
            );

            let updated_tic = portfolio::TickerActual::new(
                tic_unique_id,
                port_g_id,
                port_a_id,
                ticker_id,
                new_shares,
            );
            let res = db.update_tickers_actual(
                &user_id,
                &curr_version,
                &new_time,
                &vec![&init_tic],
                &vec![&updated_tic],
                &json!("init_port_data"),
                &json!("new_port_data"),
                &json!("actions_data"),
            );

            assert_eq!(res.is_ok(), true);
            let res_unwrap = &res.unwrap();
            assert_eq!(
                &res_unwrap.tics.get(0).unwrap().actual_shares,
                &new_shares
            );
            assert_eq!(&res_unwrap.port.version, &new_version);

            // --------------------- second update
            let new_version_2 = new_version + 1;
            let new_time_2 =
                "2016-11-28T12:00:09Z".parse::<DateTime<Utc>>().unwrap();
            let new_shares_2 = 30.0;
            let ticker_id_2 = 2;
            let tic_unique_id_2 = 13;
            let init_tic_2 = portfolio::TickerActual::new(
                tic_unique_id_2,
                port_g_id,
                port_a_id,
                ticker_id_2,
                0.0,
            );
            let updated_tic_2 = portfolio::TickerActual::new(
                tic_unique_id_2,
                port_g_id,
                port_a_id,
                ticker_id_2,
                new_shares_2,
            );
            let res = db.update_tickers_actual(
                &user_id,
                &new_version,
                &new_time_2,
                &vec![&init_tic_2],
                &vec![&updated_tic_2],
                &json!("init_port_data"),
                &json!("new_port_data"),
                &json!("actions_data"),
            );

            assert_eq!(res.is_ok(), true);
            let res_unwrap = &res.unwrap();
            assert_eq!(
                &res_unwrap.tics.get(0).unwrap().actual_shares,
                &new_shares_2
            );
            assert_eq!(&res_unwrap.port.version, &new_version_2);
        })
    }

    #[test]
    fn test_create_portfolio_actual() {
        let user_id = user_id!(1);
        let port_g_id = 1;
        let per = 90.0;

        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.create_portfolio_actual(&user_id, &port_g_id, &per);
            assert_eq!(res.is_ok(), true);
            let res = res.unwrap();
            assert_eq!(&res.fk_user_id, user_id.get_user_id());
            assert_eq!(&res.fk_port_g_id, &port_g_id);
            assert_eq!(&res.stock_percent, &per);
            assert_eq!(&res.deviation, &1.5);
            assert_eq!(&res.version, &1);

            let at = db.get_actual_tickers(&port_g_id, &res.id);
            assert_eq!(at.is_ok(), true);

            let gt = db.get_ticker_goal_by_id(&port_g_id);
            assert_eq!(gt.is_ok(), true);

            let gt = gt.unwrap();
            let at = at.unwrap();
            let at_ids: Vec<i64> = at.iter().map(|x| x.fk_tic_id).collect();

            // assert that we create an actual ticker for each goal ticker
            for t in gt {
                assert!(at_ids.contains(&t.id));
            }

            // asset initial values used to create actual tickers
            for t in at {
                assert_eq!(&t.fk_port_g_id, &port_g_id);
                assert_eq!(&t.fk_port_a_id, &res.id);
                assert_eq!(&t.actual_shares, &0.0);
            }
        })
    }

    #[test]
    fn test_get_port_goals() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.get_port_goals();

            assert_eq!(res.is_ok(), true);
        })
    }

    #[test]
    fn test_get_port_goal() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.get_port_goal(&1);
            assert_eq!(res.is_ok(), true);
        })
    }

    #[test]
    fn test_get_ticker_goal() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.get_ticker_goal_by_id(&1);

            assert_eq!(res.is_ok(), true);
        })
    }

    #[test]
    fn test_get_ticker_goal_detailed() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.get_ticker_goal_detailed(&1);

            assert_eq!(res.is_ok(), true);
        })
    }

    #[test]
    fn test_get_tickers_by_ids() {
        TestHelper::run_test(|db_name| {
            let db = TestHelper::get_test_db(db_name);
            let res = db.get_tickers_by_ids(&vec![1, 2]);

            assert_eq!(res.is_ok(), true);
        })
    }

}
