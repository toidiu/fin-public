use std::fmt;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Default)]
pub struct TickerSymbol(pub String);

impl fmt::Debug for TickerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

impl Ticker {
    pub fn is_stock(&self) -> bool {
        self.kind.eq(&InvestmentKind::Stock)
    }
}
