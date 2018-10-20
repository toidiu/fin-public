use crate::portfolio;
use crate::ticker;
use crate::ticker::TickerSymbol;
use std::collections::HashMap;

#[derive(Debug)]
pub struct UserData {
    pub id: i64,
    pub username: String,
}

#[derive(Debug)]
pub struct TickerData {
    pub id: i64,
    pub symbol: String,
    pub fk_exchange: String,
    pub fee: f32,
    pub kind: String,
}

impl TickerData {
    pub fn to_ticker(self, price: f32) -> ticker::Ticker {
        ticker::Ticker {
            id: self.id,
            symbol: symbol!(self.symbol),
            exchange: self.fk_exchange,
            fee: self.fee,
            price: price,
            kind: {
                if (self.kind == "STOCK") {
                    ticker::InvestmentKind::Stock
                } else if (self.kind == "BOND") {
                    ticker::InvestmentKind::Bond
                } else {
                    panic!("expected either STOCK or BOND")
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct PortGoalData {
    pub id: i64,
    pub stock_per: f32,
    pub deviation: f32,
    pub name: String,
    pub description: Option<String>,
}

impl PortGoalData {
    fn to_port_goal(self) -> portfolio::PortfolioGoal {
        portfolio::PortfolioGoal {
            id: self.id,
            tickers_goal: HashMap::new(),
            goal_stock_percent: self.stock_per,
            deviation_percent: self.deviation,
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(Debug)]
pub struct TickerGoalData {
    pub fk_port_g_id: i64,
    pub fk_tic_id: i64,
    pub goal_per: f32,
    pub ord: i32,
}

impl TickerGoalData {
    pub fn to_tic_goal(self) -> portfolio::TickerGoal {
        portfolio::TickerGoal {
            port_goal_id: self.fk_port_g_id,
            ticker_id: self.fk_tic_id,
            goal_percent: self.goal_per,
            order: self.ord,
        }
    }
}

#[derive(Debug)]
pub struct TickerActualData {
    pub id: i64,
    pub fk_user_id: i64,
    pub fk_port_g_id: i64,
    pub fk_tic_id: i64,
    pub actual_shares: f32,
}

impl TickerActualData {
    pub fn to_tic_actual(self) -> portfolio::TickerActual {
        portfolio::TickerActual {
            id: self.id,
            user_id: self.fk_user_id,
            port_goal_id: self.fk_port_g_id,
            ticker_id: self.fk_tic_id,
            actual_shares: self.actual_shares,
        }
    }
}
