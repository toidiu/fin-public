use crate::ticker::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PortfolioGoal {
    pub tickers_goal: HashMap<TickerSymbol, TickerGoal>,
    pub goal_stock_percent: f32,
    pub deviation_percent: f32,
}

impl PortfolioGoal {
    pub fn get_ticker(&self, symbol: &TickerSymbol) -> &TickerGoal {
        self.tickers_goal
            .get(symbol)
            .expect(&format!("add ticker to db: {:?}", symbol))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerGoal {
    pub symbol: TickerSymbol,
    pub goal_percent: f32,
    pub order: u32,
}
