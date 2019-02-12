use http::StatusCode;
use postgres::Error as PostgresError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type ResultFin<T> = Result<T, FinError>;

#[derive(Copy, Clone, Debug)]
pub enum FinError {
    NotLoggedIn,   // user is not logged in
    ServerErr,     // internal server error
    BadRequestErr, // a request is malformed (form has bad data)
    NotFoundErr,   // resource not found
    DatabaseErr,   // any database related error
}

impl Display for FinError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl From<PostgresError> for FinError {
    fn from(err: PostgresError) -> Self {
        FinError::DatabaseErr
    }
}

impl StdError for FinError {
    fn description(&self) -> &str {
        match self {
            FinError::NotLoggedIn => "user log-in required",
            FinError::BadRequestErr => "bad request",
            FinError::NotFoundErr => "not found",
            FinError::DatabaseErr | FinError::ServerErr => {
                "an error occured with the service"
            }
        }
    }
}

impl FinError {
    pub fn to_msg(&self) -> ErrMessage {
        ErrMessage {
            code: self.value(),
            message: self.to_string(),
        }
    }

    /// useful for user debugging
    fn value(&self) -> u16 {
        match self {
            FinError::NotLoggedIn => 1,
            FinError::ServerErr => 20,
            FinError::BadRequestErr => 21,
            FinError::NotFoundErr => 22,
            FinError::DatabaseErr => 25,
        }
    }
}

/// Return type to user
#[derive(Serialize)]
pub struct ErrMessage {
    code: u16,
    message: String,
}

impl ErrMessage {
    pub fn new(code: StatusCode, msg: String) -> ErrMessage {
        ErrMessage {
            code: code.as_u16(),
            message: msg,
        }
    }
}
