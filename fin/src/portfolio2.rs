#![allow(dead_code, unused)]

use crate::data;
use crate::ticker::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::num;

lazy_static! {
    pub static ref EMPTY_TICKER_DIFF: TickerDiff = {
        TickerDiff {
            symbol: TickerSymbol("".to_string()),
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
pub struct TickerDiff {
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
        let g_minus_a = goal_tic.goal_percent - actual_tic.actual_percent;
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

    pub fn empty() -> TickerDiff {
        TickerDiff {
            symbol: TickerSymbol("".to_owned()),
            goal_minus_actual: 0.0,
            action: TickerAction::Hold,
            order: 0,
        }
    }
}

// =================================

#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    name: String,
    #[serde(skip)]
    goal: PortfolioGoal,
    #[serde(skip)]
    actual: PortfolioActual,
    meta: PortfolioMeta,
    #[serde(skip)]
    tickers: HashMap<TickerSymbol, Ticker>,
}

impl Portfolio {
    pub fn get_data<T: data::TickerDatabase>(db: &T) -> Portfolio {
        let mut tickers_map = HashMap::new();
        for x in db.get_tickers() {
            tickers_map.insert(x.symbol.clone(), x);
        }

        let pg = PortfolioGoal {
            tickers: db.get_goal(),
            goal_stock_percent: 58.0,
            deviation_percent: 5.0,
        };

        let actual_tickers = {
            let mut actual = db.get_actual();
            let total_value = actual.iter().map(|x| x.1.actual_value).sum();
            actual
                .into_iter()
                .map(|x| x.1.update_actual_percent(total_value))
                .collect()
        };

        let pa = PortfolioActual::new(actual_tickers, &tickers_map);
        let meta = PortfolioMeta {
            tickers_diff: vec![],
            stock_diff: 0.0,
            portfolio_action: PortfolioAction::BuyEither,
        };

        Portfolio {
            name: "my portfolio".to_owned(),
            goal: pg,
            actual: pa,
            meta: meta,
            // eventually only request those we care about (ones in goal)
            tickers: tickers_map,
        }
    }

    // calculate that stock % is met
    pub fn update_portfolio(&mut self) {
        self.calc_stock_diff();
        self.calc_ticker_diff();
    }

    pub fn filter_based_on_stock_action(&self) -> TickerDiff {
        let filter_kind: Vec<&TickerDiff> = match self.meta.portfolio_action {
            PortfolioAction::BuyStock => self
                .meta
                .tickers_diff
                .iter()
                .filter(|x| {
                    self.tickers
                        .get(&x.symbol)
                        .expect(&format!("add ticker to db: {:?}", &x.symbol))
                        .kind
                        == InvestmentKind::Stock
                }).collect(),

            PortfolioAction::BuyBond => self
                .meta
                .tickers_diff
                .iter()
                .filter(|x| {
                    self.tickers
                        .get(&x.symbol)
                        .expect(&format!("add ticker to db: {:?}", &x.symbol))
                        .kind
                        == InvestmentKind::Bond
                }).collect(),

            PortfolioAction::BuyEither => self.meta.tickers_diff.iter().collect(),
        };

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

        let tic_diff: &TickerDiff = filter_buys.iter().fold(&empty_diff, |x, y| {
            if (x.symbol == EMPTY_TICKER_DIFF.symbol) {
                return y;
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

        tic_diff.clone()
    }

    // calculate stock difference and action
    fn calc_stock_diff(&mut self) {
        let actual_per = self.actual.actual_stock_percent;
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
    }

    // calculate gTn%-aTn% for each ticker
    fn calc_ticker_diff(&mut self) {
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

#[derive(Serialize, Deserialize, Default, Debug)]
struct PortfolioGoal {
    tickers: HashMap<TickerSymbol, TickerGoal>,
    goal_stock_percent: f32,
    deviation_percent: f32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct PortfolioActual {
    tickers: HashMap<TickerSymbol, TickerActual>,
    // calculated
    total_value: f32,
    // calculated
    actual_stock_percent: f32,
}

impl PortfolioActual {
    fn new(tickers_actual: Vec<TickerActual>, tickers: &HashMap<TickerSymbol, Ticker>) -> Self {
        let mut map = HashMap::new();
        for x in tickers_actual {
            map.insert(x.symbol.clone(), x);
        }
        PortfolioActual {
            tickers: map,
            total_value: 0.0,
            actual_stock_percent: 0.0,
        }.calculate_total()
        .calculate_stock_percent(tickers)
    }
    fn calculate_total(mut self) -> Self {
        self.total_value = self.tickers.iter().map(|x| x.1.actual_value).sum();
        self
    }
    fn calculate_stock_percent(mut self, tickers: &HashMap<TickerSymbol, Ticker>) -> Self {
        let stock_value: f32 = self
            .tickers
            .iter()
            .filter(|ref x| tickers.get(&x.1.symbol).unwrap().is_stock())
            .map(|x| x.1.actual_value)
            .sum();
        self.actual_stock_percent = (stock_value / self.total_value) * 100.0;
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerGoal {
    pub symbol: TickerSymbol,
    pub goal_percent: f32,
    pub order: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerActual {
    pub symbol: TickerSymbol,
    pub actual_value: f32,
    pub actual_shares: u32,
    // calculated
    pub actual_percent: f32,
}

impl TickerActual {
    pub fn update_actual_percent(mut self, total_value: f32) -> Self {
        self.actual_percent = (self.actual_value / total_value) * 100.0;
        self
    }
}
