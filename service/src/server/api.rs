use crate::algo::{Action, BuyNext};
use crate::data;
use crate::portfolio;
use crate::ticker::{InvestmentKind, TickerId, TickerSymbol};
use chrono::prelude::*;
use std::collections::HashMap;

// ============ PortfolioState
#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioStateResp {
    pub name: String,
    pub goal_id: i64,
    pub tickers: Vec<TickerResp>,
    pub goal_stock_percent: f32,
    pub actual_stock_percent: f32,
    pub total_value: f64,
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
    pub actual_value: f64,
    pub actual_shares: f64,
    pub price: f64,
    pub order: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioStateQuery {
    pub goal_id: i64,
}

// ============ BuyNext
#[derive(Serialize, Debug)]
pub struct BuyNextResp {
    pub requested_value: f64,
    pub actions: Vec<Action>,
    pub buy_value: f64,
    pub action_summary: HashMap<TickerId, Action>,
}

impl BuyNextResp {
    pub fn from_data(buy_next: BuyNext, requested_value: f64) -> Self {
        let mut map: HashMap<TickerId, Action> = HashMap::new();
        for action in buy_next.actions.iter() {
            let id = action.get_id();
            match map.get(&id) {
                Some(exist) => {
                    let action = action.clone() + exist.clone();
                    map.insert(id, action);
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
    pub goal_port_id: i64,
    pub actual_port_id: i64,
    pub amount: f64,
}

#[derive(Deserialize)]
pub struct BuyNextData {
    pub goal_id: i64,
    pub port_a_id: i64,
    pub actions: Vec<Action>,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewPortActualData {
    #[serde(alias = "goalPortId")]
    pub goal_id: i64,
    #[serde(alias = "stockPercent")]
    pub stock_percent: f32,
}

// ============ Portfolio Detail
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PortfolioGoalDetailResp {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub tickers_goal: HashMap<TickerId, portfolio::TickerGoalDetailed>,
}

impl PortfolioGoalDetailResp {
    pub fn new(
        data: data::GoalPortData,
        tickers_goal: HashMap<TickerId, portfolio::TickerGoalDetailed>,
    ) -> Self {
        PortfolioGoalDetailResp {
            id: data.id,
            name: data.name,
            description: data.description,
            tickers_goal: tickers_goal,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioActualResp {
    pub id: i64,
    pub fk_user_id: i64,
    pub fk_port_g_id: i64,
    pub stock_percent: f32,
    pub deviation: f32,
    pub version: i32,
    pub last_updated: DateTime<Utc>,
    pub tickers_actual: Vec<portfolio::TickerActual>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioActualDetailResp {
    pub id: i64,
    pub fk_user_id: i64,
    pub fk_port_g_id: i64,
    pub stock_percent: f32,
    pub deviation: f32,
    pub version: i32,
    pub last_updated: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
}
