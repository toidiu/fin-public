use crate::ticker::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PortfolioActual {
    pub tickers_actual: HashMap<TickerSymbol, TickerActual>,
    // calculated
    total_value: f32,
    // calculated
    actual_stock_percent: f32,
}

impl PortfolioActual {
    // fixme test!
    pub fn new(
        mut tickers_actual: HashMap<TickerSymbol, TickerActual>,
        tickers: &HashMap<TickerSymbol, Ticker>,
    ) -> Self {
        let mut updated = Self::update_tickers_actual(tickers_actual, tickers);

        // calculate ticker percent
        for x in &mut updated.tickers_actual {
            x.1.update_actual_percent(&updated.total);
        }

        let mut pa = PortfolioActual {
            tickers_actual: updated.tickers_actual,
            total_value: updated.total,
            actual_stock_percent: 0.0,
        };
        pa.calculate_stock_percent(updated.stock);
        pa
    }

    fixme test!
    /// Calculate the price of TickerActual and also calculate
    /// the total value and stock value while we are iterating
    /// over the tickers.
    fn update_tickers_actual(
        mut tickers_actual: HashMap<TickerSymbol, TickerActual>,
        tickers: &HashMap<TickerSymbol, Ticker>,
    ) -> UpdatedTickerActual {
        // calculate global values
        let mut total_value: f32 = 0.0;
        let mut stock_value: f32 = 0.0;

        // calculate ticker value
        for mut x in &mut tickers_actual {
            let ticker = tickers
                .get(&x.0)
                .expect(&format!("add ticker to db: {:?}", &x.0));

            x.1.update_actual_value(ticker.price);
            Self::update_total_and_stock_value(
                &mut total_value,
                &mut stock_value,
                ticker,
                x.1.actual_value,
            );
        }

        UpdatedTickerActual {
            tickers_actual: tickers_actual,
            total: total_value,
            stock: stock_value,
        }
    }

    /// Calculate total value and stock value of the PortfolioActual
    fn update_total_and_stock_value(
        total_value: &mut f32,
        stock_value: &mut f32,
        ticker: &Ticker,
        actual_value: f32,
    ) {
        // calculate total price of portfolio
        *total_value = *total_value + actual_value;
        // calculate stock price of portfolio
        if (ticker.is_stock()) {
            *stock_value = *stock_value + actual_value;
        }
    }

    pub fn get_stock_percent(&self) -> f32 {
        self.actual_stock_percent
    }

    /// Calculate the percent of portfolio that is Stocks.
    fn calculate_stock_percent(&mut self, stock_value: f32) {
        self.actual_stock_percent = (stock_value / self.total_value) * 100.0;
        self.actual_stock_percent = (self.actual_stock_percent * 100.00).round() / 100.00;
    }
}

/// Internal wrapper to bundle return values from a function.
struct UpdatedTickerActual {
    tickers_actual: HashMap<TickerSymbol, TickerActual>,
    total: f32,
    stock: f32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TickerActual {
    pub symbol: TickerSymbol,
    pub actual_shares: f32,
    // calculated
    actual_value: f32,
    // calculated
    actual_percent: f32,
}

impl TickerActual {
    pub fn new(symbol: TickerSymbol, shares: f32) -> Self {
        TickerActual {
            symbol: symbol,
            actual_shares: shares,
            ..Default::default()
        }
    }

    pub fn get_actual_percent(&self) -> f32 {
        self.actual_percent
    }

    fn update_actual_percent(&mut self, total_value: &f32) {
        self.actual_percent = (self.actual_value / total_value) * 100.0;
        self.actual_percent = (self.actual_percent * 100.00).round() / 100.00;
    }

