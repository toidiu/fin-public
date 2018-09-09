#![allow(dead_code, unused)]

use crate::data;
use crate::ticker::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    pub name: String,
    pub current_detail: PortfolioDetail,
    pub past_detail: Vec<PortfolioDetail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioDetail {
    goal: PortfolioGoal,
    actual: PortfolioActual,
}

#[derive(Serialize, Deserialize, Debug)]
struct PortfolioGoal {
    pub tickers: Vec<TickerGoal>,
    pub goal_stock_percent: f32,
    pub deviation_percent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PortfolioActual {
    pub tickers: Vec<TickerActual>,
    // calculated
    pub total_value: f32,
    // calculated
    pub actual_stock_percent: f32,
}
impl PortfolioActual {
    pub fn new(tickers: Vec<TickerActual>) -> Self {
        PortfolioActual {
            tickers: data::get_actual(),
            total_value: 0.0,
            actual_stock_percent: 0.0,
        }.calculate_total()
        .calculate_stock_percent()
    }
    fn calculate_total(mut self) -> Self {
        self.total_value = self.tickers.iter().map(|x| x.actual_value).sum();
        self
    }
    fn calculate_stock_percent(mut self) -> Self {
        let stock_value: f32 = self
            .tickers
            .iter()
            .filter(|ref x| data::is_stock(&x.symbol))
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
    // fn new(symbol: TickerSymbol, actual_value: f32, actual_shares: u32, total_value: f32) -> Self {
    //     TickerActual {
    //         symbol: symbol,
    //         actual_value: actual_value,
    //         actual_shares: actual_shares,
    //         actual_percent: 0.0,
    //     }.calculate_actual_percent(total_value)
    // }

    pub fn update_actual_percent(mut self, total_value: f32) -> Self {
        self.actual_percent = (self.actual_value / total_value) * 100.0;
        self
    }
}

pub fn get_data() -> Portfolio {
    let pg = PortfolioGoal {
        tickers: data::get_goal(),
        goal_stock_percent: 58.0,
        deviation_percent: 5.0,
    };

    let actual_tickers = {
        let mut actual = data::get_actual();
        let total_value = actual.iter().map(|x| x.actual_value).sum();
        actual
            .into_iter()
            .map(|x| x.update_actual_percent(total_value))
            .collect()
    };

    let pa = PortfolioActual::new(actual_tickers);

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
