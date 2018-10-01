use super::{actual::*, goal::*};
use crate::{std_ext::*, ticker::*};
use std::collections::HashMap;

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

lazy_static! {
    pub static ref EMPTY_TICKER_DIFF: TickerMeta = {
        TickerMeta {
            symbol: symbol!("EMPTY_TICKER_DIFF"),
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioMeta {
    pub tickers_meta: HashMap<TickerSymbol, TickerMeta>,
    pub total_value: f32,
    pub stock_value: f32,
    // todo maybe calculate lazily
    pub portfolio_action: PortfolioAction,
    pub stock_percent: f32,
}

impl PortfolioMeta {
    // todo test!!
    pub fn new(
        tickers: &HashMap<TickerSymbol, Ticker>,
        goal: &PortfolioGoal,
        actual: &PortfolioActual,
    ) -> Self {
        let mut meta = PortfolioMeta {
            tickers_meta: Self::populate_ticker_meta(&actual.tickers_actual),
            portfolio_action: PortfolioAction::BuyEither,
            total_value: 0.0,
            stock_value: 0.0,
            stock_percent: 0.0,
        };

        // calc ticker meta value
        // calc total value
        // calc stock value
        meta.calc_value(tickers, &actual);

        // calc ticker percent
        // calc stock percent
        meta.calc_percent();

        // calc action
        meta.calc_action(goal);

        meta
    }

    // todo test!!
    fn rounded_percent(val: f32, total_val: f32) -> f32 {
        let per = (val / total_val) * 100.0;
        (per * 100.00).round() / 100.00
    }

    // todo test
    fn populate_ticker_meta(
        tickers_actual: &HashMap<TickerSymbol, TickerActual>,
    ) -> HashMap<TickerSymbol, TickerMeta> {
        let mut map = HashMap::new();
        for (symb, _) in tickers_actual.iter() {
            map.insert(symb.clone(), TickerMeta::default(symb.clone()));
        }
        map
    }

    // todo test
    fn calc_value(
        &mut self,
        tickers: &HashMap<TickerSymbol, Ticker>,
        actual: &PortfolioActual,
    ) {
        for (symb, tic_meta) in self.tickers_meta.iter_mut() {
            let tic = tickers
                .get(&symb)
                .expect(&format!("add ticker to db: {:?}", &symb));
            let tic_act = actual.get_ticker(&symb);
            let tic_value = tic.price * tic_act.actual_shares;
            tic_meta.ticker_value = tic_value;

            // total and stock value
            self.total_value += tic_value;
            if (tic.is_stock()) {
                self.stock_value += tic_value;
            }
        }
    }

    // todo test
    fn calc_percent(&mut self) {
        self.stock_percent =
            Self::rounded_percent(self.stock_value, self.total_value);

        for (_, tic_meta) in self.tickers_meta.iter_mut() {
            tic_meta.ticker_percent =
                Self::rounded_percent(tic_meta.ticker_value, self.total_value);
        }
    }

    // todo test!!
    fn calc_action(&mut self, goal: &PortfolioGoal) {
        let diff = goal.goal_stock_percent - self.stock_percent;
        let deviation = goal.deviation_percent;
        self.portfolio_action = if ((diff < 0.0) && diff.abs() > deviation) {
            // If gS%-aS% is - and abs val above q% then buy bonds
            PortfolioAction::BuyBond
        } else if (diff > 0.0 && diff > deviation) {
            // If gS%-aS% is + and above q% then buy stocks
            PortfolioAction::BuyStock
        } else {
            // else buy stock or bond
            PortfolioAction::BuyEither
        };

        for (symb, tic_meta) in self.tickers_meta.iter_mut() {
            let goal_tic = goal.get_ticker(symb);
            tic_meta.calc_ticker_action(goal_tic.goal_percent, deviation);
        }
    }

    // todo test!!
    fn calc_total_val(&mut self) {
        let mut total_val = 0.0;
        for (s, x) in self.tickers_meta.iter() {
            total_val += x.ticker_value;
        }
        self.total_value = total_val;
    }

    pub fn get_ticker(&self, symbol: &TickerSymbol) -> &TickerMeta {
        self.tickers_meta
            .get(&symbol)
            .expect(&format!("add ticker to db: {:?}", &symbol))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerMeta {
    pub symbol: TickerSymbol,
    pub action: TickerAction,
    pub ticker_value: f32,
    pub ticker_percent: f32,
}

impl TickerMeta {
    fn default(symbol: TickerSymbol) -> TickerMeta {
        TickerMeta {
            symbol: symbol,
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        }
    }

    // todo test!!
    fn calc_ticker_action(
        &mut self,
        goal_percent: f32,
        deviation_percent: f32,
    ) {
        let g_minus_a: f32 = goal_percent - self.ticker_percent;
        let action = if (g_minus_a < 0.0 && g_minus_a.abs() > deviation_percent)
        {
            TickerAction::Sell
        } else if (g_minus_a > 0.0 && g_minus_a.abs() > deviation_percent) {
            TickerAction::Buy
        } else {
            TickerAction::Hold
        };
        self.action = action;
    }
}