    fn update_actual_value(&mut self, price: f32) {
        self.actual_value = self.actual_shares * price;
        self.actual_value = (self.actual_value * 100.00).round() / 100.00;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ta_new() {
        let ta1 = TickerActual::new(symbol!("vti"), 1.5);
        assert_eq!(ta1.symbol, symbol!("vti"));
        assert_eq!(ta1.actual_shares, 1.5);
        assert_eq!(ta1.actual_value, 0.0);
        assert_eq!(ta1.actual_percent, 0.0);
    }

    #[test]
    fn ta_get_percent() {
        let ta1 = Helper::get_tic_actual_bond();
        assert_eq!(ta1.get_actual_percent(), 22.56);
    }

    #[test]
    fn ta_actual_percent() {
        let mut ta = TickerActual {
            symbol: symbol!("vti"),
            actual_value: 200.0,
            actual_shares: 1.0,
            actual_percent: 0.0,
        };
        ta.update_actual_percent(&600.0);
        assert_eq!(33.33, ta.actual_percent);
    }

    #[test]
    fn ta_actual_value() {
        let mut ta = TickerActual {
            symbol: symbol!("vti"),
            actual_value: 0.0,
            actual_shares: 2.5,
            actual_percent: 0.0,
        };
        ta.update_actual_value(10.5);
        assert_eq!(26.25, ta.actual_value);
    }

    #[test]
    fn pa_calculate_stock_percent() {
        let mut pa = Helper::get_portfolio_actual();
        pa.calculate_stock_percent(15.0);
        assert_eq!(7.5, pa.actual_stock_percent);
    }

    #[test]
    fn pa_get_stock_percent() {
        let pa = Helper::get_portfolio_actual();
        assert_eq!(pa.get_stock_percent(), 5.5);
    }

    #[test]
    fn pa_update_total_stock_value_bond() {
        let mut pa = Helper::get_portfolio_actual();
        let mut total_value = 5.0;
        let mut stock_value = 3.0;
        PortfolioActual::update_total_and_stock_value(
            &mut total_value,
            &mut stock_value,
            &Helper::bond(),
            11.0,
        );
        assert_eq!(total_value, 16.0);
        assert_eq!(stock_value, 3.0);
    }

    #[test]
    fn pa_update_total_and_stock_value_stock() {
        let mut pa = Helper::get_portfolio_actual();
        let mut total_value = 5.0;
        let mut stock_value = 3.0;
        PortfolioActual::update_total_and_stock_value(
            &mut total_value,
            &mut stock_value,
            &Helper::stock(),
            11.0,
        );
        assert_eq!(total_value, 16.0);
        assert_eq!(stock_value, 14.0);
    }

    // ==============================
    // Helper
    // ===============================
    struct Helper {}
    impl Helper {
        fn stock() -> Ticker {
            Ticker {
                symbol: symbol!("stock"),
                fee: 0.04,
                price: 20.0,
                kind: InvestmentKind::Stock,
            }
        }

        fn bond() -> Ticker {
            Ticker {
                symbol: symbol!("bond"),
                fee: 0.04,
                price: 10.0,
                kind: InvestmentKind::Bond,
            }
        }

        fn get_ticker_map() -> HashMap<TickerSymbol, Ticker> {
            let t1 = Helper::stock();
            let t2 = Helper::bond();
            let mut map = HashMap::new();
            map.insert(t1.symbol.clone(), t1);
            map.insert(t2.symbol.clone(), t2);
            map
        }

        fn get_tic_actual_stock() -> TickerActual {
            TickerActual {
                symbol: symbol!("stock"),
                actual_value: 5.0,
                actual_shares: 1.0,
                actual_percent: 22.56,
            }
        }

        fn get_tic_actual_bond() -> TickerActual {
            TickerActual {
                symbol: symbol!("bond"),
                actual_value: 5.0,
                actual_shares: 1.0,
                actual_percent: 22.56,
            }
        }

        fn get_ticker_actual_map() -> HashMap<TickerSymbol, TickerActual> {
            let ta1 = Helper::get_tic_actual_bond();
            let ta2 = Helper::get_tic_actual_stock();
            let mut map = HashMap::new();
            map.insert(ta1.symbol.clone(), ta1);
            map.insert(ta2.symbol.clone(), ta2);
            map
        }

        fn get_portfolio_actual() -> PortfolioActual {
            let map = Self::get_ticker_actual_map();
            PortfolioActual {
                tickers_actual: map,
                total_value: 200.0,
                actual_stock_percent: 5.5,
            }
        }
    }
}
