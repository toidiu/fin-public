use crate::ticker::*;
use std::ops::Add;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "action")]
pub enum Action {
    Buy(ActionInfo),
    Sell(ActionInfo),
}

impl Action {
    pub fn get_id(&self) -> TickerId {
        match self {
            Action::Buy(ab) => ab.id.clone(),
            Action::Sell(ab) => ab.id.clone(),
        }
    }

    pub fn get_price(&self) -> f64 {
        match self {
            Action::Buy(ab) => ab.price,
            Action::Sell(ab) => ab.price,
        }
    }
}

impl Add for Action {
    type Output = Action;

    fn add(self, other: Action) -> Action {
        match (&self, &other) {
            (Action::Buy(a), Action::Buy(b)) => Action::Buy(ActionInfo {
                id: self.get_id(),
                shares: a.shares + b.shares,
                price: self.get_price(),
            }),
            // FIXME!!!!!!
            (Action::Buy(a), Action::Sell(b)) => Action::Buy(ActionInfo {
                id: self.get_id(),
                shares: b.shares - a.shares,
                price: self.get_price(),
            }),
            // FIXME!!!!!!
            (Action::Sell(a), Action::Buy(b)) => Action::Buy(ActionInfo {
                id: self.get_id(),
                shares: a.shares - b.shares,
                price: self.get_price(),
            }),
            (Action::Sell(a), Action::Sell(b)) => Action::Sell(ActionInfo {
                id: self.get_id(),
                shares: a.shares + b.shares,
                price: self.get_price(),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionInfo {
    pub id: TickerId,
    pub shares: f64,
    pub price: f64,
}
