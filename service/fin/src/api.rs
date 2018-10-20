use crate::portfolio;
use crate::portfolio::TickerActual;
use crate::ticker::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioState {
    pub tickers: Vec<TickerState>,
    pub goal_stock_percent: f32,
    pub actual_stock_percent: f32,
    pub total_value: f32,
    pub deviation_percent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerState {
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

// =================================== STATE
#[derive(Serialize, Deserialize, Debug)]
pub struct EvolvedState {
    pub init_state: portfolio::Portfolio,
    pub evolved_actual: HashMap<TickerId, TickerActual>,
    pub actions: Vec<portfolio::Action>,
    pub buy_value: f32,
}

impl EvolvedState {
    pub fn new(port: portfolio::Portfolio) -> Self {
        let actual_tickers = port.get_actual_tickers();
        EvolvedState {
            init_state: port,
            evolved_actual: actual_tickers,
            actions: Vec::new(),
            buy_value: 0.0,
        }
    }
}

#[derive(FromForm)]
pub struct AmountQuery {
    pub amount: f32,
}
