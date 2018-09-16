use crate::ticker::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PortfolioGoal {
    pub tickers: HashMap<TickerSymbol, TickerGoal>,
    pub goal_stock_percent: f32,
    pub deviation_percent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerGoal {
    pub symbol: TickerSymbol,
    pub goal_percent: f32,
    pub order: u32,
}
