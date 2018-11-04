use postgres::Error as PostgresError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type ResultFin<T> = Result<T, FinError>;

#[derive(Debug)]
pub enum FinError {
    DatabaseErr(String),
}

impl Display for FinError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl From<PostgresError> for FinError {
    fn from(err: PostgresError) -> Self {
        FinError::DatabaseErr(err.description().to_string())
    }
}

impl StdError for FinError {
    fn description(&self) -> &str {
        match *self {
            FinError::DatabaseErr(ref inner) => inner,
        }
    }
}
