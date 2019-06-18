use std::fmt;

#[derive(
    Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Default,
)]
pub struct TickerSymbol(pub String);

#[derive(
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Clone,
    Default,
    Debug,
)]
pub struct TickerId(i64);

impl TickerId {
    pub fn new(v: i64) -> Self {
        TickerId(v)
    }

    pub fn get_ticker_id(&self) -> &i64 {
        &self.0
    }
}

impl fmt::Display for TickerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for TickerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use the `Display` fmt which only selects the String
        write!(f, "{}", self)
    }
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Debug,
)]
pub enum InvestmentKind {
    #[serde(rename = "STOCK")]
    Stock,
    #[serde(rename = "BOND")]
    Bond,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ticker {
    pub id: i64,
    pub symbol: TickerSymbol,
    pub exchange: i32,
    pub fee: f32,
    pub price: f64,
    kind: InvestmentKind,
}

impl Ticker {
    pub fn new(
        id: i64,
        symbol: TickerSymbol,
        exchange: i32,
        fee: f32,
        price: f64,
        kind: InvestmentKind,
    ) -> Self {
        Ticker {
            id: id,
            symbol: symbol,
            exchange: exchange,
            fee: fee,
            price: price,
            kind: kind,
        }
    }

    pub fn get_kind(&self) -> &InvestmentKind {
        &self.kind
    }

    pub fn is_stock(&self) -> bool {
        self.kind.eq(&InvestmentKind::Stock)
    }

    pub fn is_bond(&self) -> bool {
        self.kind.eq(&InvestmentKind::Bond)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    struct Helper {}
    impl Helper {
        fn get_stock() -> Ticker {
            Ticker {
                id: 1,
                symbol: symbol!("vwo"),
                exchange: 1,
                fee: 0.14,
                price: 43.0,
                kind: InvestmentKind::Stock,
            }
        }

        fn get_bond() -> Ticker {
            Ticker {
                id: 1,
                symbol: symbol!("vtip"),
                exchange: 1,
                fee: 0.06,
                price: 49.0,
                kind: InvestmentKind::Bond,
            }
        }
    }

    #[test]
    fn stock_should_be_stock() {
        let stock = Helper::get_stock();
        assert!(stock.is_stock());
        assert_eq!(false, stock.is_bond());
    }

    #[test]
    fn bond_should_be_bond() {
        let bond = Helper::get_bond();
        assert!(bond.is_bond());
        assert_eq!(false, bond.is_stock());
    }

    #[test]
    fn symbol_should_have_clean_display_fmt() {
        let symbol = symbol!("abc");
        assert_eq!("abc", format!("{}", symbol));
    }

    #[test]
    fn symbol_should_have_clean_debug_fmt() {
        let symbol = symbol!("abc");
        assert_eq!("abc", format!("{:?}", symbol));
    }
}
