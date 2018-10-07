#![allow(dead_code, unused)]

mod pg_data;
mod static_data;

pub use self::{pg_data::PgTickerDatabase, static_data::DefaultTickerDatabase};

use crate::{portfolio, ticker::*};
use std::collections::HashMap;

pub trait TickerDatabase {
    fn get_tickers(&self) -> HashMap<TickerSymbol, Ticker>;
    fn get_goal(&self) -> HashMap<TickerSymbol, portfolio::TickerGoal>;
    fn get_actual(&self) -> HashMap<TickerSymbol, portfolio::TickerActual>;
}
