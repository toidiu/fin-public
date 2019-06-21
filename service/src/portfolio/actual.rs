use crate::ticker::*;
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PortfolioActual {
    pub id: i64,
    pub fk_user_id: i64,
    pub fk_port_g_id: i64,
    pub stock_percent: f32,
    pub deviation_percent: f32,
    version: i32,
    pub last_updated: DateTime<Utc>,
    pub tickers_actual: HashMap<TickerId, TickerActual>,
}

impl PortfolioActual {
    pub fn new(
        id: i64,
        fk_user_id: i64,
        fk_port_g_id: i64,
        stock_percent: f32,
        deviation_percent: f32,
        version: i32,
        last_updated: DateTime<Utc>,
        tickers_actual: HashMap<TickerId, TickerActual>,
    ) -> Self {
        PortfolioActual {
            id: id,
            fk_user_id: fk_user_id,
            fk_port_g_id: fk_port_g_id,
            stock_percent: stock_percent,
            deviation_percent: deviation_percent,
            version: version,
            last_updated: last_updated,
            tickers_actual: tickers_actual,
        }
    }

    pub(super) fn get_ticker_a(&self, id: &TickerId) -> &TickerActual {
        self.tickers_actual
            .get(&id)
            .expect(&format!("add ticker to db: {:?}", &id))
    }

