#![allow(dead_code, unused)]

use crate::data;
use crate::ticker::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::num;

mod actual;
mod goal;

// export this as portfolio::TickerGoal
pub use crate::portfolio::actual::{PortfolioActual, TickerActual};
pub use crate::portfolio::goal::{PortfolioGoal, TickerGoal};

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PortfolioAction {
    BuyStock,
    BuyBond,
    BuyEither,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TickerAction {
    Buy,
    Sell,
    Hold,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct TickerDiff {
    symbol: TickerSymbol,
    goal_minus_actual: f32,
    action: TickerAction,
    // used to display the tickers in deterministic order each time
    order: u32,
}

impl TickerDiff {
    pub fn new(
        actual_tic: &TickerActual,
        goal_tic: &TickerGoal,
        deviation_percent: f32,
    ) -> TickerDiff {
        let g_minus_a = goal_tic.goal_percent - actual_tic.get_actual_percent();
        TickerDiff {
            symbol: actual_tic.symbol.clone(),
            goal_minus_actual: g_minus_a,
            action: {
                if (g_minus_a < 0.0 && g_minus_a.abs() > deviation_percent) {
                    TickerAction::Sell
                } else if (g_minus_a > 0.0 && g_minus_a.abs() > deviation_percent) {
                    TickerAction::Buy
                } else {
                    TickerAction::Hold
                }
            },
            order: goal_tic.order,
        }
    }
}

// =================================

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

#[derive(Serialize, Deserialize, Debug)]
struct PortfolioMeta {
    // calculated
    tickers_diff: Vec<TickerDiff>,
    // calculated
    stock_diff: f32,
    // calculated
    portfolio_action: PortfolioAction,
}

impl PortfolioMeta {
    pub fn default() -> Self {
        PortfolioMeta {
            tickers_diff: vec![],
            stock_diff: 0.0,
            portfolio_action: PortfolioAction::BuyEither,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Helper {}
    impl Helper {}

}
