use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type ResultIex<T> = Result<T, IexError>;

#[derive(Copy, Clone, Debug)]
pub enum IexError {
    Wrong,
}

impl Display for IexError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for IexError {
    fn description(&self) -> &str {
        match self {
            IexError::Wrong => "something went wrong",
        }
    }
}
