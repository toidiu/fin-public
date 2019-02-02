use crate::data;
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
    pub actual_shares: f32,
    pub price: f32,
    pub order: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioStateQuery {
    pub goal_id: i64,
}

// ============ BuyNext
#[derive(Serialize, Debug)]
pub struct BuyNextResp {
    pub requested_value: f32,
    pub actions: Vec<portfolio::Action>,
    pub buy_value: f32,
    pub action_summary: HashMap<TickerId, portfolio::Action>,
}

impl BuyNextResp {
    pub fn from_data(
        buy_next: portfolio::BuyNext,
        requested_value: f32,
    ) -> Self {
        let mut map: HashMap<TickerId, portfolio::Action> = HashMap::new();
        for action in buy_next.actions.iter() {
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
        BuyNextResp {
            requested_value: requested_value,
            actions: buy_next.actions,
            buy_value: buy_next.buy_value,
            action_summary: map,
        }
    }
}

#[derive(Deserialize)]
pub struct BuyNextQuery {
    pub goal_id: i64,
    pub amount: f32,
}

#[derive(Deserialize)]
pub struct BuyNextData {
    pub goal_id: i64,
    pub actions: Vec<portfolio::Action>,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

// ============ Portfolio Detail
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PortfolioGoalDetailResp {
    pub id: i64,
    pub tickers_goal: HashMap<TickerId, portfolio::TickerGoalDetailed>,
    pub goal_stock_percent: f32,
    pub deviation_percent: f32,
    pub name: String,
    pub description: Option<String>,
}

impl PortfolioGoalDetailResp {
    pub fn new(
        data: data::PortGoalData,
        tickers_goal: HashMap<TickerId, portfolio::TickerGoalDetailed>,
    ) -> Self {
        PortfolioGoalDetailResp {
            id: data.id,
            tickers_goal: tickers_goal,
            goal_stock_percent: data.stock_per,
            deviation_percent: data.deviation,
            name: data.name,
            description: data.description,
        }
    }
}
