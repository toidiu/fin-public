// Helper macros to make life easier.
use log;

macro_rules! lineError(
    ($logger:expr, $msg:expr) => (
        error!($logger, "line: {} - {}", line!(), $msg);
    )
);

macro_rules! lineInfo(
    ($logger:expr, $msg:expr) => (
        info!($logger, "line: {} - {}", line!(), $msg);
    )
);

macro_rules! matches(
    ($e:expr, $p:pat) => (
        match $e {
            $p => true,
            _ => false
        }
    )
);

macro_rules! tic_id(
    ($s:expr) => (
        TickerId::new($s)
    )
);

macro_rules! symbol(
    ($s:expr) => (
        TickerSymbol($s.to_owned())
    )
);

macro_rules! user_id(
    ($s:expr) => (
        server::UserId::new($s)
    )
);

// struct ExtIterator<I: Iterator> {
//     underlying: I,
// }

// impl<I> Iterator for ExtIterator<I>
// where
//     I: Iterator,
// {
//     type Item = I::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.underlying.next()
//     }
// }

pub struct StdExt {}

impl StdExt {
    // todo test!!
    pub fn round_two_digits(val: &mut f32) {
        *val = (*val * 100.00).round() / 100.00;
    }

    pub fn round_two_digits_64(val: &mut f64) {
        *val = (*val * 100.00).round() / 100.00;
    }
}

pub trait ExtIterator: Iterator
where
    Self: Sized,
{
    /// This extends the `is_empty()` semantic to Iterator
    /// trait, which lets us check if the iterator is empty
    /// without actually creating a collection.
    ///
    /// Example:
    ///
    /// ```ignore
    /// extern crate fin;
    /// use fin::std_ext::ExtIterator;
    /// let v = vec![1, 2, 3];
    /// let check = v
    ///     .iter()
    ///     .filter(|x| **x == 1)
    ///     // .collect::<Vec<_>>()  // <-- not needed
    ///     .is_empty();
    /// assert_eq!(check, false);
    /// ```
    fn is_empty(self) -> bool {
        self.peekable().peek().is_none()
    }
}

impl<I: Iterator> ExtIterator for I {}

#[cfg(test)]
mod test {

    use super::ExtIterator;
    use crate::portfolio::*;
    use crate::ticker::*;

    enum TestMacro {
        Foo,
        Bar,
    }

    #[test]
    fn iter_is_not_empty() {
        let v = vec![1, 2, 3];
        let check = v.iter().filter(|x| **x == 1).is_empty();
        assert_eq!(check, false);
    }

    #[test]
    fn iter_is_empty() {
        let v = vec![1, 2, 3];
        let check = v.iter().filter(|x| **x == 0).is_empty();
        assert_eq!(check, true);
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
