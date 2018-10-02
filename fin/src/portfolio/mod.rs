#![allow(dead_code, unused)]

use self::meta::EMPTY_TICKER_DIFF;
pub use self::{
    action::{Action, ActionBuy},
    actual::{PortfolioActual, TickerActual},
    goal::{PortfolioGoal, TickerGoal},
    meta::{PortfolioMeta, TickerMeta},
};
use crate::{api, data, std_ext::*, ticker::*};
use std::{cmp::Ordering, collections::HashMap, num};

mod action;
mod actual;
mod goal;
mod meta;

#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    name: String,
    goal: PortfolioGoal,
    actual: PortfolioActual,
    meta: PortfolioMeta,
    tickers: HashMap<TickerSymbol, Ticker>,
}

impl Portfolio {
    // todo test!!
    pub fn new<T: data::TickerDatabase>(db: &T) -> Portfolio {
        // get goal
        let pg = PortfolioGoal {
            tickers_goal: db.get_goal(),
            goal_stock_percent: 58.0,
            deviation_percent: 5.0,
        };

        // get tickers map
        // todo eventually only request those we care about (ones in goal)
        let tickers_map: HashMap<TickerSymbol, Ticker> = db.get_tickers();

        // get actual
        let pa = PortfolioActual::new(db.get_actual(), &tickers_map);

        // get meta
        let meta = PortfolioMeta::new(&tickers_map, &pg, &pa);

        Portfolio {
            name: "my portfolio".to_owned(),
            goal: pg,
            actual: pa,
            meta: meta,
            tickers: tickers_map,
        }
    }

    // todo test!!!
    pub fn evolve(&self, action: action::Action) -> Portfolio {
        let port = match action {
            action::Action::Buy(buy) => {
                // buy actual share and re-calculate
                let pa = self.actual.buy_share(
                    &buy.symbol,
                    buy.shares,
                    &self.tickers,
                );

                // re-calculate meta
                let meta = PortfolioMeta::new(&self.tickers, &self.goal, &pa);

                // clone goal and tickers
                let pg = self.goal.clone();
                let tickers_map = self.tickers.clone();

                // return new Portfolio
                Portfolio {
                    name: "my portfolio".to_owned(),
                    goal: pg,
                    actual: pa,
                    meta: meta,
                    tickers: tickers_map,
                }
            }
        };

        port
    }

    // todo test!!
    pub fn get_state(&self) -> api::PortfolioState {
        let mut tickers: Vec<api::TickerState> = self
            .goal
            .tickers_goal
            .iter()
            .map(|x| {
                let ticker = self.get_ticker(x.0);
                let tg = self.goal.get_ticker(x.0);
                let tm = self.meta.get_ticker(x.0);

                api::TickerState {
                    symbol: x.0.clone(),
                    kind: ticker.kind.clone(),
                    fee: ticker.fee,
                    goal_percent: tg.goal_percent,
                    actual_percent: tm.ticker_percent,
                    actual_value: tm.ticker_value,
                    price: ticker.price,
                    order: tg.order,
                }
            }).collect();
        tickers.sort_by(|a, b| a.order.cmp(&b.order));
        api::PortfolioState {
            tickers: tickers,
            goal_stock_percent: self.goal.goal_stock_percent,
            actual_stock_percent: self.meta.stock_percent,
            total_value: self.meta.total_value,
            deviation_percent: self.goal.deviation_percent,
        }
    }

    // todo test!!
    pub fn get_buy_next(&self) -> action::Action {
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
                if (x.symbol == EMPTY_TICKER_DIFF.symbol) {
                    return y;
                } else if (y.symbol == EMPTY_TICKER_DIFF.symbol) {
                    return x;
                }
                let x_price = self.get_ticker(&x.symbol).price;
                let y_price = self.get_ticker(&y.symbol).price;

                if (x_price < y_price) {
                    x
                } else {
                    y
                }
            });

        action::Action::Buy(action::ActionBuy {
            symbol: tic_diff.symbol.clone(),
            shares: 1.0,
        })
    }

    // todo test!!
    fn get_ticker(&self, symbol: &TickerSymbol) -> Ticker {
        self.tickers
            .get(symbol)
            .expect(&format!("add ticker to db: {:?}", &symbol))
            .clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Helper {}
    impl Helper {}

}
