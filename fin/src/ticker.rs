#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Debug, Default)]
pub struct TickerSymbol(pub String);

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Debug)]
pub enum InvestmentKind {
    #[serde(rename = "stock")]
    Stock,
    #[serde(rename = "bond")]
    Bond,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ticker {
    pub symbol: TickerSymbol,
    pub fee: f32,
    pub price: f32,
    pub kind: InvestmentKind,
}
