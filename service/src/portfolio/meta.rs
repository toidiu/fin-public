use super::SMALL_PERCENT_DEVIATION;
use super::{actual::*, goal::*};
use crate::std_ext::*;
use crate::ticker::*;
use std::collections::HashMap;

lazy_static! {
    pub static ref EMPTY_TICKER_META: TickerMeta = {
        TickerMeta {
            id: tic_id!(-1),
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        }
    };
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum PortfolioAction {
    BuyStock,
    BuyBond,
    BuyEither,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum TickerAction {
    Buy,
    Sell,
    Hold,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioMeta {
    pub tickers_meta: HashMap<TickerId, TickerMeta>,
    pub total_value: f64,
    pub stock_value: f64,
    // todo maybe calculate lazily
    pub portfolio_action: PortfolioAction,
    pub stock_percent: f32,
}

impl PortfolioMeta {
    pub(super) fn new(
        tickers: &HashMap<TickerId, Ticker>,
        goal: &PortfolioGoal,
        actual: &PortfolioActual,
    ) -> Self {
        let mut meta = PortfolioMeta {
            tickers_meta: Self::populate_ticker_meta(&actual.tickers_actual),
            portfolio_action: PortfolioAction::BuyEither,
            total_value: 0.0,
            stock_value: 0.0,
            stock_percent: 0.0,
        };

        meta.recalculate(&tickers, &actual, &goal);

        meta
    }

    pub(super) fn recalculate(
        &mut self,
        tickers: &HashMap<TickerId, Ticker>,
        actual: &PortfolioActual,
        goal: &PortfolioGoal,
    ) {
        // calc ticker meta value
        // calc total value
        // calc stock value
        self.calc_value(tickers, actual);

        // calc ticker percent
        // calc stock percent
        self.calc_percent();

        // calc action
        self.calc_action(
            goal,
            &actual.stock_percent,
            &actual.deviation_percent,
        );
    }

    fn populate_ticker_meta(
        tickers_actual: &HashMap<TickerId, TickerActual>,
    ) -> HashMap<TickerId, TickerMeta> {
        let mut map = HashMap::new();
        for (t_id, _) in tickers_actual.iter() {
            map.insert(t_id.clone(), TickerMeta::default(&t_id));
        }
        map
    }

    pub(super) fn calc_value(
        &mut self,
        tickers: &HashMap<TickerId, Ticker>,
        actual: &PortfolioActual,
    ) {
        let mut temp_stock_value = 0.0;
        for (t_id, tic_meta) in self.tickers_meta.iter_mut() {
            let tic = tickers
                .get(&t_id)
                .expect(&format!("add ticker to db: {:?}", &t_id));
            let tic_act = actual.get_ticker_a(&t_id);
            let tic_value = tic.price * tic_act.actual_shares;

            tic_meta.ticker_value = tic_value;
            StdExt::round_two_digits_64(&mut tic_meta.ticker_value);

            // stock value
            if (tic.is_stock()) {
                temp_stock_value += tic_value;
            }
        }
        self.stock_value = temp_stock_value;
        self.calc_total_value();

        // round total value to 2 digits
        StdExt::round_two_digits_64(&mut self.total_value);
    }

    pub(super) fn calc_percent(&mut self) {
        // dont divide by 0
        if (self.total_value == 0.0) {
            self.stock_percent = 0.0;
            for (_, tic_meta) in self.tickers_meta.iter_mut() {
                tic_meta.ticker_percent = 0.0;
            }
        } else {
            self.stock_percent =
                ((self.stock_value / self.total_value) * 100.0) as f32;
            StdExt::round_two_digits(&mut self.stock_percent);

            for (_, tic_meta) in self.tickers_meta.iter_mut() {
                tic_meta.ticker_percent =
                    ((tic_meta.ticker_value / self.total_value) * 100.0) as f32;
                StdExt::round_two_digits(&mut tic_meta.ticker_percent);
            }
        }
    }

    pub(super) fn calc_action(
        &mut self,
        goal: &PortfolioGoal,
        desired_stock_percent: &f32,
        deviation_percent: &f32,
    ) {
        // desired - calculated stock percent
        let diff = desired_stock_percent - self.stock_percent;
        self.portfolio_action =
            if ((diff < 0.0) && diff.abs() > *deviation_percent) {
                // If gS%-aS% is - and abs val above q% then buy bonds
                PortfolioAction::BuyBond
            } else if (diff > 0.0 && diff > *deviation_percent) {
                // If gS%-aS% is + and above q% then buy stocks
                PortfolioAction::BuyStock
            } else {
                // else buy stock or bond
                PortfolioAction::BuyEither
            };

        for (t_id, tic_meta) in self.tickers_meta.iter_mut() {
            let goal_tic = goal.get_ticker_g(t_id);
            tic_meta
                .calc_ticker_action(goal_tic.goal_percent, *deviation_percent);
        }
    }

    pub(super) fn calc_total_value(&mut self) {
        let mut total_val = 0.0;
        for (_s, x) in self.tickers_meta.iter() {
            total_val += x.ticker_value;
        }
        self.total_value = total_val;
    }

    pub(super) fn get_ticker(&self, id: &TickerId) -> &TickerMeta {
        self.tickers_meta
            .get(&id)
            .expect(&format!("add ticker to db: {:?}", &id))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerMeta {
    pub id: TickerId,
    pub action: TickerAction,
    pub ticker_value: f64,
    pub ticker_percent: f32,
}

impl TickerMeta {
    fn default(id: &TickerId) -> TickerMeta {
        TickerMeta {
            id: id.clone(),
            action: TickerAction::Hold,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        }
    }

    fn calc_ticker_action(
        &mut self,
        desired_percent: f32,
        deviation_percent: f32,
    ) {
        // desired - calculated stock percent
        let actual_minus_desired = self.ticker_percent - desired_percent;
        let is_more = actual_minus_desired > 0.0;
        // TODO make SMALL_PERCENT_DEVIATION a % based calculation.. too many unknowns
        let is_significant = (actual_minus_desired.abs() > deviation_percent
            || (self.ticker_percent < 1.5
                && actual_minus_desired.abs() > SMALL_PERCENT_DEVIATION));

        self.action = match is_more {
            true if is_significant => TickerAction::Sell,
            false if is_significant => TickerAction::Buy,
            _ => TickerAction::Hold,
        };
    }
}

#[cfg(test)]
mod tests {

    use super::super::test_helper::TestHelper;
    use super::*;

    #[test]
    fn test_new_portfolio_meta() {
        assert!(true)
    }

    #[test]
    fn test_recalculate() {
        assert!(true)
    }

    #[test]
    fn test_populate_ticker_meta() {
        assert!(true)
    }

    #[test]
    fn test_calc_value() {
        let mut pm = TestHelper::get_port_meta_value();
        let tickers = TestHelper::get_tickers();
        let port_actual = TestHelper::get_actual_port();
        pm.calc_value(&tickers, &port_actual);

        assert_eq!(pm.stock_value, 70.0);
        assert_eq!(pm.total_value, 120.0);
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(1)).unwrap().ticker_value,
            10.0
        );
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(2)).unwrap().ticker_value,
            50.0
        );
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(3)).unwrap().ticker_value,
            60.0
        );
    }

    #[test]
    fn test_calc_percent() {
        let mut pm = TestHelper::get_port_meta_per();
        pm.calc_percent();
        assert_eq!(pm.stock_percent, 40.0);
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(1)).unwrap().ticker_percent,
            10.0
        );
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(2)).unwrap().ticker_percent,
            30.0
        );
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(3)).unwrap().ticker_percent,
            60.0
        );
    }

    #[test]
    fn test_calc_percent_dont_divide_by_zero() {
        let mut pm = TestHelper::get_port_meta_per();
        pm.total_value = 0.0;
        pm.calc_percent();
        assert_eq!(pm.stock_percent, 0.0);
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(1)).unwrap().ticker_percent,
            0.0
        );
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(2)).unwrap().ticker_percent,
            0.0
        );
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(3)).unwrap().ticker_percent,
            0.0
        );
    }

    #[test]
    fn test_calc_action() {
        let goal: PortfolioGoal = TestHelper::get_port_goal();
        let stock_percent = 90.0;
        let deviation_percent = 1.5;
        let mut pm = TestHelper::get_port_meta_action();

        pm.calc_action(&goal, &80.0, &deviation_percent);
        assert_eq!(pm.portfolio_action, PortfolioAction::BuyStock);
        pm.calc_action(&goal, &20.0, &deviation_percent);
        assert_eq!(pm.portfolio_action, PortfolioAction::BuyBond);
        pm.calc_action(&goal, &40.0, &deviation_percent);
        assert_eq!(pm.portfolio_action, PortfolioAction::BuyEither);

        // make sure that ticker action is also called
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(1)).unwrap().action,
            TickerAction::Sell
        );
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(2)).unwrap().action,
            TickerAction::Hold
        );
        assert_eq!(
            pm.tickers_meta.get(&tic_id!(3)).unwrap().action,
            TickerAction::Buy
        );
    }

    #[test]
    fn test_calc_total_value() {
        let mut pm = TestHelper::get_port_meta_total_val();
        pm.calc_total_value();
        assert_eq!(pm.total_value, 321.0);
    }

    #[test]
    fn test_default_ticker_meta() {
        let id = tic_id!(1);
        let tm = TickerMeta::default(&id);
        assert_eq!(tm.id, id);
        assert_eq!(tm.action, TickerAction::Hold);
        assert_eq!(tm.ticker_value, 0.0);
        assert_eq!(tm.ticker_percent, 0.0);
    }

    #[test]
    fn test_calc_ticker_action() {
        let mut tm = TestHelper::get_ticker_meta();
        tm.calc_ticker_action(4.0, 0.5);
        assert_eq!(tm.action, TickerAction::Sell);
        tm.calc_ticker_action(4.0, 1.5);
        assert_eq!(tm.action, TickerAction::Hold);

        tm.calc_ticker_action(6.0, 0.5);
        assert_eq!(tm.action, TickerAction::Buy);
        tm.calc_ticker_action(6.0, 1.5);
        assert_eq!(tm.action, TickerAction::Hold);
    }

    #[test]
    fn test_calc_ticker_action_small_percent() {
        let mut zero_tm = TestHelper::get_ticker_meta_zero_percent();
        zero_tm.calc_ticker_action(0.6, 1.5);
        assert_eq!(zero_tm.action, TickerAction::Buy);
    }

}
