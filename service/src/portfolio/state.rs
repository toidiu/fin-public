use super::actual::{self, *};
use super::algo::{self, *};
use super::goal::{self, *};
use super::meta::{self, *};
use crate::algo::{Action, ActionInfo};
use crate::backend;
use crate::portfolio::ticker::{self, *};

use crate::errors::ResultFin;
use crate::{data, server, std_ext::*};
use chrono::prelude::*;
use std::{cmp::Ordering, collections::HashMap, num};

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioState {
    goal: PortfolioGoal,
    actual: PortfolioActual,
    meta: PortfolioMeta,
    tickers: HashMap<TickerId, Ticker>,
}

// **************************************
// mutate
// **************************************
impl PortfolioState {
    /// Modify actual and then re-calculate meta.
    pub(crate) fn apply_action(&mut self, action: &Action) {
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
// read
// **************************************
impl PortfolioState {
    pub(crate) fn new(
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
    pub(crate) fn get_ticker(&self, id: &TickerId) -> &Ticker {
        &self.tickers.get(id).expect(&format!(
            "{} add ticker to db: {:?}",
            line!(),
            &id
        ))
    }

    // todo test!!
    pub(crate) fn get_actual_port(&self) -> &PortfolioActual {
        &self.actual
    }

    // todo test!!
    pub(crate) fn get_actual_tickers(
        &self,
    ) -> &HashMap<TickerId, TickerActual> {
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

    pub(crate) fn get_current_version(&self) -> &i32 {
        &self.actual.get_version()
    }
}

/// We are implementing Into rather than From because we are
/// accessing private fields. We could expose them but that
/// increases the surface area of the api.
impl Into<server::PortfolioStateResp> for PortfolioState {
    // todo test!!
    fn into(self) -> server::PortfolioStateResp {
        let mut tickers: Vec<server::TickerResp> = self
            .goal
            .tickers_goal
            .iter()
            .map(|x| {
                let ticker = self.get_ticker(x.0);
                let tg = self.goal.get_ticker_g(x.0);
                let tm = self.meta.get_ticker(x.0);
                let ta = self.actual.get_ticker_a(x.0);

                server::TickerResp {
                    id: x.0.clone(),
                    symbol: ticker.symbol.clone(),
                    kind: ticker.get_kind().clone(),
                    fee: ticker.fee,
                    goal_percent: tg.goal_percent,
                    actual_percent: tm.ticker_percent,
                    actual_shares: ta.actual_shares,
                    actual_value: tm.ticker_value,
                    price: ticker.price,
                    order: tg.get_order(),
                }
            })
            .collect();
        tickers.sort_by(|a, b| a.order.cmp(&b.order));
        server::PortfolioStateResp {
            name: self.goal.name,
            goal_id: self.goal.id,
            tickers,
            goal_stock_percent: self.actual.stock_percent,
            actual_stock_percent: self.meta.stock_percent,
            total_value: self.meta.total_value,
            deviation_percent: self.actual.deviation_percent,
        }
    }
}
