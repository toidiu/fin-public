#![allow(dead_code, unused)]

use crate::algo;
use crate::ticker::{self, *};

mod actual;
mod goal;
mod meta;
mod state;
mod test_helper;

pub use self::{
    actual::{PortfolioActual, TickerActual},
    goal::{GoalTicker, PortfolioGoal, TickerGoalDetailed},
    meta::*,
    state::PortfolioState,
};

pub const SMALL_PERCENT_DEVIATION: f32 = 0.2_f32;
pub const PERCENT_DEVIATION: f32 = 1.5_f32;
