#![allow(dead_code, unused)]

use super::{actual::*, goal::*, meta::*};
use crate::std_ext::*;
use crate::ticker::*;
use chrono::prelude::*;
use std::collections::HashMap;

pub struct TestHelper {}

impl TestHelper {
    pub fn get_port_goal() -> PortfolioGoal {
        PortfolioGoal::new(
            1,
            "name".to_string(),
            Some("description".to_string()),
            &Self::get_goal_tickers(),
            &Self::get_tickers(),
            &50.0,
        )
    }

    pub fn get_tickers() -> HashMap<TickerId, Ticker> {
        let mut map = HashMap::new();
        // id, symbol, exchange, fee, price, kind,
        let t1 =
            Ticker::new(1, symbol!("t1"), 2, 1.25, 10.0, InvestmentKind::Stock);
        let t2 =
            Ticker::new(2, symbol!("t2"), 2, 2.25, 50.0, InvestmentKind::Bond);
        let t3 =
            Ticker::new(3, symbol!("t3"), 2, 3.25, 60.0, InvestmentKind::Stock);
        map.insert(tic_id!(t1.id.clone()), t1);
        map.insert(tic_id!(t2.id.clone()), t2);
        map.insert(tic_id!(t3.id.clone()), t3);
        map
    }

    pub fn get_actual_port() -> PortfolioActual {
        // id, fk_user_id, fk_port_g_id, stock_percent,
        // deviation_percent, version, last_updated, tickers_actual
        PortfolioActual::new(
            1,
            1,
            2,
            50.0,
            1.5,
            1,
            Utc::now(),
            Self::get_actual_tickers(),
        )
    }

    fn get_actual_tickers() -> HashMap<TickerId, TickerActual> {
        let mut map = HashMap::new();
        // id, port_goal_id, port_actual_id, ticker_id, actual_shares
        let t1 = TickerActual::new(1, 1, 1, 1, 1.0);
        let t2 = TickerActual::new(2, 1, 1, 2, 1.0);
        let t3 = TickerActual::new(3, 1, 1, 3, 1.0);
        map.insert(tic_id!(t1.id.clone()), t1);
        map.insert(tic_id!(t2.id.clone()), t2);
        map.insert(tic_id!(t3.id.clone()), t3);
        map
    }

    pub fn get_goal_tickers() -> HashMap<TickerId, GoalTicker> {
        let mut map = HashMap::new();
        // goal percent gets calculated based on stock percent in PortfolioGoal::new
        // id, port_goal_id, ticker_id, goal_percent, order
        let t1 = GoalTicker::new(1, 1, 1, 40.0, 1); // stock is 40 of 50% so 20%
        let t2 = GoalTicker::new(2, 1, 2, 100.0, 2); // only bond is 100 so 50%
        let t3 = GoalTicker::new(3, 1, 3, 60.0, 3); // stock is 60 of 50% so 30%

        map.insert(tic_id!(t1.ticker_id.clone()), t1);
        map.insert(tic_id!(t2.ticker_id.clone()), t2);
        map.insert(tic_id!(t3.ticker_id.clone()), t3);

        map
    }

    pub fn get_port_meta_value() -> PortfolioMeta {
        let mut map = HashMap::new();
        let t1 = TickerMeta {
            id: tic_id!(1),
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        };
        let t2 = TickerMeta {
            id: tic_id!(2),
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        };
        let t3 = TickerMeta {
            id: tic_id!(3),
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        };
        map.insert(t1.id.clone(), t1.clone());
        map.insert(t2.id.clone(), t2.clone());
        map.insert(t3.id.clone(), t3.clone());
        PortfolioMeta {
            tickers_meta: map,
            total_value: 10.0,
            stock_value: 10.0,
            portfolio_action: PortfolioAction::BuyEither,
            stock_percent: 0.0,
        }
    }

    pub fn get_port_meta_action() -> PortfolioMeta {
        let mut map = HashMap::new();
        let t1 = TickerMeta {
            id: tic_id!(1),
            action: TickerAction::Hold,
            ticker_value: 60.0,
            ticker_percent: 60.0,
        };
        let t2 = TickerMeta {
            id: tic_id!(2),
            action: TickerAction::Hold,
            ticker_value: 50.0,
            ticker_percent: 50.0,
        };
        let t3 = TickerMeta {
            id: tic_id!(3),
            action: TickerAction::Hold,
            ticker_value: 10.0,
            ticker_percent: 10.0,
        };
        map.insert(t1.id.clone(), t1.clone());
        map.insert(t2.id.clone(), t2.clone());
        map.insert(t3.id.clone(), t3.clone());
        PortfolioMeta {
            tickers_meta: map,
            total_value: 100.0,
            stock_value: 40.0,
            portfolio_action: PortfolioAction::BuyEither,
            stock_percent: 40.0,
        }
    }

    pub fn get_port_meta_per() -> PortfolioMeta {
        let mut map = HashMap::new();
        let t1 = TickerMeta {
            id: tic_id!(1),
            action: TickerAction::Hold,
            ticker_value: 10.0,
            ticker_percent: 0.0,
        };
        let t2 = TickerMeta {
            id: tic_id!(2),
            action: TickerAction::Hold,
            ticker_value: 30.0,
            ticker_percent: 0.0,
        };
        let t3 = TickerMeta {
            id: tic_id!(3),
            action: TickerAction::Hold,
            ticker_value: 60.0,
            ticker_percent: 0.0,
        };
        map.insert(t1.id.clone(), t1.clone());
        map.insert(t2.id.clone(), t2.clone());
        map.insert(t3.id.clone(), t3.clone());
        PortfolioMeta {
            tickers_meta: map,
            total_value: 100.0,
            stock_value: 40.0,
            portfolio_action: PortfolioAction::BuyEither,
            stock_percent: 0.0,
        }
    }

    pub fn get_port_meta_total_val() -> PortfolioMeta {
        let mut map = HashMap::new();
        let t1 = TickerMeta {
            id: tic_id!(1),
            action: TickerAction::Hold,
            ticker_value: 1.0,
            ticker_percent: 0.0,
        };
        let t2 = TickerMeta {
            id: tic_id!(2),
            action: TickerAction::Hold,
            ticker_value: 20.0,
            ticker_percent: 0.0,
        };
        let t3 = TickerMeta {
            id: tic_id!(3),
            action: TickerAction::Hold,
            ticker_value: 300.0,
            ticker_percent: 0.0,
        };
        map.insert(t1.id.clone(), t1.clone());
        map.insert(t2.id.clone(), t2.clone());
        map.insert(t3.id.clone(), t3.clone());
        PortfolioMeta {
            tickers_meta: map,
            total_value: 0.0,
            stock_value: 0.0,
            portfolio_action: PortfolioAction::BuyEither,
            stock_percent: 0.0,
        }
    }

    pub fn get_ticker_meta() -> TickerMeta {
        let id = tic_id!(1);
        TickerMeta {
            id: id,
            action: TickerAction::Hold,
            ticker_value: 100.0,
            ticker_percent: 5.0,
        }
    }

    pub fn get_ticker_meta_zero_percent() -> TickerMeta {
        let id = tic_id!(1);
        TickerMeta {
            id: id,
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        }
    }
}
