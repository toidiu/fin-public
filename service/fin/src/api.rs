use crate::portfolio::{
    self, InvestmentKind, Ticker, TickerActual, TickerId, TickerSymbol,
};
use std::collections::HashMap;

// ============ PortfolioState
#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioStateResp {
    pub name: String,
    pub tickers: Vec<TickerResp>,
    pub goal_stock_percent: f32,
    pub actual_stock_percent: f32,
    pub total_value: f32,
    pub deviation_percent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerResp {
    pub id: TickerId,
    pub symbol: TickerSymbol,
    pub kind: InvestmentKind,
    pub fee: f32,
    pub goal_percent: f32,
    pub actual_percent: f32,
    pub actual_value: f32,
    pub price: f32,
    pub order: i32,
}

#[derive(FromForm)]
pub struct PortfolioStateQuery {
    pub user_id: i64,
    pub goal_id: i64,
}

// ============ BuyNext
#[derive(Serialize, Debug)]
pub struct BuyNextResp {
    #[serde(skip_serializing)]
    pub init_state: portfolio::Portfolio,
    #[serde(skip_serializing)]
    pub evolved_actual: HashMap<TickerId, TickerActual>,
    pub actions: Vec<portfolio::Action>,
    pub buy_value: f32,
    pub action_summary: HashMap<TickerId, portfolio::Action>,
}

impl BuyNextResp {
    pub fn new(port: portfolio::Portfolio) -> Self {
        let actual_tickers = port.get_actual_tickers();
        BuyNextResp {
            init_state: port,
            evolved_actual: actual_tickers,
            actions: Vec::new(),
            buy_value: 0.0,
            action_summary: HashMap::new(),
        }
    }

    pub fn generate_summary(mut self) -> Self {
        let mut map: HashMap<TickerId, portfolio::Action> = HashMap::new();
        for action in self.actions.iter() {
            let id = action.get_id();
            match map.get(&id) {
                Some(exist) => {
                    map.insert(id, action.clone() + exist.clone());
                }

                None => {
                    map.insert(id, action.clone());
                }
            };
        }
        self.action_summary = map;
        self
    }
}

#[derive(FromForm)]
pub struct BuyNextQuery {
    pub user_id: i64,
    pub goal_id: i64,
    pub amount: f32,
}

#[derive(Deserialize)]
pub struct BuyNextForm {
    pub user_id: i64,
    pub goal_id: i64,
    pub amount: f32,
}
