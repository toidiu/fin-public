#![allow(dead_code, unused)]

use mock_derive::mock;

use crate::portfolio2;
use crate::ticker::*;

#[mock]
pub trait TickerDatabase {
    fn get_ticker(&self, symbol: &TickerSymbol) -> Option<Ticker>;
    fn get_tickers(&self) -> Vec<Ticker>;
    fn get_goal(&self) -> Vec<portfolio2::TickerGoal>;
    fn get_actual(&self) -> Vec<portfolio2::TickerActual>;

    fn is_stock(&self, symbol: &TickerSymbol) -> bool {
        let ticker = self.get_ticker(&symbol);
        match ticker {
            Some(t) => InvestmentKind::Stock.eq(&t.kind),
            None => false
        }
    }

    // // FIXME switch to this
    // fn is_stock(ticker: &Ticker, symbol: &TickerSymbol) -> bool {
    //     InvestmentKind::Stock.eq(&ticker.kind)
    // }
}

pub struct DefaultTickerDatabase {}

impl TickerDatabase for DefaultTickerDatabase {
    fn get_ticker(&self, symbol: &TickerSymbol) -> Option<Ticker> {
        let tickers = self.get_tickers();
        let d: Vec<Ticker> = tickers
            .into_iter()
            .filter(|x| x.symbol.eq(&symbol))
            .collect();
        d.first().cloned()
    }

    fn get_tickers(&self) -> Vec<Ticker> {
        let vti = Ticker {
            symbol: TickerSymbol("vti".to_owned()),
            fee: 0.04,
            price: 150.0,
            kind: InvestmentKind::Stock,
        };
        let vtv = Ticker {
            symbol: TickerSymbol("vtv".to_owned()),
            fee: 0.05,
            price: 111.0,
            kind: InvestmentKind::Stock,
        };
        let vwob = Ticker {
            symbol: TickerSymbol("vwob".to_owned()),
            fee: 0.32,
            price: 75.0,
            kind: InvestmentKind::Bond,
        };
        vec![vti, vtv, vwob, ]
    }

    fn get_goal(&self) -> Vec<portfolio2::TickerGoal> {
        let g_vti = portfolio2::TickerGoal {
            symbol: TickerSymbol("vti".to_owned()),
            goal_percent: 20.0,
        };
        let g_vtv = portfolio2::TickerGoal {
            symbol: TickerSymbol("vtv".to_owned()),
            goal_percent: 12.0,
        };
        // let g_voe = portfolio2::TickerGoal {
        //     symbol: TickerSymbol("voe".to_owned()),
        //     goal_percent: 20.0,
        // };
        // let g_vbr = portfolio2::TickerGoal {
        //     symbol: TickerSymbol("vbr".to_owned()),
        //     goal_percent: 20.0,
        // };
        // let g_vea = portfolio2::TickerGoal {
        //     symbol: TickerSymbol("vea".to_owned()),
        //     goal_percent: 20.0,
        // };
        // let g_vwo = portfolio2::TickerGoal {
        //     symbol: TickerSymbol("vwo".to_owned()),
        //     goal_percent: 20.0,
        // };
        // let g_vtip = portfolio2::TickerGoal {
        //     symbol: TickerSymbol("vtip".to_owned()),
        //     goal_percent: 20.0,
        // };
        // let g_agg = portfolio2::TickerGoal {
        //     symbol: TickerSymbol("agg".to_owned()),
        //     goal_percent: 20.0,
        // };
        // let g_mub = portfolio2::TickerGoal {
        //     symbol: TickerSymbol("mub".to_owned()),
        //     goal_percent: 20.0,
        // };
        // let g_bndx = portfolio2::TickerGoal {
        //     symbol: TickerSymbol("bndx".to_owned()),
        //     goal_percent: 20.0,
        // };
        let g_vwob = portfolio2::TickerGoal {
            symbol: TickerSymbol("vwob".to_owned()),
            goal_percent: 20.0,
        };
        vec![
            g_vti, g_vtv, //g_voe, g_vbr, g_vea, g_vwo, g_vtip, g_agg, g_mub, g_bndx,
            g_vwob,
        ]
    }

