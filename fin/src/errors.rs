pub type ResultFinErr<T> = Result<T, FinError>;

#[derive(Debug)]
pub enum FinError {
    DatabaseErr(String),
}
