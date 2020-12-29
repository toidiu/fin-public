use crate::algo::{Action, ActionInfo};
use crate::portfolio::{
    self, PortfolioActual, PortfolioGoal, PortfolioState, TickerActual,
};
use crate::std_ext::*;
use chrono::prelude::*;
use portfolio::*;
use std::collections::HashMap;

struct TestHelperBuyNext {}

impl TestHelperBuyNext {
    fn helper_get_port_state() -> PortfolioState {
        let pa = Self::helper_get_actual_port();
        let pg = Self::helper_get_goal_port();
        let tickers = Self::helper_get_tickers();
        PortfolioState::new(pa, pg, tickers)
    }

    fn helper_get_tic_metas() -> Vec<portfolio::TickerMeta> {
        let mut ret = Vec::new();
        ret.push(Self::helper_get_meta(&tic_id!(1)));
        ret.push(Self::helper_get_meta(&tic_id!(3)));
        ret
    }

    fn helper_get_meta(id: &TickerId) -> TickerMeta {
        TickerMeta {
            id: id.clone(),
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        }
    }

    fn helper_get_tickers() -> HashMap<TickerId, Ticker> {
        let mut map = HashMap::new();
        // id, symbol, exchange, fee, price, kind,
        let t1 =
            Ticker::new(1, symbol!("t1"), 2, 1.25, 1.0, InvestmentKind::Stock);
        let t2 =
            Ticker::new(2, symbol!("t2"), 2, 2.25, 1.0, InvestmentKind::Bond);
        let t3 =
            Ticker::new(3, symbol!("t3"), 2, 3.25, 1.0, InvestmentKind::Stock);
        map.insert(tic_id!(t1.id), t1);
        map.insert(tic_id!(t2.id), t2);
        map.insert(tic_id!(t3.id), t3);
        map
    }

    fn helper_get_actual_port() -> PortfolioActual {
        PortfolioActual::new(
            // id
            1,
            // fk_user_id
            1,
            // fk_port_g_id
            1,
            // stock_percent,
            50.0,
            // deviation_percent
            1.5,
            // version
            1,
            // last_updated
            Utc::now(),
            "name".to_string(),
            "description".to_string(),
            // tickers_actual
            Self::helper_get_actual_tickers(),
        )
    }

    fn helper_get_actual_tickers() -> HashMap<TickerId, TickerActual> {
        let mut map = HashMap::new();
        // id, port_goal_id, port_actual_id, ticker_id, actual_shares
        let t1 = TickerActual::new(1, 1, 1, 1, 24.0);
        let t2 = TickerActual::new(2, 1, 1, 2, 0.0);
        let t3 = TickerActual::new(3, 1, 1, 3, 26.0);
        map.insert(tic_id!(t1.id), t1);
        map.insert(tic_id!(t2.id), t2);
        map.insert(tic_id!(t3.id), t3);
        map
    }

    fn helper_get_goal_port() -> PortfolioGoal {
        PortfolioGoal::new(
            // id
            1,
            // name
            "name".to_string(),
            // description
            Some("".to_string()),
            // tickers_goal
            &Self::helper_get_goal_tickers(),
            // tickers_map
            &Self::helper_get_tickers(),
            // actual_stock_percent
            &50.0,
        )
    }

    pub fn helper_get_goal_tickers() -> HashMap<TickerId, GoalTicker> {
        let mut map = HashMap::new();
        // goal percent gets calculated based on stock percent in PortfolioGoal::new
        // id, port_goal_id, ticker_id, goal_percent, order
        let t1 = GoalTicker::new(1, 1, 1, 50.0, 1); // stock is 50 of 50% so 25%
        let t2 = GoalTicker::new(2, 1, 2, 100.0, 2); // bond is 100 of 50% so 50%
        let t3 = GoalTicker::new(3, 1, 3, 50.0, 3); // stock is 50 of 50% so 25%

        map.insert(tic_id!(t1.ticker_id), t1);
        map.insert(tic_id!(t2.ticker_id), t2);
        map.insert(tic_id!(t3.ticker_id), t3);

        map
    }
}
