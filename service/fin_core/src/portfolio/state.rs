use super::actual::{self, *};
use super::algo::{self, *};
use super::goal::{self, *};
use super::meta::{self, *};
use crate::algo::{Action, ActionInfo};
use crate::portfolio::ticker::{self, *};

use crate::std_ext::*;
use chrono::prelude::*;
use std::{cmp::Ordering, collections::HashMap, num};

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioState {
    pub goal: PortfolioGoal,
    pub actual: PortfolioActual,
    pub meta: PortfolioMeta,
    tickers: HashMap<TickerId, Ticker>,
}

// **************************************
// mutate
// **************************************
impl PortfolioState {
    /// Modify actual and then re-calculate meta.
    pub fn apply_action(&mut self, action: &Action) {
        match action {
            Action::Buy(info) => {
                // buy actual share
                self.actual.buy_share(&info.id, info.shares);
            }

            Action::Sell(info) => {
                // sell actual share
                self.actual.sell_share(&info.id, info.shares);
            }
        }
        // re-calculate meta
        self.meta
            .recalculate(&self.tickers, &self.actual, &self.goal);
    }
}

// **************************************
// immutable
// **************************************
impl PortfolioState {
    pub fn new(
        pa: PortfolioActual,
        pg: PortfolioGoal,
        tickers_map: HashMap<TickerId, Ticker>,
    ) -> PortfolioState {
        // get meta
        let meta = PortfolioMeta::new(&tickers_map, &pg, &pa);
        PortfolioState {
            goal: pg,
            actual: pa,
            meta,
            tickers: tickers_map,
        }
    }

    // todo test!!
    pub fn get_ticker(&self, id: &TickerId) -> &Ticker {
        &self.tickers.get(id).expect(&format!(
            "{} add ticker to db: {:?}",
            line!(),
            &id
        ))
    }

    // todo test!!
    pub fn get_actual_port(&self) -> &PortfolioActual {
        &self.actual
    }

    // todo test!!
    pub fn get_actual_tickers(&self) -> &HashMap<TickerId, TickerActual> {
        &self.actual.tickers_actual
    }

    // todo test!!
    pub(crate) fn get_goal_ticker(&self, id: &TickerId) -> &GoalTicker {
        &self.goal.get_ticker_g(id)
    }

    // todo test!!
    pub(crate) fn get_meta_tickers(&self) -> &HashMap<TickerId, TickerMeta> {
        &self.meta.tickers_meta
    }

    // todo test!!
    pub(crate) fn get_meta_ticker(&self, id: &TickerId) -> &TickerMeta {
        &self.meta.tickers_meta.get(id).expect(&format!(
            "{} add ticker to db: {:?}",
            line!(),
            id
        ))
    }

    // todo test!!
    pub(crate) fn get_portfolio_action(&self) -> &meta::PortfolioAction {
        &self.meta.portfolio_action
    }

    pub fn get_current_version(&self) -> &i32 {
        &self.actual.get_version()
    }
}
