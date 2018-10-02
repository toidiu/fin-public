use crate::ticker::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioState {
    pub tickers: Vec<TickerState>,
    pub goal_stock_percent: f32,
    pub actual_stock_percent: f32,
    pub total_value: f32,
    pub deviation_percent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerState {
    pub symbol: TickerSymbol,
    pub kind: InvestmentKind,
    pub fee: f32,
    pub goal_percent: f32,
    pub actual_percent: f32,
    pub actual_value: f32,
    pub price: f32,
    pub order: u32,
}
