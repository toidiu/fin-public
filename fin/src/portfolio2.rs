#![allow(dead_code, unused)]

use crate::data;
use crate::ticker::*;
use std::num;

#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    pub name: String,
    pub current_detail: PortfolioDetail,
    pub past_detail: Vec<PortfolioDetail>,
}

pub enum Action {
    BuyStock,
    BuyBond,
    BuyEither,
}

impl Portfolio {
    pub fn is_stock_per_greater(&self) -> Action {
        let actual_per = self.current_detail.actual.actual_stock_percent;
        let goal_per = self.current_detail.goal.goal_stock_percent;
        let deviation = self.current_detail.goal.deviation_percent;

        let diff = goal_per - actual_per;
        if ((diff < 0.0) && diff.abs() > deviation) {
            // If gS%-aS% is - and abs val above q% then buy bonds
            Action::BuyStock
        } else if (diff > 0.0 && diff > deviation) {
            // If gS%-aS% is + and above q% then buy stocks
            Action::BuyBond
        } else {
            // else buy stock or bond
            Action::BuyEither
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioDetail {
    goal: PortfolioGoal,
    actual: PortfolioActual,
}

#[derive(Serialize, Deserialize, Debug)]
struct PortfolioGoal {
    tickers: Vec<TickerGoal>,
    goal_stock_percent: f32,
    deviation_percent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PortfolioActual {
    tickers: Vec<TickerActual>,
    // calculated
    total_value: f32,
    // calculated
    actual_stock_percent: f32,
}

impl PortfolioActual {
    fn new<T: data::TickerDatabase>(tickers: Vec<TickerActual>, db: T) -> Self {
        PortfolioActual {
            tickers: tickers,
            total_value: 0.0,
            actual_stock_percent: 0.0,
        }.calculate_total()
        .calculate_stock_percent(db)
    }
    fn calculate_total(mut self) -> Self {
        self.total_value = self.tickers.iter().map(|x| x.actual_value).sum();
        self
    }
    fn calculate_stock_percent<T: data::TickerDatabase>(mut self, db: T) -> Self {
        let stock_value: f32 = self
            .tickers
            .iter()
            .filter(|ref x| db.get_ticker(&x.symbol).is_stock())
            .map(|x| x.actual_value)
            .sum();
        self.actual_stock_percent = (stock_value / self.total_value) * 100.0;
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerGoal {
    pub symbol: TickerSymbol,
    pub goal_percent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerActual {
    pub symbol: TickerSymbol,
    pub actual_value: f32,
    pub actual_shares: u32,
    // calculated
    pub actual_percent: f32,
}

impl TickerActual {
    pub fn update_actual_percent(mut self, total_value: f32) -> Self {
        self.actual_percent = (self.actual_value / total_value) * 100.0;
        self
    }
}

pub fn get_data<T: data::TickerDatabase>(db: T) -> Portfolio {
    let pg = PortfolioGoal {
        tickers: db.get_goal(),
        goal_stock_percent: 58.0,
        deviation_percent: 5.0,
    };

    let actual_tickers = {
        let mut actual = db.get_actual();
        let total_value = actual.iter().map(|x| x.actual_value).sum();
        actual
            .into_iter()
            .map(|x| x.update_actual_percent(total_value))
            .collect()
    };

    let pa = PortfolioActual::new(actual_tickers, db);

    let pd = PortfolioDetail {
        goal: pg,
        actual: pa,
    };
    let p = Portfolio {
        name: "my portfolio".to_owned(),
        current_detail: pd,
        past_detail: vec![],
    };
    p
}
