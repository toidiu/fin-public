use crate::algo::{self, BuyNext};
use crate::algo::{Action, ActionInfo};
use crate::data;
use crate::errors::*;
use crate::portfolio;
use crate::server;
use crate::std_ext::ExtIterator;
use crate::ticker::{InvestmentKind, Ticker, TickerId, TickerSymbol};
use chrono::prelude::*;
use postgres::Connection;
use std::collections::HashMap;

pub trait PortfolioBackend {
    fn get_tickers(&self, ids: &Vec<i64>) -> HashMap<TickerId, Ticker>;

    fn get_port_goals(&self)
        -> ResultFin<Vec<server::PortfolioGoalDetailResp>>;

    // FIXME return ResultFin
    fn get_tic_goal(
        &self,
        port_g_id: &i64,
    ) -> HashMap<TickerId, portfolio::GoalTicker>;

    // FIXME return ResultFin
    fn get_tic_goal_detailed(
        &self,
        port_g_id: &i64,
    ) -> HashMap<TickerId, portfolio::TickerGoalDetailed>;

    fn get_port_actual_list_by_user_id(
        &self,
        user_id: &i64,
    ) -> ResultFin<Vec<server::PortfolioActualDetailResp>>;

    fn get_port_actual(
        &self,
        port_a_id: &i64,
        actual_tickers: &HashMap<TickerId, portfolio::TickerActual>,
    ) -> ResultFin<portfolio::PortfolioActual>;

    fn get_actual_tickers(
        &self,
        port_g_id: &i64,
        port_a_id: &i64,
    ) -> ResultFin<HashMap<TickerId, portfolio::TickerActual>>;

    fn get_port_goal(
        &self,
        port_g_id: &i64,
        goal_tickers: &HashMap<TickerId, portfolio::GoalTicker>,
        tickers_map: &HashMap<TickerId, Ticker>,
        actual_stock_percent: &f32,
    ) -> ResultFin<portfolio::PortfolioGoal>;

    fn update_actual(
        &self,
        user_id: &i64,
        current_port_version: &i32,
        init_tickers_actual: &Vec<&portfolio::TickerActual>,
        updated_tickers_actual: &Vec<&portfolio::TickerActual>,
        init_port_data: &portfolio::PortfolioActual,
        new_port_data: &portfolio::PortfolioActual,
        actions: &Vec<Action>,
    ) -> ResultFin<Vec<portfolio::TickerActual>>;

    fn create_port_a(
        &self,
        user_id: &i64,
        goal_id: &i64,
        stock_percent: &f32,
    ) -> ResultFin<server::PortfolioActualResp>;

    fn get_buy_next(
        &self,
        user_id: &i64,
        port_g_id: &i64,
        port_a_id: &i64,
        buy_amount: f64,
    ) -> ResultFin<BuyNext>;

    fn execute_actions(
        &self,
        user_id: &i64,
        port_g_id: &i64,
        port_a_id: &i64,
        actions: &Vec<Action>,
    ) -> ResultFin<portfolio::PortfolioState>;
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
    fn get_port_goal(
        &self,
        port_g_id: &i64,
        goal_tickers: &HashMap<TickerId, portfolio::GoalTicker>,
        tickers_map: &HashMap<TickerId, Ticker>,
        actual_stock_percent: &f32,
    ) -> ResultFin<portfolio::PortfolioGoal> {
        self.db.get_port_goal(port_g_id).map(|p| {
            p.to_port_goal(goal_tickers, tickers_map, actual_stock_percent)
        })
    }

