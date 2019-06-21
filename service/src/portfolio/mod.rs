use crate::algo;
use crate::ticker;

mod actual;
mod goal;
mod meta;
mod state;
mod test_helper;

pub use self::{
    actual::{PortfolioActual, TickerActual},
    goal::{GoalTicker, PortfolioGoal, TickerGoalDetailed},
    meta::{PortfolioAction, TickerAction, TickerMeta, EMPTY_TICKER_META},
    state::PortfolioState,
};

pub const SMALL_PERCENT_DEVIATION: f32 = 0.2_f32;
pub const PERCENT_DEVIATION: f32 = 1.5_f32;
