use crate::api;
use crate::data::{self, *};
use crate::errors::*;
use crate::portfolio::{self, Ticker, TickerId};
use crate::std_ext::ExtIterator;
use postgres::Connection;
use std::collections::HashMap;

pub trait PortfolioBackend {
    fn get_tickers(&self, ids: &Vec<i64>) -> HashMap<TickerId, Ticker>;

    fn get_port_goals(&self) -> ResultFin<Vec<api::PortfolioGoalDetailResp>>;

    fn get_tic_goal(
        &self,
        port_g_id: &i64,
    ) -> HashMap<TickerId, portfolio::TickerGoal>;

    fn get_tic_goal_detailed(
        &self,
        port_g_id: &i64,
    ) -> HashMap<TickerId, portfolio::TickerGoalDetailed>;

    fn get_actual(
        &self,
        user_id: &i64,
        port_g_id: &i64,
    ) -> ResultFin<HashMap<TickerId, portfolio::TickerActual>>;

    fn get_port_goal(&self, port_g_id: &i64) -> ResultFin<data::PortGoalData>;

    fn update_actual(
        &self,
        init_tickers_actual: &Vec<&portfolio::TickerActual>,
        updated_tickers_actual: &Vec<&portfolio::TickerActual>,
    ) -> ResultFin<Vec<portfolio::TickerActual>>;
}

pub struct DefaultPortfolioBackend<T: data::FinDb> {
    db: T,
}

impl<T: data::FinDb> DefaultPortfolioBackend<T> {
    pub fn new(db: T) -> DefaultPortfolioBackend<T> {
        DefaultPortfolioBackend { db: db }
    }
}

impl<T: data::FinDb> PortfolioBackend for DefaultPortfolioBackend<T> {
    fn get_port_goal(&self, port_g_id: &i64) -> ResultFin<data::PortGoalData> {
        self.db.get_port_goal(port_g_id)
    }

    fn get_tickers(&self, ids: &Vec<i64>) -> HashMap<TickerId, Ticker> {
        let res_tickers = self.db.get_tickers_by_ids(ids);
        let mut tic_map = HashMap::new();
        let iex = iex_rs::Iex {};

        // get
        if let Ok(tickers) = res_tickers {
            let symbol_list: Vec<String> =
                tickers.iter().map(|x| x.symbol.clone()).collect();

            let mut p_map: HashMap<String, f32> = HashMap::new();
            debug!("symbol list pre filter: {:?}", symbol_list);

            if (symbol_list.len() > 0) {
                let filtered_p_map = iex.get_price(symbol_list).unwrap();
                // add filtered_p_map to p_map
                for (s, p) in filtered_p_map {
                    p_map.insert(s.clone(), p.price);
                }
            }

            for x in tickers {
                if let Some(price) = p_map.get(&x.symbol) {
                    let price = p_map.get(&x.symbol).expect(
                        "expected ticker price to be here either through iex",
                    );
                    tic_map.insert(tic_id!(x.id.clone()), x.to_ticker(*price));
                }
            }
        };

        tic_map
    }

    fn get_port_goals(&self) -> ResultFin<Vec<api::PortfolioGoalDetailResp>> {
        //todo eventually just do a join at the postgres level and have 1 query
        self.db.get_port_goals().map(|v| {
            v.into_iter()
                .map(|data| {
                    error!("{}", &data.id);
                    let goal_tickers = self.get_tic_goal_detailed(&data.id);
                    api::PortfolioGoalDetailResp::new(data, goal_tickers)
                })
                .collect()
        })
    }

    fn get_tic_goal(
        &self,
        port_g_id: &i64,
    ) -> HashMap<TickerId, portfolio::TickerGoal> {
        let tg = self.db.get_ticker_goal(port_g_id);
        let mut map = HashMap::new();
        if let Ok(g_tickers) = tg {
            for x in g_tickers {
                map.insert(tic_id!(x.fk_tic_id.clone()), x.to_tic_goal());
            }
        };

        map
    }

    fn get_tic_goal_detailed(
        &self,
        port_g_id: &i64,
    ) -> HashMap<TickerId, portfolio::TickerGoalDetailed> {
        let tg = self.db.get_ticker_goal_detailed(port_g_id);
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
        let ta = self.db.get_ticker_actual(user_id, port_g_id)?;
        debug!("==========={:?}", ta);
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
        self.db
            .update_tickers_actual(init_tickers_actual, updated_tickers_actual)
            .map(|res| res.into_iter().map(|x| x.to_tic_actual()).collect())
    }
    //     fn get_user(&self, email: &String) -> ResultFin<data::UserData> {
    //         self.db.get_user(email)
    //     }

    //     fn get_user_with_pass(
    //         &self,
    //         email: &String,
    //     ) -> ResultFin<data::UserDataWithPass> {
    //         self.db.get_user_with_pass(email)
    //     }

    //     fn does_user_exist(&self, email: &String) -> ResultFin<bool> {
    //         self.db.does_user_exist(email)
    //     }

    //     fn create_user(
    //         &self,
    //         email: &String,
    //         password: &String,
    //     ) -> ResultFin<data::UserData> {
    //         self.db.create_user(email, password)
    //     }
}