    fn get_tickers(&self, ids: &Vec<i64>) -> HashMap<TickerId, Ticker> {
        let res_tickers = self.db.get_tickers_by_ids(ids);
        let mut tic_map = HashMap::new();
        let iex = iex_rs::Iex {};

        // get
        if let Ok(tickers) = res_tickers {
            let symbol_list: Vec<String> =
                tickers.iter().map(|x| x.symbol.clone()).collect();

            let mut p_map: HashMap<String, f64> = HashMap::new();

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

    fn get_port_goals(
        &self,
    ) -> ResultFin<Vec<server::PortfolioGoalDetailResp>> {
        //todo eventually just do a join at the postgres level and have 1 query
        self.db.get_port_goals().map(|v| {
            v.into_iter()
                .map(|data| {
                    let goal_tickers = self.get_tic_goal_detailed(&data.id);
                    server::PortfolioGoalDetailResp::new(data, goal_tickers)
                })
                .collect()
        })
    }

    fn get_tic_goal(
        &self,
        port_g_id: &i64,
    ) -> HashMap<TickerId, portfolio::GoalTicker> {
        let tg = self.db.get_ticker_goal_by_id(port_g_id);
        let mut map = HashMap::new();
        if let Ok(g_tickers) = tg {
            for x in g_tickers {
                map.insert(tic_id!(x.fk_tic_id.clone()), x.into());
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

    fn get_port_actual_list_by_user_id(
        &self,
        user_id: &i64,
    ) -> ResultFin<Vec<server::PortfolioActualDetailResp>> {
        self.db
            .get_port_actual_list_by_user_id(user_id)
            .map(|list| list.into_iter().map(|item| item.to_resp()).collect())
    }

    fn get_port_actual(
        &self,
        port_a_id: &i64,
        actual_tickers: &HashMap<TickerId, portfolio::TickerActual>,
    ) -> ResultFin<portfolio::PortfolioActual> {
        self.db
            .get_port_actual(port_a_id)
            .map(|pa| pa.to_actual_port(actual_tickers))
    }

    fn get_actual_tickers(
        &self,
        port_g_id: &i64,
        port_a_id: &i64,
    ) -> ResultFin<HashMap<TickerId, portfolio::TickerActual>> {
        let ta = self.db.get_actual_tickers(port_g_id, port_a_id)?;
        let mut map = HashMap::new();

        for x in ta {
            map.insert(tic_id!(x.fk_tic_id.clone()), x.into());
        }

        Ok(map)
    }

    fn update_actual(
        &self,
        user_id: &i64,
        current_port_version: &i32,
        init_tickers_actual: &Vec<&portfolio::TickerActual>,
        updated_tickers_actual: &Vec<&portfolio::TickerActual>,
        init_port: &portfolio::PortfolioActual,
        new_port: &portfolio::PortfolioActual,
        actions: &Vec<Action>,
    ) -> ResultFin<Vec<portfolio::TickerActual>> {
        let init_port_data =
            serde_json::to_value(init_port).map_err(|err| {
                error!("{}: {}", line!(), err);
                FinError::ServerErr
            })?;
        let new_port_data = serde_json::to_value(new_port).map_err(|err| {
            error!("{}: {}", line!(), err);
            FinError::ServerErr
        })?;
        let actions_data = serde_json::to_value(&actions).map_err(|err| {
            error!("{}: {}", line!(), err);
            FinError::ServerErr
        })?;
        self.db
            .update_tickers_actual(
                user_id,
                current_port_version,
                &Utc::now(),
                init_tickers_actual,
                updated_tickers_actual,
                &init_port_data,
                &new_port_data,
                &actions_data,
            )
            .map(|res| res.tics.into_iter().map(|x| x.into()).collect())
    }

    fn create_port_a(
        &self,
        user_id: &i64,
        goal_id: &i64,
        stock_percent: &f32,
    ) -> ResultFin<server::PortfolioActualResp> {
        let port_a =
            self.db
                .create_portfolio_actual(user_id, goal_id, stock_percent)?;
        let a_tickers = self
            .db
            .get_actual_tickers(goal_id, &port_a.id)?
            .into_iter()
            .map(|t| t.into())
            .collect();

        Ok(port_a.to_actual_port_resp(&a_tickers))
    }

    fn get_buy_next(
        &self,
        user_id: &i64,
        port_g_id: &i64,
        port_a_id: &i64,
        buy_amount: f64,
    ) -> ResultFin<BuyNext> {
        // actual info
        let tic_actual = self.get_actual_tickers(port_g_id, port_a_id)?;
        let port_actual = self.get_port_actual(port_a_id, &tic_actual)?;

        // tickers info
        let actual_ticker_ids = tic_actual.keys().map(|x| x.0).collect();
        let tickers_map: HashMap<TickerId, Ticker> =
            self.get_tickers(&actual_ticker_ids);

        // goal info
        let goal_tickers = self.get_tic_goal(port_g_id);
        let port_goal = self.get_port_goal(
            port_g_id,
            &goal_tickers,
            &tickers_map,
            &port_actual.stock_percent,
        )?;

        // TODO simplify by capturing above into one fn
        // construct a port state and a BuyNext
        let port = portfolio::PortfolioState::new(
            &port_actual,
            &port_goal,
            &tickers_map,
        );
        let mut buy_next = BuyNext::new(port);

        while (buy_next.buy_value < buy_amount) {
            let port_actual =
                self.get_port_actual(port_a_id, &buy_next.evolved_actual)?;
            let port_state = portfolio::PortfolioState::new(
                &port_actual,
                &port_goal,
                &tickers_map,
            );
            if let None =
                BuyNext::get_next_action(&mut buy_next, buy_amount, port_state)
            {
                break;
            }
        }
        Ok(buy_next)
    }

    fn execute_actions(
        &self,
        user_id: &i64,
        port_g_id: &i64,
        port_a_id: &i64,
        actions: &Vec<Action>,
    ) -> ResultFin<portfolio::PortfolioState> {
        // actual info
        let tic_actual = self.get_actual_tickers(port_g_id, port_a_id)?;
        let port_actual = self.get_port_actual(port_a_id, &tic_actual)?;

        // tickers info
        let actual_ticker_ids = tic_actual.keys().map(|x| x.0).collect();
        let tickers_map: HashMap<TickerId, Ticker> =
            self.get_tickers(&actual_ticker_ids);

        // goal info
        let goal_tickers = self.get_tic_goal(port_g_id);
        let port_goal = self.get_port_goal(
            port_g_id,
            &goal_tickers,
            &tickers_map,
            &port_actual.stock_percent,
        )?;

        let init_port = portfolio::PortfolioState::new(
            &port_actual,
            &port_goal,
            &tickers_map,
        );

        let mut new_port = portfolio::PortfolioState::new(
            &port_actual,
            &port_goal,
            &tickers_map,
        );

        for a in actions {
            new_port = new_port.apply_action(&a);
        }

        // initial ticker values
        let init_tic_actual = tic_actual.values().collect();
        // evolved ticker values
        let evolved_tic_actual = new_port.get_actual_tickers().clone();
        let evolved_tic_actual = evolved_tic_actual.values().collect();

        let inserted_tic_actual = self.update_actual(
            user_id,
            new_port.get_current_version(),
            &init_tic_actual,
            &evolved_tic_actual,
            &init_port.get_actual_port(),
            &new_port.get_actual_port(),
            &actions,
        );

        Ok(new_port)
    }
}
