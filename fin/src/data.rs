#![allow(dead_code, unused)]

use mock_derive::mock;

use crate::portfolio2;
use crate::ticker::*;
use std::collections::BTreeMap;

#[mock]
pub trait TickerDatabase {
    fn get_ticker(&self, symbol: &TickerSymbol) -> Ticker;
    fn get_tickers(&self) -> Vec<Ticker>;
    fn get_goal(&self) -> BTreeMap<TickerSymbol, portfolio2::TickerGoal>;
    fn get_actual(&self) -> BTreeMap<TickerSymbol, portfolio2::TickerActual>;
}

pub struct DefaultTickerDatabase {}

impl TickerDatabase for DefaultTickerDatabase {
    fn get_ticker(&self, symbol: &TickerSymbol) -> Ticker {
        let tickers = self.get_tickers();
        let d: Vec<Ticker> = tickers
            .into_iter()
            .filter(|x| x.symbol.eq(&symbol))
            .collect();
        d.first()
            .cloned()
            .expect(&format!("add stock to database: {:?}", symbol))
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

        let voe = Ticker {
            symbol: TickerSymbol("voe".to_owned()),
            fee: 0.07,
            price: 115.0,
            kind: InvestmentKind::Stock,
        };

        let vbr = Ticker {
            symbol: TickerSymbol("vbr".to_owned()),
            fee: 0.07,
            price: 142.0,
            kind: InvestmentKind::Stock,
        };

        let vea = Ticker {
            symbol: TickerSymbol("vea".to_owned()),
            fee: 0.07,
            price: 43.0,
            kind: InvestmentKind::Stock,
        };

        let vwo = Ticker {
            symbol: TickerSymbol("vwo".to_owned()),
            fee: 0.14,
            price: 43.0,
            kind: InvestmentKind::Stock,
        };

        let vtip = Ticker {
            symbol: TickerSymbol("vtip".to_owned()),
            fee: 0.06,
            price: 49.0,
            kind: InvestmentKind::Bond,
        };

        let agg = Ticker {
            symbol: TickerSymbol("agg".to_owned()),
            fee: 0.05,
            price: 106.0,
            kind: InvestmentKind::Bond,
        };

        let mub = Ticker {
            symbol: TickerSymbol("mub".to_owned()),
            fee: 0.07,
            price: 109.0,
            kind: InvestmentKind::Bond,
        };

        let bndx = Ticker {
            symbol: TickerSymbol("bndx".to_owned()),
            fee: 0.11,
            price: 54.0,
            kind: InvestmentKind::Bond,
        };

        let vwob = Ticker {
            symbol: TickerSymbol("vwob".to_owned()),
            fee: 0.32,
            price: 75.0,
            kind: InvestmentKind::Bond,
        };

        vec![vti, vtv, voe, vbr, vea, vwo, vtip, agg, mub, bndx, vwob]
    }

    fn get_goal(&self) -> BTreeMap<TickerSymbol, portfolio2::TickerGoal> {
        let g_vti = portfolio2::TickerGoal {
            symbol: TickerSymbol("vti".to_owned()),
            goal_percent: 20.0,
            order: 1,
        };
        let g_vtv = portfolio2::TickerGoal {
            symbol: TickerSymbol("vtv".to_owned()),
            goal_percent: 6.0,
            order: 2,
        };
        let g_voe = portfolio2::TickerGoal {
            symbol: TickerSymbol("voe".to_owned()),
            goal_percent: 4.0,
            order: 3,
        };
        let g_vbr = portfolio2::TickerGoal {
            symbol: TickerSymbol("vbr".to_owned()),
            goal_percent: 3.0,
            order: 4,
        };
        let g_vea = portfolio2::TickerGoal {
            symbol: TickerSymbol("vea".to_owned()),
            goal_percent: 15.0,
            order: 5,
        };
        let g_vwo = portfolio2::TickerGoal {
            symbol: TickerSymbol("vwo".to_owned()),
            goal_percent: 10.0,
            order: 6,
        };
        let g_vtip = portfolio2::TickerGoal {
            symbol: TickerSymbol("vtip".to_owned()),
            goal_percent: 3.0,
            order: 7,
        };
        let g_agg = portfolio2::TickerGoal {
            symbol: TickerSymbol("agg".to_owned()),
            goal_percent: 4.0,
            order: 8,
        };
        let g_mub = portfolio2::TickerGoal {
            symbol: TickerSymbol("mub".to_owned()),
            goal_percent: 14.0,
            order: 9,
        };
        let g_bndx = portfolio2::TickerGoal {
            symbol: TickerSymbol("bndx".to_owned()),
            goal_percent: 12.0,
            order: 10,
        };
        let g_vwob = portfolio2::TickerGoal {
            symbol: TickerSymbol("vwob".to_owned()),
            goal_percent: 9.0,
            order: 11,
        };
        let v = vec![
            g_vti, g_vtv, g_voe, g_vbr, g_vea, g_vwo, g_vtip, g_agg, g_mub, g_bndx, g_vwob,
        ];

        // create a map
        let mut map: BTreeMap<TickerSymbol, portfolio2::TickerGoal> = BTreeMap::new();
        for x in v {
            map.insert(x.symbol.clone(), x);
        }
        map
    }

