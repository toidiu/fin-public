use crate::portfolio;
use crate::server;
use crate::ticker::{InvestmentKind, Ticker, TickerId, TickerSymbol};
use chrono::prelude::*;
use std::borrow::{Cow, ToOwned};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct UserDataWithPass {
    pub id: i64,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct UserData {
    pub id: i64,
    pub email: String,
}

#[derive(Debug, FromSql, ToSql)]
#[postgres(name = "dom_tic_kind")]
pub struct DomainTicKind(String);

#[derive(Debug, PostgresMapper)]
#[pg_mapper(table = "tickers")]
pub struct TickerData {
    pub id: i64,
    pub symbol: String,
    pub fk_exchange: i32,
    pub fee: f32,
    pub kind: DomainTicKind,
}

impl TickerData {
    pub fn to_ticker(self, price: f64) -> Ticker {
        Ticker::new(
            self.id,
            symbol!(self.symbol),
            self.fk_exchange,
            self.fee,
            price,
            {
                if (self.kind.0 == "STOCK") {
                    InvestmentKind::Stock
                } else if (self.kind.0 == "BOND") {
                    InvestmentKind::Bond
                } else {
                    panic!("expected either STOCK or BOND")
                }
            },
        )
    }
}

#[derive(Debug, PostgresMapper)]
#[pg_mapper(table = "goal_port")]
pub struct GoalPortData {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

impl GoalPortData {
    pub fn to_port_goal(
        self,
        tickers_goal: &HashMap<TickerId, portfolio::GoalTicker>,
        tickers_map: &HashMap<TickerId, Ticker>,
        actual_stock_percent: &f32,
    ) -> portfolio::PortfolioGoal {
        portfolio::PortfolioGoal::new(
            self.id,
            self.name,
            self.description,
            tickers_goal,
            tickers_map,
            actual_stock_percent,
        )
    }
}

#[derive(Debug, PostgresMapper)]
#[pg_mapper(table = "goal_tic")]
pub struct GoalTickerData {
    pub id: i64,
    pub fk_port_g_id: i64,
    pub fk_tic_id: i64,
    pub tic_goal_per: f32, //ticker percent
    pub ord: i32,
}

impl From<GoalTickerData> for portfolio::GoalTicker {
    fn from(item: GoalTickerData) -> portfolio::GoalTicker {
        portfolio::GoalTicker::new(
            item.id,
            item.fk_port_g_id,
            item.fk_tic_id,
            item.tic_goal_per,
            item.ord,
        )
    }
}

#[derive(Debug, PostgresMapper)]
#[pg_mapper(table = "")]
pub struct GoalTickerDetailData {
    pub fk_port_g_id: i64,
    pub fk_tic_id: i64,
    pub tic_goal_per: f32,
    pub ord: i32,
    pub symbol: String,
}

impl GoalTickerDetailData {
    pub fn to_tic_goal(self) -> portfolio::TickerGoalDetailed {
        portfolio::TickerGoalDetailed {
            port_goal_id: self.fk_port_g_id,
            ticker_id: self.fk_tic_id,
            goal_percent: self.tic_goal_per,
            order: self.ord,
            symbol: symbol!(self.symbol),
        }
    }
}

pub struct ActualFullData {
    pub port: ActualPortData,
    pub tics: Vec<ActualTickerData>,
}

#[derive(Debug, PostgresMapper)]
#[pg_mapper(table = "")]
pub struct ActualPortDetailData {
    pub id: i64,
    pub fk_user_id: i64,
    pub fk_port_g_id: i64,
    pub stock_percent: f32,
    pub deviation: f32,
    pub version: i32,
    pub last_updated: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
}

impl ActualPortDetailData {
    pub fn to_resp(self) -> server::PortfolioActualDetailResp {
        server::PortfolioActualDetailResp {
            id: self.id,
            fk_user_id: self.fk_user_id,
            fk_port_g_id: self.fk_port_g_id,
            stock_percent: self.stock_percent,
            deviation: self.deviation,
            version: self.version,
            last_updated: self.last_updated,
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(Debug, PostgresMapper)]
#[pg_mapper(table = "actual_port")]
pub struct ActualPortData {
    pub id: i64,
    pub fk_user_id: i64,
    pub fk_port_g_id: i64,
    pub stock_percent: f32,
    pub deviation: f32,
    pub version: i32,
    pub last_updated: DateTime<Utc>,
}

impl ActualPortData {
    pub fn to_actual_port(
        self,
        tickers_actual: &HashMap<TickerId, portfolio::TickerActual>,
    ) -> portfolio::PortfolioActual {
        portfolio::PortfolioActual::new(
            self.id,
            self.fk_user_id,
            self.fk_port_g_id,
            self.stock_percent,
            self.deviation,
            self.version,
            self.last_updated,
            tickers_actual.clone(),
        )
    }

    pub fn to_actual_port_resp(
        self,
        tickers_actual: &Vec<portfolio::TickerActual>,
    ) -> server::PortfolioActualResp {
        server::PortfolioActualResp {
            id: self.id,
            fk_user_id: self.fk_user_id,
            fk_port_g_id: self.fk_port_g_id,
            stock_percent: self.stock_percent,
            deviation: self.deviation,
            version: self.version,
            last_updated: self.last_updated,
            tickers_actual: tickers_actual.clone(),
        }
    }
}

#[derive(Debug, PostgresMapper)]
#[pg_mapper(table = "actual_tic")]
pub struct ActualTickerData {
    pub id: i64,
    pub fk_port_g_id: i64,
    pub fk_port_a_id: i64,
    pub fk_tic_id: i64,
    pub actual_shares: f64,
}

impl From<ActualTickerData> for portfolio::TickerActual {
    fn from(item: ActualTickerData) -> Self {
        portfolio::TickerActual::new(
            item.id,
            item.fk_port_g_id,
            item.fk_port_a_id,
            item.fk_tic_id,
            item.actual_shares,
        )
    }
}

#[derive(Debug, FromSql, ToSql)]
#[postgres(name = "dom_port_action")]
pub struct DomainPortAction(String);

impl DomainPortAction {
    pub fn new(s: String) -> Self {
        assert!(
            s == "TICKER" || s == "PERCENT",
            "dom_port_action was not valid: {}",
            s
        );
        DomainPortAction(s)
    }
}

#[derive(Debug, PostgresMapper)]
#[pg_mapper(table = "old_actual_port")]
pub struct OldActualPortData {
    pub id: i64,
    pub fk_port_a_id: i64,
    pub version: i32,
    pub init_port_a_data: serde_json::Value,
    pub new_port_a_data: serde_json::Value,
    pub actions_data: serde_json::Value,
    pub port_action: DomainPortAction,
}
