use super::Portfolio;
use super::{action::*, actual::*, ticker::*};
use crate::api;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BuyNext {
    pub init_state: Portfolio,
    pub evolved_actual: HashMap<TickerId, TickerActual>,
    pub actions: Vec<Action>,
    pub buy_value: f32,
    pub action_summary: HashMap<TickerId, Action>,
}

impl BuyNext {
    pub fn new(port: Portfolio) -> Self {
        let actual_tickers = port.get_actual_tickers();
        BuyNext {
            init_state: port,
            evolved_actual: actual_tickers,
            actions: Vec::new(),
            buy_value: 0.0,
            action_summary: HashMap::new(),
        }
    }
}
