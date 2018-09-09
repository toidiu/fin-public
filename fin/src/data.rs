#![allow(dead_code, unused)]

use crate::portfolio1::*;
use crate::portfolio2;
use crate::ticker::*;

pub fn is_stock(symbol: &TickerSymbol) -> bool {
    let ticker = get_ticker(&symbol);
    InvestmentKind::Stock.eq(&ticker.kind)
}

pub fn get_goal() -> Vec<portfolio2::TickerGoal> {
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

pub fn get_actual() -> Vec<portfolio2::TickerActual> {
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

pub fn get_ticker(symbol: &TickerSymbol) -> Ticker {
    let tickers = get_tickers();
    let d: Vec<Ticker> = tickers
        .into_iter()
        .filter(|x|
            x.symbol.eq(&symbol)
        )
        .collect();
    d.first().unwrap().clone()
}

pub fn get_tickers() -> Vec<Ticker> {
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
    // let voe = Ticker {
    //     symbol: TickerSymbol("voe".to_owned()),
    //     fee: 0.07,
    //     price: 115.0,
    //     kind: InvestmentKind::Stock,
    // };
    // let vbr = Ticker {
    //     symbol: TickerSymbol("vbr".to_owned()),
    //     fee: 0.07,
    //     price: 142.0,
    //     kind: InvestmentKind::Stock,
    // };
    // let vea = Ticker {
    //     symbol: TickerSymbol("vea".to_owned()),
    //     fee: 0.07,
    //     price: 43.0,
    //     kind: InvestmentKind::Stock,
    // };
    // let vwo = Ticker {
    //     symbol: TickerSymbol("vwo".to_owned()),
    //     fee: 0.14,
    //     price: 43.0,
    //     kind: InvestmentKind::Stock,
    // };
    // let vtip = Ticker {
    //     symbol: TickerSymbol("vtip".to_owned()),
    //     fee: 0.06,
    //     price: 49.0,
    //     kind: InvestmentKind::Bond,
    // };
    // let agg = Ticker {
    //     symbol: TickerSymbol("agg".to_owned()),
    //     fee: 0.05,
    //     price: 106.0,
    //     kind: InvestmentKind::Bond,
    // };
    // let mub = Ticker {
    //     symbol: TickerSymbol("mub".to_owned()),
    //     fee: 0.07,
    //     price: 109.0,
    //     kind: InvestmentKind::Bond,
    // };
    // let bndx = Ticker {
    //     symbol: TickerSymbol("bndx".to_owned()),
    //     fee: 0.11,
    //     price: 54.0,
    //     kind: InvestmentKind::Bond,
    // };
    let vwob = Ticker {
        symbol: TickerSymbol("vwob".to_owned()),
        fee: 0.32,
        price: 75.0,
        kind: InvestmentKind::Bond,
    };
    vec![
        vti, vtv, //voe, vbr, vea, vwo, vtip, agg, mub, bndx,
        vwob,
    ]
}

pub fn get_data_portfolio1() -> Portfolio {
    let tg_vti = TickerStatus {
        ticker: Ticker {
            symbol: TickerSymbol("vti".to_owned()),
            fee: 0.04,
            price: 150.0,
            kind: InvestmentKind::Stock,
        },
        goal: TickerGoal {
            // symbol: TickerSymbol("vti".to_owned()),
            goal_percent: 20.0,
            current_percent: 22.56,
            current_value: 300.0,
            current_shares: 1,
        },
    };

    let tg_vtv = TickerStatus {
        ticker: Ticker {
            symbol: TickerSymbol("vtv".to_owned()),
            fee: 0.05,
            price: 111.0,
            kind: InvestmentKind::Stock,
        },
        goal: TickerGoal {
            // symbol: TickerSymbol("vtv".to_owned()),
            goal_percent: 6.0,
            current_percent: 8.35,
            current_value: 111.0,
            current_shares: 1,
        },
    };

    let tg_voe = TickerStatus {
        ticker: Ticker {
            symbol: TickerSymbol("voe".to_owned()),
            fee: 0.07,
            price: 115.0,
            kind: InvestmentKind::Stock,
        },
        goal: TickerGoal {
            // symbol: TickerSymbol("voe".to_owned()),
            goal_percent: 4.0,
            current_percent: 8.65,
            current_value: 115.0,
            current_shares: 1,
        },
    };

    let tg_vbr = TickerStatus {
        ticker: Ticker {
            symbol: TickerSymbol("vbr".to_owned()),
            fee: 0.07,
            price: 142.0,
            kind: InvestmentKind::Stock,
        },
        goal: TickerGoal {
            // symbol: TickerSymbol("vbr".to_owned()),
            goal_percent: 3.0,
            current_percent: 10.68,
            current_value: 142.0,
            current_shares: 1,
        },
    };

    let tg_vea = TickerStatus {
        ticker: Ticker {
            symbol: TickerSymbol("vea".to_owned()),
            fee: 0.07,
            price: 43.0,
            kind: InvestmentKind::Stock,
        },
        goal: TickerGoal {
            // symbol: TickerSymbol("vea".to_owned()),
            goal_percent: 15.0,
            current_percent: 9.70,
            current_value: 129.0,
            current_shares: 3,
        },
    };

    let tg_vwo = TickerStatus {
        ticker: Ticker {
            symbol: TickerSymbol("vwo".to_owned()),
            fee: 0.14,
            price: 43.0,
            kind: InvestmentKind::Stock,
        },
        goal: TickerGoal {
            // symbol: TickerSymbol("vwo".to_owned()),
            goal_percent: 10.0,
            current_percent: 6.47,
            current_value: 86.0,
            current_shares: 2,
        },
    };

    let tg_vtip = TickerStatus {
        ticker: Ticker {
            symbol: TickerSymbol("vtip".to_owned()),
            fee: 0.06,
            price: 49.0,
            kind: InvestmentKind::Bond,
        },
        goal: TickerGoal {
            // symbol: TickerSymbol("vtip".to_owned()),
            goal_percent: 3.0,
            current_percent: 3.68,
            current_value: 49.0,
            current_shares: 1,
        },
    };

    let tg_agg = TickerStatus {
        ticker: Ticker {
            symbol: TickerSymbol("agg".to_owned()),
            fee: 0.05,
            price: 106.0,
            kind: InvestmentKind::Bond,
        },
        goal: TickerGoal {
            // symbol: TickerSymbol("agg".to_owned()),
            goal_percent: 4.0,
            current_percent: 7.97,
            current_value: 106.0,
            current_shares: 1,
        },
    };

    let tg_mub = TickerStatus {
        ticker: Ticker {
            symbol: TickerSymbol("mub".to_owned()),
            fee: 0.07,
            price: 109.0,
            kind: InvestmentKind::Bond,
        },
        goal: TickerGoal {
            // symbol: TickerSymbol("mub".to_owned()),
            goal_percent: 14.0,
            current_percent: 8.2,
            current_value: 109.0,
            current_shares: 1,
        },
    };

    let tg_bndx = TickerStatus {
        ticker: Ticker {
            symbol: TickerSymbol("bndx".to_owned()),
            fee: 0.11,
            price: 54.0,
            kind: InvestmentKind::Bond,
        },
        goal: TickerGoal {
            // symbol: TickerSymbol("bndx".to_owned()),
            goal_percent: 12.0,
            current_percent: 8.12,
            current_value: 108.0,
            current_shares: 2,
        },
    };

    let tg_vwob = TickerStatus {
        ticker: Ticker {
            symbol: TickerSymbol("vwob".to_owned()),
            fee: 0.32,
            price: 75.0,
            kind: InvestmentKind::Bond,
        },
        goal: TickerGoal {
            // symbol: TickerSymbol("vwob".to_owned()),
            goal_percent: 9.0,
            current_percent: 5.64,
            current_value: 75.0,
            current_shares: 1,
        },
    };

    let detail = PortfolioDetails::new(
        vec![tg_vti, tg_vtv, tg_voe, tg_vbr, tg_vea, tg_vwo],
        vec![tg_vtip, tg_agg, tg_mub, tg_bndx, tg_vwob],
        58.0,
        5.0,
    );

    println!("=========");
    println!("total stocks: {}", detail.get_stock_value());
    println!("total bonds: {}", detail.get_bond_value());
    let port = Portfolio {
        name: "my portfolio".to_owned(),
        current_detail: detail,
        past_detail: vec![],
    };

    port
}
