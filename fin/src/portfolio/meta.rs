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
    pub tickers_diff: Vec<TickerDiff>,
    // calculated
    pub stock_diff: f32,
    // calculated
    pub portfolio_action: PortfolioAction,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerDiff {
    pub symbol: TickerSymbol,
    pub goal_minus_actual: f32,
    pub action: TickerAction,
    // used to display the tickers in deterministic order each time
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
