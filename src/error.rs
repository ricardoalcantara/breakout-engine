pub type BreakoutResult<T> = Result<T, BreakoutError>;

#[derive(Debug)]
pub enum BreakoutError {
    GenericError(String),
}
