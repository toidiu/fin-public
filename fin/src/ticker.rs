#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TickerSymbol(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub enum InvestmentKind {
    #[serde(rename = "stock")]
    Stock,
    #[serde(rename = "bond")]
    Bond,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    pub symbol: TickerSymbol,
    pub fee: f32,
    pub price: f32,
    pub kind: InvestmentKind,
}
