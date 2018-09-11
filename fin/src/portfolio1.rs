use crate::ticker::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    pub name: String,
    pub current_detail: PortfolioDetails,
    pub past_detail: Vec<PortfolioDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioDetails {
    pub stocks: Vec<TickerStatus>,
    pub bonds: Vec<TickerStatus>,
    pub goal_stock_percent: f32,

    // calculated
    pub total_value: f32,
    // calculated
    pub current_stock_percent: f32,
    pub deviation_percent: f32,
}

impl PortfolioDetails {
    pub fn new(
        stocks: Vec<TickerStatus>,
        bonds: Vec<TickerStatus>,
        goal_stock_percent: f32,
        deviation_percent: f32,
    ) -> Self {
        PortfolioDetails {
            stocks: stocks,
            bonds: bonds,
            goal_stock_percent: goal_stock_percent,
            current_stock_percent: 0.0,
            total_value: 0.0,
            deviation_percent: deviation_percent,
        }
        // calculate total value before others
        .set_total_value()
        .set_stock_percent()
    }

    pub fn get_stock_value(&self) -> f32 {
        let s: f32 = self.stocks.iter().map(|s| s.goal.current_value).sum();
        s.into()
    }

    pub fn get_bond_value(&self) -> f32 {
        let b: f32 = self.bonds.iter().map(|s| s.goal.current_value).sum();
        b.into()
    }

    fn set_stock_percent(mut self) -> Self {
        let stocks = self.get_stock_value();
        self.current_stock_percent = (stocks / self.total_value) * 100.0;
        self
    }

    fn set_total_value(mut self) -> Self {
        let s: f32 = self.stocks.iter().map(|s| s.goal.current_value).sum();
        let b: f32 = self.bonds.iter().map(|s| s.goal.current_value).sum();

        self.total_value = (s + b).into();
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerGoal {
    // #[serde(skip)]
    // pub symbol: TickerSymbol,
    #[serde(rename = "goalPercent")]
    pub goal_percent: f32,

    // TODO calculate this
    #[serde(rename = "currentPercent")]
    pub current_percent: f32,
    #[serde(rename = "currentValue")]
    pub current_value: f32,

    // either keep this here or maybe have another table that records
    // the transactions we have made. we can then calculate what the
    // current shares are based on that table. (table could be
    // expensive)
    #[serde(rename = "currentShares")]
    pub current_shares: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerStatus {
    #[serde(flatten)]
    pub ticker: Ticker,
    #[serde(flatten)]
    pub goal: TickerGoal,
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
