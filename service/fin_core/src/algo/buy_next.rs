use crate::algo::{Action, ActionInfo};
use crate::portfolio::{
    self, PortfolioActual, PortfolioGoal, PortfolioState, TickerActual,
};
use crate::std_ext::*;
use std::collections::HashMap;

/// This object is used to pass BuyNext info from backend
/// to the server interface layer. This is necessary because
/// BuyNext contains a reference to PortfolioState and
/// therefore cannot be passed across owned boundary.
#[derive(Debug)]
pub struct BuyNextInfo {
    pub actions: Vec<Action>,
    pub buy_value: f64,
}

impl<'s> From<BuyNext<'s>> for BuyNextInfo {
    fn from(item: BuyNext) -> Self {
        BuyNextInfo {
            actions: item.actions,
            buy_value: item.buy_value,
        }
    }
}

/// A state which captures the buy next algorithm. It takes
/// a reference to the PortfolioState and calculates the
/// actions and buy value.
#[derive(Debug)]
pub struct BuyNext<'s> {
    pub port_state: &'s mut PortfolioState,
    pub actions: Vec<Action>,
    pub buy_value: f64,
}

impl<'s> BuyNext<'s> {
    pub fn new(port: &'s mut PortfolioState) -> Self {
        BuyNext {
            port_state: port,
            actions: Vec::new(),
            buy_value: 0.0, // default uninitialized value
        }
    }

    // todo test!!!
    /// This buys one stock or bond and return the action. The
    /// action might be None if the `buy_amount` is too small
    /// and the stock/bond prices are too high.
    pub fn buy_one(&mut self, buy_amount: f64) -> Option<Action> {
        // get action
        let action = self.get_action();

        // buying more would put us above the buy value
        if (self.buy_value + action.get_price() > buy_amount) {
            None
        } else {
            // update PortfolioState
            self.port_state.apply_action(&action);

            // update BuyNext
            self.update_buy_next(&action);

            Some(action)
        }
    }

    // todo test!!!
    pub fn update_buy_next(&mut self, action: &Action) {
        // update buy_value
        self.buy_value += action.get_price();
        StdExt::round_two_digits_64(&mut self.buy_value);

        // update action
        self.actions.push(action.clone());
    }

    pub fn get_action(&self) -> Action {
        let filter_kind = Self::filter_kind(&self.port_state);

        let filter_buys = Self::filter_buys(filter_kind);

        let filter_percent_diff =
            Self::filter_percent_diff(&self.port_state, filter_buys);

        Action::Buy(ActionInfo {
            id: filter_percent_diff.id.clone(),
            shares: 1.0, // 1 action so we purchase 1 share
            price: self.port_state.get_ticker(&filter_percent_diff.id).price,
        })
    }

    // todo test!!
    pub fn get_action2(&self) -> Action {
        let filtered = Self::filter_kind(&self.port_state);

        let filtered = Self::filter_buys(filtered);

        let filtered = Self::filter_percent_diff(&self.port_state, filtered);

        Action::Buy(ActionInfo {
            id: filtered.id.clone(),
            shares: 1.0, // 1 action so we purchase 1 share
            price: self.port_state.get_ticker(&filtered.id).price,
        })
    }

    /// filter based on portfolio action
    pub fn filter_kind(
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

    /// filter based on ticker action (buys)
    pub fn filter_buys(
        tic_metas: Vec<&portfolio::TickerMeta>,
    ) -> Vec<&portfolio::TickerMeta> {
        // filter buys
        let buys = tic_metas
            .iter()
            .filter(|x| matches!(&x.action, portfolio::TickerAction::Buy))
            .collect::<Vec<&&portfolio::TickerMeta>>();

        if (buys.is_empty()) {
            // dont filter since we dont have buys
            tic_metas
        } else {
            buys.into_iter().map(|x| *x).collect()
        }
    }

    /// filter based on greatest percent difference
    // todo make this sort so we get a list of actions we can do
    pub fn filter_percent_diff(
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

            let meta_per = port_state.get_meta_ticker(&x.id).ticker_percent;
            let desired_per = port_state.get_goal_ticker(&x.id).goal_percent;
            let x_actual_minus_desired = desired_per - meta_per;

            let meta_per = port_state.get_meta_ticker(&y.id).ticker_percent;
            let desired_per = port_state.get_goal_ticker(&y.id).goal_percent;
            let y_actual_minus_desired = desired_per - meta_per;

            if (x_actual_minus_desired > y_actual_minus_desired) {
                x
            } else {
                y.clone()
            }
        })
    }
}

#[cfg(test)]
mod test {

    use super::super::test_helper_buy_next::*;
    use super::*;
    use chrono::prelude::*;
    use portfolio::*;

    #[test]
    // filter based on portfolio action
    fn bn_test_filter_kind() {
        let p_state = TestHelperBuyNext::helper_get_port_state();
        let meta = BuyNext::filter_kind(&p_state);

        assert_eq!(&PortfolioAction::BuyBond, p_state.get_portfolio_action());
        assert_eq!(1, meta.len());
        assert_eq!(tic_id!(2), meta.get(0).expect("test").id);
    }

    #[test]
    // filter based on ticker action (buys)
    fn bn_test_filter_buys_no_buys() {
        let mut metas = Vec::new();
        metas.push(TickerMeta {
            id: tic_id!(1),
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        });
        metas.push(TickerMeta {
            id: tic_id!(2),
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        });
        metas.push(TickerMeta {
            id: tic_id!(3),
            action: TickerAction::Sell,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        });
        metas.push(TickerMeta {
            id: tic_id!(4),
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        });
        let metas = metas.iter().collect();
        let meta = BuyNext::filter_buys(metas);

        assert_eq!(4, meta.len());
    }

    #[test]
    // filter based on ticker action (buys)
    fn bn_test_filter_buys() {
        let mut metas = Vec::new();
        metas.push(TickerMeta {
            id: tic_id!(1),
            action: TickerAction::Buy,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        });
        metas.push(TickerMeta {
            id: tic_id!(2),
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        });
        metas.push(TickerMeta {
            id: tic_id!(3),
            action: TickerAction::Sell,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        });
        metas.push(TickerMeta {
            id: tic_id!(4),
            action: TickerAction::Buy,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        });
        let metas = metas.iter().collect();
        let meta = BuyNext::filter_buys(metas);

        assert_eq!(2, meta.len());
        assert_eq!(tic_id!(1), meta.get(0).expect("test").id);
        assert_eq!(tic_id!(4), meta.get(1).expect("test").id);
    }

    #[test]
    // filter based on greatest percent difference
    fn bn_test_filter_percent_diff() {
        let p_state = TestHelperBuyNext::helper_get_port_state();
        let metas = TestHelperBuyNext::helper_get_tic_metas();
        let metas = metas.iter().collect();
        let meta = BuyNext::filter_percent_diff(&p_state, metas);

        assert_eq!(tic_id!(1), meta.id);
    }
}
