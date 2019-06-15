use crate::algo::{Action, ActionInfo};
use crate::backend;
use crate::data;
use crate::errors::{FinError, ResultFin};
use crate::portfolio::{
    self, PortfolioActual, PortfolioGoal, PortfolioState, TickerActual,
};
use crate::server;
use crate::std_ext::*;
use crate::ticker::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BuyNext {
    pub init_state: PortfolioState,
    pub evolved_actual: HashMap<TickerId, TickerActual>,
    pub actions: Vec<Action>,
    pub buy_value: f64,
    pub action_summary: HashMap<TickerId, Action>,
}

impl BuyNext {
    pub fn new(port: PortfolioState) -> Self {
        let initial_actual_tickers = port.get_actual_tickers().clone();
        BuyNext {
            init_state: port,
            evolved_actual: initial_actual_tickers,
            actions: Vec::new(),
            buy_value: 0.0, // default uninitialized value
            action_summary: HashMap::new(),
        }
    }

    // todo test!!!
    pub fn get_next_action(
        buy_next: &mut BuyNext,
        buy_amount: f64,
        port_state: PortfolioState,
    ) -> Option<Action> {
        // get action
        let action = Self::get_buy_next_action(&port_state);

        // buying more would put us above the buy value
        if (buy_next.buy_value + action.get_price() > buy_amount) {
            None
        } else {
            // update PortfolioState
            let port_state = port_state.apply_action(&action);

            // update BuyNext
            Self::update_buy_next(
                buy_next,
                port_state.get_actual_tickers(),
                &action,
            );

            Some(action)
        }
    }

    // todo test!!!
    fn update_buy_next(
        buy_next: &mut BuyNext,
        updated_actual_tic: &HashMap<TickerId, TickerActual>,
        action: &Action,
    ) {
        // update buy_value
        buy_next.buy_value += action.get_price();
        StdExt::round_two_digits_64(&mut buy_next.buy_value);

        // update action
        buy_next.actions.push(action.clone());

        // update final state
        buy_next.evolved_actual = updated_actual_tic.clone();
    }

    // todo test!!
    pub fn get_buy_next_action(
        port_state: &portfolio::PortfolioState,
    ) -> Action {
        /// filter based on portfolio action
        let filter_kind = Self::filter_kind(port_state);

        /// filter based on ticker action (buys)
        let filter_buys = Self::filter_buys(filter_kind);

        /// filter based on greatest percent difference
        let filter_percent_diff =
            Self::filter_percent_diff(port_state, filter_buys);

        Action::Buy(ActionInfo {
            id: filter_percent_diff.id.clone(),
            shares: 1.0, // 1 action so we purchase 1 share
            price: port_state.get_ticker(&filter_percent_diff.id).price,
        })
    }

    // todo test!!!
    fn filter_kind(
        port_state: &portfolio::PortfolioState,
    ) -> Vec<&portfolio::TickerMeta> {
        match port_state.get_portfolio_action() {
            portfolio::PortfolioAction::BuyStock => port_state
                .get_meta_tickers()
                .iter()
                .filter(|x| port_state.get_ticker(&x.0).is_stock())
                .map(|x| x.1)
                .collect(),

            portfolio::PortfolioAction::BuyBond => port_state
                .get_meta_tickers()
                .iter()
                .filter(|x| port_state.get_ticker(&x.0).is_bond())
                .map(|x| x.1)
                .collect(),

            portfolio::PortfolioAction::BuyEither => {
                port_state.get_meta_tickers().values().collect()
            }
        }
    }

    // todo test!!!
    fn filter_buys(
        tic_metas: Vec<&portfolio::TickerMeta>,
    ) -> Vec<&portfolio::TickerMeta> {
        // filter buys
        let contains_no_buys = tic_metas
            .iter()
            .filter(|x| matches!(&x.action, portfolio::TickerAction::Buy))
            .collect::<Vec<&&portfolio::TickerMeta>>();

        // todo test
        if (contains_no_buys.is_empty()) {
            // dont filter since we dont have buys
            tic_metas
        } else {
            contains_no_buys.into_iter().map(|x| *x).collect()
        }
    }

    // todo test!!!
    fn filter_percent_diff(
        port_state: &portfolio::PortfolioState,
        tic_metas: Vec<&portfolio::TickerMeta>,
    ) -> portfolio::TickerMeta {
        let empty_diff = portfolio::EMPTY_TICKER_META.clone();
        tic_metas.into_iter().fold(empty_diff, |x, y| {
            if (x.id == portfolio::EMPTY_TICKER_META.id) {
                return y.clone();
            } else if (y.id == portfolio::EMPTY_TICKER_META.id) {
                return x;
            }

            let actual = port_state.get_meta_ticker(&x.id).ticker_percent;
            let desired = port_state.get_goal_ticker(&x.id).goal_percent;
            let x_actual_minus_desired = actual - desired;

            let actual = port_state.get_meta_ticker(&y.id).ticker_percent;
            let desired = port_state.get_goal_ticker(&y.id).goal_percent;
            let y_actual_minus_desired = actual - desired;

            if (x_actual_minus_desired > y_actual_minus_desired) {
                y.clone()
            } else {
                x
            }
        })
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_get_next_action() {
        assert!(true)
    }
}
