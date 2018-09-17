#![allow(dead_code, unused)]

use mock_derive::mock;

use crate::portfolio;
use crate::ticker::*;
use std::collections::HashMap;

#[mock]
pub trait TickerDatabase {
    fn get_tickers(&self) -> HashMap<TickerSymbol, Ticker>;
    fn get_goal(&self) -> HashMap<TickerSymbol, portfolio::TickerGoal>;
    fn get_actual(&self) -> HashMap<TickerSymbol, portfolio::TickerActual>;
}

pub struct DefaultTickerDatabase {}

impl TickerDatabase for DefaultTickerDatabase {
    fn get_tickers(&self) -> HashMap<TickerSymbol, Ticker> {
        let vti = Ticker {
            symbol: symbol!("vti"),
            fee: 0.04,
            price: 150.0,
            kind: InvestmentKind::Stock,
        };
        let vtv = Ticker {
            symbol: symbol!("vtv"),
            fee: 0.05,
            price: 111.0,
            kind: InvestmentKind::Stock,
        };
        let vwob = Ticker {
            symbol: symbol!("vwob"),
            fee: 0.32,
            price: 75.0,
            kind: InvestmentKind::Bond,
        };

        let voe = Ticker {
            symbol: symbol!("voe"),
            fee: 0.07,
            price: 115.0,
            kind: InvestmentKind::Stock,
        };

        let vbr = Ticker {
            symbol: symbol!("vbr"),
            fee: 0.07,
            price: 142.0,
            kind: InvestmentKind::Stock,
        };

        let vea = Ticker {
            symbol: symbol!("vea"),
            fee: 0.07,
            price: 43.0,
            kind: InvestmentKind::Stock,
        };

        let vwo = Ticker {
            symbol: symbol!("vwo"),
            fee: 0.14,
            price: 43.0,
            kind: InvestmentKind::Stock,
        };

        let vtip = Ticker {
            symbol: symbol!("vtip"),
            fee: 0.06,
            price: 49.0,
            kind: InvestmentKind::Bond,
        };

        let agg = Ticker {
            symbol: symbol!("agg"),
            fee: 0.05,
            price: 106.0,
            kind: InvestmentKind::Bond,
        };

        let mub = Ticker {
            symbol: symbol!("mub"),
            fee: 0.07,
            price: 109.0,
            kind: InvestmentKind::Bond,
        };

        let bndx = Ticker {
            symbol: symbol!("bndx"),
            fee: 0.11,
            price: 54.0,
            kind: InvestmentKind::Bond,
        };

        let vwob = Ticker {
            symbol: symbol!("vwob"),
            fee: 0.32,
            price: 75.0,
            kind: InvestmentKind::Bond,
        };

        let v = vec![vti, vtv, voe, vbr, vea, vwo, vtip, agg, mub, bndx, vwob];

        // create a map
        let mut map: HashMap<TickerSymbol, Ticker> = HashMap::new();
        for x in v {
            map.insert(x.symbol.clone(), x);
        }
        map
    }

    fn get_goal(&self) -> HashMap<TickerSymbol, portfolio::TickerGoal> {
        let g_vti = portfolio::TickerGoal {
            symbol: symbol!("vti"),
            goal_percent: 20.0,
            order: 1,
        };
        let g_vtv = portfolio::TickerGoal {
            symbol: symbol!("vtv"),
            goal_percent: 6.0,
            order: 2,
        };
        let g_voe = portfolio::TickerGoal {
            symbol: symbol!("voe"),
            goal_percent: 4.0,
            order: 3,
        };
        let g_vbr = portfolio::TickerGoal {
            symbol: symbol!("vbr"),
            goal_percent: 3.0,
            order: 4,
        };
        let g_vea = portfolio::TickerGoal {
            symbol: symbol!("vea"),
            goal_percent: 15.0,
            order: 5,
        };
        let g_vwo = portfolio::TickerGoal {
            symbol: symbol!("vwo"),
            goal_percent: 10.0,
            order: 6,
        };
        let g_vtip = portfolio::TickerGoal {
            symbol: symbol!("vtip"),
            goal_percent: 3.0,
            order: 7,
        };
        let g_agg = portfolio::TickerGoal {
            symbol: symbol!("agg"),
            goal_percent: 4.0,
            order: 8,
        };
        let g_mub = portfolio::TickerGoal {
            symbol: symbol!("mub"),
            goal_percent: 14.0,
            order: 9,
        };
        let g_bndx = portfolio::TickerGoal {
            symbol: symbol!("bndx"),
            goal_percent: 12.0,
            order: 10,
        };
        let g_vwob = portfolio::TickerGoal {
            symbol: symbol!("vwob"),
            goal_percent: 9.0,
            order: 11,
        };
        let v = vec![
            g_vti, g_vtv, g_voe, g_vbr, g_vea, g_vwo, g_vtip, g_agg, g_mub, g_bndx, g_vwob,
        ];

        // create a map
        let mut map: HashMap<TickerSymbol, portfolio::TickerGoal> = HashMap::new();
        for x in v {
            map.insert(x.symbol.clone(), x);
        }
        map
    }

    fn get_actual(&self) -> HashMap<TickerSymbol, portfolio::TickerActual> {
        let a_vti = portfolio::TickerActual::new(symbol!("vti"), 2.0);
        let a_vtv = portfolio::TickerActual::new(symbol!("vtv"), 1.0);
        let a_voe = portfolio::TickerActual::new(symbol!("voe"), 1.0);
        let a_vbr = portfolio::TickerActual::new(symbol!("vbr"), 1.0);
        let a_vea = portfolio::TickerActual::new(symbol!("vea"), 3.0);
        let a_vwo = portfolio::TickerActual::new(symbol!("vwo"), 2.0);
        let a_vtip = portfolio::TickerActual::new(symbol!("vtip"), 1.0);
        let a_agg = portfolio::TickerActual::new(symbol!("agg"), 1.0);
        let a_mub = portfolio::TickerActual::new(symbol!("mub"), 1.0);
        let a_bndx = portfolio::TickerActual::new(symbol!("bndx"), 2.0);
        let a_vwob = portfolio::TickerActual::new(symbol!("vwob"), 1.0);
        let v = vec![
            a_vti, a_vtv, a_voe, a_vbr, a_vea, a_vwo, a_vtip, a_agg, a_mub, a_bndx, a_vwob,
        ];

        // create a map
        let mut map: HashMap<TickerSymbol, portfolio::TickerActual> = HashMap::new();
        for x in v {
            map.insert(x.symbol.clone(), x);
        }
        map
    }
}
