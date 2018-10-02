use crate::ticker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Buy(ActionBuy),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionBuy {
    pub symbol: TickerSymbol,
    pub shares: f32,
}
