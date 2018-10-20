use crate::ticker::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    Buy(ActionInfo),
    Sell(ActionInfo),
}

impl Action {
    pub fn get_symbol(&self) -> TickerId {
        match self {
            Action::Buy(ab) => ab.id.clone(),
            Action::Sell(ab) => ab.id.clone(),
        }
    }

    pub fn get_price(&self) -> f32 {
        match self {
            Action::Buy(ab) => ab.price,
            Action::Sell(ab) => ab.price,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionInfo {
    pub id: TickerId,
    pub shares: f32,
    pub price: f32,
}
