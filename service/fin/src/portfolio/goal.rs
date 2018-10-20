use crate::ticker::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PortfolioGoal {
    pub id: i64,
    pub tickers_goal: HashMap<TickerId, TickerGoal>,
    pub goal_stock_percent: f32,
    pub deviation_percent: f32,
    pub name: String,
    pub description: Option<String>,
}

impl PortfolioGoal {
    pub fn get_ticker(&self, id: &TickerId) -> &TickerGoal {
        self.tickers_goal
            .get(id)
            .expect(&format!("add ticker to db: {:?}", id))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerGoal {
    pub port_goal_id: i64,
    pub ticker_id: i64,
    pub goal_percent: f32,
    pub order: i32,
}
