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
        // calculate global values
        let mut total_value: f32 = 0.0;
        let mut stock_value: f32 = 0.0;

        // calculate ticker value
        for mut x in &mut tickers_actual {
            let ticker = tickers
                .get(x.0)
                .expect(&format!("add ticker to db: {:?}", &x.0));

            x.1.update_actual_value(ticker.price);

            // calculate total price of portfolio
            total_value = total_value + x.1.actual_value;
            // calculate stock price of portfolio
            if (ticker.is_stock()) {
                stock_value = stock_value + x.1.actual_value;
            }
        }

        // calculate ticker percent
        for mut x in &mut tickers_actual {
            x.1.update_actual_percent(total_value);
        }

        let mut pa = PortfolioActual {
            tickers_actual: tickers_actual,
            total_value: total_value,
            actual_stock_percent: 0.0,
        };
        pa.calculate_stock_percent(stock_value);
        pa
    }

    // fixme test!
    pub fn get_stock_percent(&self) -> f32 {
        self.actual_stock_percent
    }

    fn calculate_stock_percent(&mut self, stock_value: f32) {
        self.actual_stock_percent = (stock_value / self.total_value) * 100.0;
        self.actual_stock_percent = (self.actual_stock_percent * 100.00).round() / 100.00;
    }
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
    // fixme test!
    pub fn new(symbol: TickerSymbol, shares: f32) -> Self {
        TickerActual {
            symbol: symbol,
            actual_shares: shares,
            ..Default::default()
        }
    }

    // fixme test!
    pub fn get_actual_percent(&self) -> f32 {
        self.actual_percent
    }

    fn update_actual_percent(&mut self, total_value: f32) {
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

    struct Helper {}
    impl Helper {
        // fn get_ticker_map() -> HashMap<TickerSymbol, Ticker> {
        //     let t1 = Ticker {
        //         symbol: symbol!("vti"),
        //         fee: 0.04,
        //         price: 150.0,
        //         kind: InvestmentKind::Stock,
        //     };
        //     let t2 = Ticker {
        //         symbol: symbol!("vtv"),
        //         fee: 0.05,
        //         price: 111.0,
        //         kind: InvestmentKind::Stock,
        //     };

        //     let mut map = HashMap::new();
        //     map.insert(t1.symbol.clone(), t1);
        //     map.insert(t2.symbol.clone(), t2);
        //     map
        // }

        fn get_ticker_actual_map() -> HashMap<TickerSymbol, TickerActual> {
            let ta1 = TickerActual {
                symbol: symbol!("vti"),
                actual_value: 5.0,
                actual_shares: 1.0,
                actual_percent: 22.56,
            };
            let ta2 = TickerActual {
                symbol: symbol!("vtv"),
                actual_value: 10.0,
                actual_shares: 1.0,
                actual_percent: 8.35,
            };

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
                actual_stock_percent: 0.0,
            }
        }
    }

    #[test]
    fn new_portfolio_actual() {
        // fixme
        assert!(true);
    }

    #[test]
    fn ta_actual_percent() {
        let mut ta = TickerActual {
            symbol: symbol!("vti"),
            actual_value: 200.0,
            actual_shares: 1.0,
            actual_percent: 0.0,
        };
        ta.update_actual_percent(600.0);
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
    fn pa_stock_percent() {
        let mut pa = Helper::get_portfolio_actual();
        pa.calculate_stock_percent(15.0);
        assert_eq!(7.5, pa.actual_stock_percent);
    }

}
