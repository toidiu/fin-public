use super::action::{self, *};
use super::actual::{self, *};
use super::buy_next::{self, *};
use super::goal::{self, *};
use super::meta::{self, *};
use crate::backend;
use crate::ticker::{self, *};
use chrono::prelude::*;

use crate::errors::ResultFin;
use crate::{api, data, std_ext::*};
use std::{cmp::Ordering, collections::HashMap, num};

#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    name: String,
    goal: PortfolioGoal,
    actual: actual::PortfolioActual,
    meta: PortfolioMeta,
    tickers: HashMap<TickerId, Ticker>,
}

impl Portfolio {
    pub fn new<T: backend::PortfolioBackend>(
        db: &T,
        actual: &HashMap<TickerId, TickerActual>,
        tickers_map: &HashMap<TickerId, Ticker>,
        pg: &PortfolioGoal,
    ) -> Portfolio {
        let actual = actual.clone();
        // get actual
        let pa = actual::PortfolioActual::new(actual, tickers_map);

        // get meta
        let meta = PortfolioMeta::new(tickers_map, &pg, &pa);

        Portfolio {
            name: pg.name.clone(),
            goal: pg.clone(),
            actual: pa,
            meta: meta,
            tickers: tickers_map.clone(),
        }
    }

    pub fn execute_action<T: backend::PortfolioBackend>(
        db: &T,
        user_id: &i64,
        port_g_id: &i64,
        actions: &Vec<action::Action>,
    ) -> ResultFin<Portfolio> {
        // get port
        let actual = db.get_actual(user_id, port_g_id)?;
        let goal_tickers = db.get_tic_goal(port_g_id);
        let port_goal = db
            .get_port_goal(port_g_id)
            .unwrap()
            .to_port_goal(goal_tickers);

        let keys = actual.keys().map(|x| x.0).collect();
        let tickers_map: HashMap<TickerId, Ticker> = db.get_tickers(&keys);
        let mut port = Portfolio::new(db, &actual, &tickers_map, &port_goal);

        // let mut port = buy_next.init_state;
        for a in actions {
            port = port.evolve(&a);
        }

        // update
        let init_tic_actual = actual.values().collect();
        // get evolved values
        let evolved_tic_actual = port.get_actual_tickers();
        let evolved_tic_actual = evolved_tic_actual.values().collect();
        let inserted_tic_actual =
            db.update_actual(&init_tic_actual, &evolved_tic_actual);

        Ok(port)
    }

    // todo test!!!
    pub fn evolve(mut self, action: &action::Action) -> Portfolio {
        match action {
            action::Action::Buy(info) => {
                // buy actual share and re-calculate
                let pa =
                    self.actual.buy_share(&info.id, info.shares, &self.tickers);

                // re-calculate meta
                let meta = PortfolioMeta::new(&self.tickers, &self.goal, &pa);

                self.actual = pa;
                self.meta = meta;
                self
            }

            action::Action::Sell(info) => {
                // sell actual share and re-calculate
                let pa = self.actual.sell_share(
                    &info.id,
                    info.shares,
                    &self.tickers,
                );

                // re-calculate meta
                let meta = PortfolioMeta::new(&self.tickers, &self.goal, &pa);

                self.actual = pa;
                self.meta = meta;
                self
            }
        }
    }

    // todo test!!
    pub fn get_state(&self) -> api::PortfolioStateResp {
        let mut tickers: Vec<api::TickerResp> = self
            .goal
            .tickers_goal
            .iter()
            .map(|x| {
                let ticker = self.get_ticker(x.0);
                let tg = self.goal.get_ticker(x.0);
                let tm = self.meta.get_ticker(x.0);
                let ta = self.actual.get_ticker(x.0);

                api::TickerResp {
                    id: x.0.clone(),
                    symbol: ticker.symbol,
                    kind: ticker.kind.clone(),
                    fee: ticker.fee,
                    goal_percent: tg.goal_percent,
                    actual_percent: tm.ticker_percent,
                    actual_shares: ta.actual_shares,
                    actual_value: tm.ticker_value,
                    price: ticker.price,
                    order: tg.order,
                }
            })
            .collect();
        tickers.sort_by(|a, b| a.order.cmp(&b.order));
        api::PortfolioStateResp {
            name: self.name.clone(),
            tickers: tickers,
            goal_stock_percent: self.goal.goal_stock_percent,
            actual_stock_percent: self.meta.stock_percent,
            total_value: self.meta.total_value,
            deviation_percent: self.goal.deviation_percent,
        }
    }

    // todo test!!
    pub fn get_buy_next_action(&self) -> action::Action {
        /// filter based on portfolio action
        let filter_kind: Vec<&TickerMeta> = match self.meta.portfolio_action {
            meta::PortfolioAction::BuyStock => self
                .meta
                .tickers_meta
                .iter()
                .filter(|x| self.get_ticker(&x.0).is_stock())
                .map(|x| x.1)
                .collect(),

            meta::PortfolioAction::BuyBond => self
                .meta
                .tickers_meta
                .iter()
                .filter(|x| self.get_ticker(&x.0).is_bond())
                .map(|x| x.1)
                .collect(),

            meta::PortfolioAction::BuyEither => {
                self.meta.tickers_meta.values().collect()
            }
        };

        /// filter based on ticker action (buys)
        let contains_no_buys = filter_kind
            .iter()
            .filter(|x| matches!(&x.action, meta::TickerAction::Buy))
            .collect::<Vec<&&TickerMeta>>()
            .is_empty();

        // todo test
        let filter_buys: Vec<&TickerMeta> = if (contains_no_buys) {
            // dont filter since we dont have buys
            filter_kind
        } else {
            // filter buys
            filter_kind
                .into_iter()
                .filter(|x| matches!(x.action, meta::TickerAction::Buy))
                .collect()
        };

        /// filter based on price (cheapest)
        let empty_diff = EMPTY_TICKER_DIFF.clone();
        let tic_diff: &TickerMeta =
            filter_buys.iter().fold(&empty_diff, |x, y| {
                if (x.id == EMPTY_TICKER_DIFF.id) {
                    return y;
                } else if (y.id == EMPTY_TICKER_DIFF.id) {
                    return x;
                }
                let x_price = self.get_ticker(&x.id).price;
                let y_price = self.get_ticker(&y.id).price;

                if (x_price < y_price) {
                    x
                } else {
                    y
                }
            });

        action::Action::Buy(action::ActionInfo {
            id: tic_diff.id.clone(),
            shares: 1.0,
            price: self.get_ticker(&tic_diff.id).price,
        })
    }

    // todo test!!
    fn get_ticker(&self, id: &TickerId) -> Ticker {
        self.tickers
            .get(id)
            .expect(&format!("add ticker to db: {:?}", &id))
            .clone()
    }

    // todo test!!
    pub fn get_actual_tickers(&self) -> HashMap<TickerId, TickerActual> {
        self.actual.tickers_actual.clone()
    }
}