    fn get_actual(&self) -> Vec<portfolio2::TickerActual> {
        let a_vti = portfolio2::TickerActual {
            symbol: TickerSymbol("vti".to_owned()),
            actual_value: 300.0,
            actual_shares: 2,
            actual_percent: 0.0,
        };
        let a_vtv = portfolio2::TickerActual {
            symbol: TickerSymbol("vtv".to_owned()),
            actual_value: 300.0,
            actual_shares: 2,
            actual_percent: 0.0,
        };
        // let a_voe = portfolio2::TickerActual {
        //     symbol: TickerSymbol("voe".to_owned()),
        //     actual_value: 300.0,
        //     actual_shares: 2,
        //     actual_percent: 0.0,
        // };
        // let a_vbr = portfolio2::TickerActual {
        //     symbol: TickerSymbol("vbr".to_owned()),
        //     actual_value: 300.0,
        //     actual_shares: 2,
        //     actual_percent: 0.0,
        // };
        // let a_vea = portfolio2::TickerActual {
        //     symbol: TickerSymbol("vea".to_owned()),
        //     actual_value: 300.0,
        //     actual_shares: 2,
        //     actual_percent: 0.0,
        // };
        // let a_vwo = portfolio2::TickerActual {
        //     symbol: TickerSymbol("vwo".to_owned()),
        //     actual_value: 300.0,
        //     actual_shares: 2,
        //     actual_percent: 0.0,
        // };
        // let a_vtip = portfolio2::TickerActual {
        //     symbol: TickerSymbol("vtip".to_owned()),
        //     actual_value: 300.0,
        //     actual_shares: 2,
        //     actual_percent: 0.0,
        // };
        // let a_agg = portfolio2::TickerActual {
        //     symbol: TickerSymbol("agg".to_owned()),
        //     actual_value: 300.0,
        //     actual_shares: 2,
        //     actual_percent: 0.0,
        // };
        // let a_mub = portfolio2::TickerActual {
        //     symbol: TickerSymbol("mub".to_owned()),
        //     actual_value: 300.0,
        //     actual_shares: 2,
        //     actual_percent: 0.0,
        // };
        // let a_bndx = portfolio2::TickerActual {
        //     symbol: TickerSymbol("bndx".to_owned()),
        //     actual_value: 300.0,
        //     actual_shares: 2,
        //     actual_percent: 0.0,
        // };
        let a_vwob = portfolio2::TickerActual {
            symbol: TickerSymbol("vwob".to_owned()),
            actual_value: 300.0,
            actual_shares: 2,
            actual_percent: 0.0,
        };
        vec![
            a_vti, a_vtv, //a_voe, a_vbr, a_vea, a_vwo, a_vtip, a_agg, a_mub, a_bndx,
            a_vwob,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

        fn stock() -> Ticker {
            Ticker {
                symbol: TickerSymbol("abc".to_owned()),
                fee: 0.04,
                price: 150.0,
                kind: InvestmentKind::Stock,
            }
        }
        fn bond() -> Ticker {
            Ticker {
                symbol: TickerSymbol("mub".to_owned()),
                fee: 0.04,
                price: 150.0,
                kind: InvestmentKind::Bond,
            }
        }

    #[test]
    fn pass_is_stock() {
        // let fallback = DefaultTickerDatabase{};
        let mut mock = MockTickerDatabase::new();
        // mock.set_fallback(fallback);
        let method = mock.method_is_stock()
            .return_result_of(|| {
                println!("=======");
                true
            }
        );
        mock.set_is_stock(method);
        // let method = mock.method_get_tickers()
        //     .return_result_of(|| {
        //         println!("=======");
        //         let stock = stock();
        //         let bond = bond();
        //         vec![stock, bond]
        //     }
        // );
        // mock.set_get_tickers(method);

        let f = mock.is_stock(&TickerSymbol("abc".to_owned()));
        assert!(f);
    }

    #[test]
    fn fail_is_stock() {
        let fallback = DefaultTickerDatabase{};
        let mut mock = MockTickerDatabase::new();
        // mock.set_fallback(fallback);
        mock.method_get_tickers()
            .return_result_of(|| {
                let stock = stock();
                let bond = bond();
                vec![stock, bond]
            }
        );

        let f = mock.is_stock(&TickerSymbol("mub".to_owned()));
        assert!(!f);
    }
}

