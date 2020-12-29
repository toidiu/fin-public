use crate::algo;

mod actual;
mod goal;
mod meta;
mod state;
mod test_helper_meta;
mod ticker;

pub use self::{
    actual::{PortfolioActual, TickerActual},
    goal::{GoalTicker, PortfolioGoal, TickerGoalDetailed},
    meta::{PortfolioAction, TickerAction, TickerMeta, EMPTY_TICKER_META},
    state::PortfolioState,
    ticker::{InvestmentKind, Ticker, TickerId, TickerSymbol},
};

pub const SMALL_PERCENT_DEVIATION: f32 = 0.2_f32;
pub const PERCENT_DEVIATION: f32 = 1.5_f32;