    fn get_actual(&self) -> BTreeMap<TickerSymbol, portfolio2::TickerActual> {
        let a_vti = portfolio2::TickerActual {
            symbol: TickerSymbol("vti".to_owned()),
            actual_value: 300.0,
            actual_shares: 1,
            actual_percent: 22.56,
        };
        let a_vtv = portfolio2::TickerActual {
            symbol: TickerSymbol("vtv".to_owned()),
            actual_value: 111.0,
            actual_shares: 1,
            actual_percent: 8.35,
        };
        let a_voe = portfolio2::TickerActual {
            symbol: TickerSymbol("voe".to_owned()),
            actual_value: 115.0,
            actual_shares: 1,
            actual_percent: 8.65,
        };
        let a_vbr = portfolio2::TickerActual {
            symbol: TickerSymbol("vbr".to_owned()),
            actual_value: 142.0,
            actual_shares: 1,
            actual_percent: 10.6,
        };
        let a_vea = portfolio2::TickerActual {
            symbol: TickerSymbol("vea".to_owned()),
            actual_value: 129.0,
            actual_shares: 3,
            actual_percent: 9.7,
        };
        let a_vwo = portfolio2::TickerActual {
            symbol: TickerSymbol("vwo".to_owned()),
            actual_value: 86.0,
            actual_shares: 1,
            actual_percent: 6.4,
        };
        let a_vtip = portfolio2::TickerActual {
            symbol: TickerSymbol("vtip".to_owned()),
            actual_value: 49.0,
            actual_shares: 1,
            actual_percent: 3.6,
        };
        let a_agg = portfolio2::TickerActual {
            symbol: TickerSymbol("agg".to_owned()),
            actual_value: 106.0,
            actual_shares: 1,
            actual_percent: 7.9,
        };
        let a_mub = portfolio2::TickerActual {
            symbol: TickerSymbol("mub".to_owned()),
            actual_value: 109.0,
            actual_shares: 1,
            actual_percent: 8.2,
        };
        let a_bndx = portfolio2::TickerActual {
            symbol: TickerSymbol("bndx".to_owned()),
            actual_value: 108.0,
            actual_shares: 2,
            actual_percent: 8.1,
        };
        let a_vwob = portfolio2::TickerActual {
            symbol: TickerSymbol("vwob".to_owned()),
            actual_value: 75.0,
            actual_shares: 1,
            actual_percent: 5.6,
        };
        let v = vec![
            a_vti, a_vtv, a_voe, a_vbr, a_vea, a_vwo, a_vtip, a_agg, a_mub, a_bndx, a_vwob,
        ];

        // create a map
        let mut map: BTreeMap<TickerSymbol, portfolio2::TickerActual> = BTreeMap::new();
        for x in v {
            map.insert(x.symbol.clone(), x);
        }
        map
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//         fn stock() -> Ticker {
//             Ticker {
//                 symbol: TickerSymbol("abc".to_owned()),
//                 fee: 0.04,
//                 price: 150.0,
//                 kind: InvestmentKind::Stock,
//             }
//         }
//         fn bond() -> Ticker {
//             Ticker {
//                 symbol: TickerSymbol("mub".to_owned()),
//                 fee: 0.04,
//                 price: 150.0,
//                 kind: InvestmentKind::Bond,
//             }
//         }

//     #[test]
//     fn pass_is_stock() {
//         // let fallback = DefaultTickerDatabase{};
//         let mut mock = MockTickerDatabase::new();
//         // mock.set_fallback(fallback);
//         let method = mock.method_is_stock()
//             .return_result_of(|| {
//                 println!("=======");
//                 true
//             }
//         );
//         mock.set_is_stock(method);
//         // let method = mock.method_get_tickers()
//         //     .return_result_of(|| {
//         //         println!("=======");
//         //         let stock = stock();
//         //         let bond = bond();
//         //         vec![stock, bond]
//         //     }
//         // );
//         // mock.set_get_tickers(method);

//         let f = mock.is_stock(&TickerSymbol("abc".to_owned()));
//         assert!(f);
//     }

//     #[test]
//     fn fail_is_stock() {
//         let fallback = DefaultTickerDatabase{};
//         let mut mock = MockTickerDatabase::new();
//         // mock.set_fallback(fallback);
//         mock.method_get_tickers()
//             .return_result_of(|| {
//                 let stock = stock();
//                 let bond = bond();
//                 vec![stock, bond]
//             }
//         );

//         let f = mock.is_stock(&TickerSymbol("mub".to_owned()));
//         assert!(!f);
//     }
// }
