#![allow(dead_code, unused)]

use crate::data;
use crate::ticker::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::num;

mod actual;
mod goal;
mod meta;

use crate::portfolio::meta::{PortfolioAction, TickerAction};

// export this as portfolio::{...}
pub use crate::portfolio::actual::{PortfolioActual, TickerActual};
pub use crate::portfolio::goal::{PortfolioGoal, TickerGoal};
pub use crate::portfolio::meta::{PortfolioMeta, TickerDiff};

lazy_static! {
    static ref EMPTY_TICKER_DIFF: TickerDiff = {
        TickerDiff {
            symbol: TickerSymbol("EMPTY_TICKER_DIFF".to_string()),
            goal_minus_actual: 0.0,
            action: TickerAction::Hold,
            order: 0,
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    name: String,
    goal: PortfolioGoal,
    actual: PortfolioActual,
    meta: PortfolioMeta,
    tickers: HashMap<TickerSymbol, Ticker>,
}

impl Portfolio {
    pub fn new<T: data::TickerDatabase>(db: &T) -> Portfolio {
        // get goal
        let pg = PortfolioGoal {
            tickers: db.get_goal(),
            goal_stock_percent: 58.0,
            deviation_percent: 5.0,
        };

        // get tickers map
        // TODO eventually only request those we care about (ones in goal)
        let tickers_map: HashMap<TickerSymbol, Ticker> = db.get_tickers();

        // get actual
        let pa = PortfolioActual::new(db.get_actual(), &tickers_map);

        // get meta
        let meta = PortfolioMeta::default();

        let mut port = Portfolio {
            name: "my portfolio".to_owned(),
            goal: pg,
            actual: pa,
            meta: meta,
            tickers: tickers_map,
        };

        port.init()
    }

    // calculate that stock % is met
    fn init(mut self) -> Self {
        // TODO maybe meta init should be in PortfolioMeta impl
        self.calc_stock_diff().calc_ticker_diff()
    }

    pub fn get_buy_next(&self) -> Ticker {
        let filter_kind: Vec<&TickerDiff> = match self.meta.portfolio_action {
            PortfolioAction::BuyStock => self
                .meta
                .tickers_diff
                .iter()
                .filter(|x| {
                    self.tickers
                        .get(&x.symbol)
                        .expect(&format!("add ticker to db: {:?}", &x.symbol))
                        .is_stock()
                }).collect(),

            PortfolioAction::BuyBond => self
                .meta
                .tickers_diff
                .iter()
                .filter(|x| {
                    self.tickers
                        .get(&x.symbol)
                        .expect(&format!("add ticker to db: {:?}", &x.symbol))
                        .is_bond()
                }).collect(),

            PortfolioAction::BuyEither => self.meta.tickers_diff.iter().collect(),
        };

        // fixme broken logic
        let filter_buys: Vec<&TickerDiff> = if (!filter_kind.is_empty()) {
            // filter buys
            filter_kind
                .into_iter()
                .filter(|x| matches!(x.action, TickerAction::Buy))
                .collect()
        } else {
            // dont filter
            filter_kind
        };

        // filter cheapest
        let empty_diff = EMPTY_TICKER_DIFF.clone();

        // fixme maybe user scan
        let tic_diff: &TickerDiff = filter_buys.iter().fold(&empty_diff, |x, y| {
            if (x.symbol == EMPTY_TICKER_DIFF.symbol) {
                return y;
            } else if (y.symbol == EMPTY_TICKER_DIFF.symbol) {
                return x;
            }
            let x_price = self
                .tickers
                .get(&x.symbol)
                .expect(&format!("add ticker to db: {:?}", &x.symbol))
                .price;
            let y_price = self
                .tickers
                .get(&y.symbol)
                .expect(&format!("add ticker to db: {:?}", &y.symbol))
                .price;

            if (x_price < y_price) {
                x
            } else {
                y
            }
        });

        self.get_ticker(&tic_diff.symbol)
    }

    fn get_ticker(&self, symbol: &TickerSymbol) -> Ticker {
        self.tickers
            .get(symbol)
            .expect(&format!("add ticker to db: {:?}", &symbol))
            .clone()
    }

    // calculate stock difference and action
    fn calc_stock_diff(mut self) -> Self {
        let actual_per = self.actual.get_stock_percent();
        let goal_per = self.goal.goal_stock_percent;
        let deviation = self.goal.deviation_percent;

        let diff = goal_per - actual_per;
        self.meta.portfolio_action = if ((diff < 0.0) && diff.abs() > deviation) {
            // If gS%-aS% is - and abs val above q% then buy bonds
            PortfolioAction::BuyBond
        } else if (diff > 0.0 && diff > deviation) {
            // If gS%-aS% is + and above q% then buy stocks
            PortfolioAction::BuyStock
        } else {
            // else buy stock or bond
            PortfolioAction::BuyEither
        };
        self.meta.stock_diff = diff;
        self
    }

    // calculate gTn%-aTn% for each ticker
    fn calc_ticker_diff(mut self) -> Self {
        let mut v: Vec<TickerDiff> = self
            .actual
            .tickers
            .iter()
            .map(|symb_tic_actual| {
                let goal_tic = self
                    .goal
                    .tickers
                    .get(symb_tic_actual.0)
                    .expect(&format!("add ticker to db: {:?}", symb_tic_actual.0));
                TickerDiff::new(symb_tic_actual.1, goal_tic, self.goal.deviation_percent)
            }).collect();
        v.sort_by(|a, b| a.order.cmp(&b.order));
        self.meta.tickers_diff = v;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Helper {}
    impl Helper {}

}
