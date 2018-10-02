use crate::ticker::*;

pub enum Action {
    Buy(ActionBuy),
}
pub struct ActionBuy {
    pub symbol: TickerSymbol,
    pub shares: f32,
}
