use super::actual::*;
use super::goal::*;
use crate::ticker::*;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioMeta {
    // calculated
    pub tickers_diff: HashMap<TickerSymbol, TickerDiff>,
    // calculated
    pub stock_diff: f32,
    // calculated
    pub portfolio_action: PortfolioAction,
}

impl PortfolioMeta {
    pub fn new(goal: &PortfolioGoal, actual: &PortfolioActual) -> Self {
        let mut meta = PortfolioMeta {
            tickers_diff: HashMap::new(),
            stock_diff: 0.0,
            portfolio_action: PortfolioAction::BuyEither,
        };
        meta.calc_stock_diff(goal, actual);
        meta.calc_ticker_diff(goal, actual);
        meta
    }

    // calculate stock difference and action
    fn calc_stock_diff(&mut self, goal: &PortfolioGoal, actual: &PortfolioActual) {
        let actual_per = actual.get_stock_percent();
        let goal_per = goal.goal_stock_percent;
        let deviation = goal.deviation_percent;

        let diff = goal_per - actual_per;
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
        self.stock_diff = diff;
    }

    // calculate gTn%-aTn% for each ticker
    fn calc_ticker_diff(&mut self, goal: &PortfolioGoal, actual: &PortfolioActual) {
        let mut v: Vec<TickerDiff> = actual
            .tickers_actual
            .iter()
            .map(|symb_tic_actual| {
                let goal_tic = goal
                    .tickers_goal
                    .get(symb_tic_actual.0)
                    .expect(&format!("add ticker to db: {:?}", symb_tic_actual.0));
                TickerDiff::new(symb_tic_actual.1, goal_tic, goal.deviation_percent)
            }).collect();
        // create a map
        let mut map: HashMap<TickerSymbol, TickerDiff> = HashMap::new();
        for x in v {
            map.insert(x.symbol.clone(), x);
        }
        self.tickers_diff = map;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerDiff {
    pub symbol: TickerSymbol,
    pub goal_minus_actual: f32,
    pub action: TickerAction,
    // used to display the tickers in deterministic order each time
    // fixme remove
    pub order: u32,
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