    pub(super) fn get_version(&self) -> &i32 {
        &self.version
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerActual {
    pub id: i64,
    pub port_goal_id: i64,
    pub port_actual_id: i64,
    ticker_id: i64,
    pub actual_shares: f64,
}

impl TickerActual {
    pub(crate) fn new(
        id: i64,
        port_goal_id: i64,
        port_actual_id: i64,
        ticker_id: i64,
        actual_shares: f64,
    ) -> Self {
        TickerActual {
            id,
            port_goal_id,
            port_actual_id,
            ticker_id,
            actual_shares,
        }
    }

    pub(crate) fn get_ticker_id(&self) -> i64 {
        self.ticker_id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn ta_new() {
    //     let ta1 = TickerActual::new(symbol!("vti"), 1.5);
    //     assert_eq!(ta1.symbol, symbol!("vti"));
    //     assert_eq!(ta1.actual_shares, 1.5);
    //     // assert_eq!(ta1.aavv, 0.0);
    //     // assert_eq!(ta1.actual_percent, 0.0);
    // }

    // #[test]
    // fn ta_get_percent() {
    // let ta1 = Helper::get_ta_bond();
    // assert_eq!(ta1.get_actual_percent(), 5.4);
    // }

    // #[test]
    // fn ta_actual_percent() {
    // let mut ta = TickerActual {
    // symbol: symbol!("vti"),
    // aavv: 200.0,
    // actual_shares: 1.0,
    // actual_percent: 0.0,
    // };
    // ta.update_actual_percent(&600.0);
    // assert_eq!(33.33, ta.actual_percent);
    // }

    // #[test]
    // fn ta_actual_value() {
    // let mut ta = TickerActual {
    // symbol: symbol!("vti"),
    // aavv: 0.0,
    // actual_shares: 2.5,
    // actual_percent: 0.0,
    // };
    // ta.update_actual_value(10.5);
    // assert_eq!(26.25, ta.aavv);
    // }

    // #[test]
    // fn pa_update_stock_percent() {
    // let mut pa = Helper::get_portfolio_actual();
    // pa.update_stock_percent(15.0);
    // assert_eq!(7.5, pa.actual_stock_percent);
    // }

    // #[test]
    // fn pa_get_stock_percent() {
    // let pa = Helper::get_portfolio_actual();
    // assert_eq!(pa.get_stock_percent(), 5.5);
    // }

    // #[test]
    // fn pa_update_total_stock_value_bond() {
    //     let mut pa = Helper::get_portfolio_actual();
    //     let mut total_value = 5.0;
    //     let mut stock_value = 3.0;
    //     PortfolioActual::update_total_and_stock_value(
    //         &mut total_value,
    //         &mut stock_value,
    //         &Helper::bond(),
    //         // 11.0,
    //     );
    //     assert_eq!(total_value, 16.0);
    //     assert_eq!(stock_value, 3.0);
    // }

    // #[test]
    // fn pa_update_total_and_stock_value_stock() {
    //     let mut pa = Helper::get_portfolio_actual();
    //     let mut total_value = 5.0;
    //     let mut stock_value = 3.0;
    //     PortfolioActual::update_total_and_stock_value(
    //         &mut total_value,
    //         &mut stock_value,
    //         &Helper::stock(),
    //         // 11.0,
    //     );
    //     assert_eq!(total_value, 16.0);
    //     assert_eq!(stock_value, 14.0);
    // }

    // #[test]
    // fn pa_update_ticker_actual() {
    //     let tic_map = Helper::get_ticker_map();
    //     let tic_actual_map = Helper::get_ticker_actual_map();

    //     let updated = PortfolioActual::update_ta_value(tic_actual_map,
    // &tic_map);

    //     let ta_b = Helper::get_ta_bond();
    //     let t_b = Helper::bond();
    //     let ta_s = Helper::get_ta_stock();
    //     let t_s = Helper::stock();
    //     let updated_stock =
    // updated.tickers_actual.get(&symbol!("stock")).unwrap();
    //     let updated_bond =
    // updated.tickers_actual.get(&symbol!("bond")).unwrap();

    //     let calc_total_val = (t_s.price * ta_s.actual_shares) + (t_b.price *
    // ta_b.actual_shares);     assert_eq!(updated.total, calc_total_val);
    //     let calc_stock_val = t_s.price * ta_s.actual_shares;
    //     assert_eq!(updated.stock, calc_stock_val);
    //     let calc_ta_s_val = t_s.price * ta_s.actual_shares;
    //     // assert_eq!(updated_stock.aavv, calc_ta_s_val);
    //     let calc_ta_b_val = t_b.price * ta_b.actual_shares;
    //     // assert_eq!(updated_bond.aavv, calc_ta_b_val);
    // }

    // #[test]
    // fn pa_update_ticker_actual_percent() {
    //     let tic_actual_map = Helper::get_ticker_actual_map();
    //     let total_val = 330.0;
    //
    // let updated = PortfolioActual::update_ta_percent(tic_actual_map,
    // &total_val);
    //
    // let updated_stock = updated.get(&symbol!("stock")).unwrap();
    // let updated_bond = updated.get(&symbol!("bond")).unwrap();
    // let ta_b = Helper::get_ta_bond();
    // let ta_s = Helper::get_ta_stock();
    //
    // let calc_ta_s_per = (ta_s.aavv / total_val) * 100.0;
    // let calc_ta_s_per = ((calc_ta_s_per) * 100.00).round() / 100.00;
    // assert_eq!(updated_stock.actual_percent, calc_ta_s_per);
    // let calc_ta_b_per = (ta_b.aavv / total_val) * 100.0;
    // let calc_ta_b_per = ((calc_ta_b_per) * 100.00).round() / 100.00;
    // assert_eq!(updated_bond.actual_percent, calc_ta_b_per);
    // }

    // #[test]
    // fn pa_buy_share() {
    //     let sym = symbol!("stock");
    //     let mut pa = Helper::get_portfolio_actual();
    //     let tickers = Helper::get_ticker_map();
    //     let orig_shares = pa.get_ta(&sym).actual_shares;

    //     let updated_pa = pa.buy_share(&sym, 2.5, &tickers);

    //     assert_eq!(updated_pa.get_ta(&sym).actual_shares, orig_shares + 2.5);
    // }

    // ==============================
    // Helper
    // ===============================
    struct Helper {}
    impl Helper {
        // fn stock() -> Ticker {
        //     Ticker {
        //         symbol: symbol!("stock"),
        //         fee: 0.04,
        //         price: 20.0,
        //         kind: InvestmentKind::Stock,
        //     }
        // }

        // fn bond() -> Ticker {
        //     Ticker {
        //         symbol: symbol!("bond"),
        //         fee: 0.04,
        //         price: 10.0,
        //         kind: InvestmentKind::Bond,
        //     }
        // }

        //         fn get_ta_stock() -> TickerActual {
        //             TickerActual {
        //                 symbol: symbol!("stock"),
        //                 // aavv: 5.0,
        //                 actual_shares: 1.5,
        //                 // actual_percent: 2.6,
        //             }
        //         }

        //         fn get_ta_bond() -> TickerActual {
        //             TickerActual {
        //                 symbol: symbol!("bond"),
        //                 // aavv: 10.0,
        //                 actual_shares: 2.0,
        //                 // actual_percent: 5.4,
        //             }
        //         }

        // fn get_ticker_map() -> HashMap<TickerId, Ticker> {
        //     let t1 = Helper::stock();
        //     let t2 = Helper::bond();
        //     let mut map = HashMap::new();
        //     map.insert(t1.symbol.clone(), t1);
        //     map.insert(t2.symbol.clone(), t2);
        //     map
        // }

        // fn get_ticker_actual_map() -> HashMap<TickerId, TickerActual> {
        //     let ta1 = Helper::get_ta_bond();
        //     let ta2 = Helper::get_ta_stock();
        //     let mut map = HashMap::new();
        //     map.insert(ta1.symbol.clone(), ta1);
        //     map.insert(ta2.symbol.clone(), ta2);
        //     map
        // }

        // fn get_portfolio_actual() -> PortfolioActual {
        //     let map = Self::get_ticker_actual_map();
        //     PortfolioActual {
        //         tickers_actual: map,
        //         /* total_value: 200.0,
        //          * actual_stock_percent: 5.5, */
        //     }
        // }
    }
}
