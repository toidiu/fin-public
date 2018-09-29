use std::fmt;

#[derive(
    Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Default,
)]
pub struct TickerSymbol(pub String);

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

    pub fn is_bond(&self) -> bool {
        self.kind.eq(&InvestmentKind::Bond)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    struct Helper {}
    impl Helper {
        pub fn get_stock() -> Ticker {
            Ticker {
                symbol: symbol!("vwo"),
                fee: 0.14,
                price: 43.0,
                kind: InvestmentKind::Stock,
            }
        }

        pub fn get_bond() -> Ticker {
            Ticker {
                symbol: symbol!("vtip"),
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
