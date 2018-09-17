/*
 * Helper macros to make life easier.
 */

macro_rules! matches(
    ($e:expr, $p:pat) => (
        match $e {
            $p => true,
            _ => false
        }
    )
);

macro_rules! symbol(
    ($s:expr) => (
        TickerSymbol($s.to_owned())
    )
);

#[cfg(test)]
mod test {

    use crate::ticker::*;

    enum TestMacro {
        Foo,
        Bar,
    }

    #[test]
    fn symbol_should_equal() {
        assert_eq!(TickerSymbol("bla".to_owned()), symbol!("bla"));
    }

    #[test]
    fn should_match() {
        let foo = TestMacro::Foo;
        assert!(matches!(foo, TestMacro::Foo));
        let bar = TestMacro::Bar;
        assert!(matches!(bar, TestMacro::Bar));
    }

    #[test]
    fn should_not_match() {
        let foo = TestMacro::Foo;
        assert_eq!(false, matches!(foo, TestMacro::Bar));
    }
}
