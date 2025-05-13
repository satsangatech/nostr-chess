#[derive(Debug)]
pub enum ChessError {
    NotFound(&'static str),
    InvalidPgn(std::io::Error),
}
impl std::error::Error for ChessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidPgn(err) => Some(err),
            Self::NotFound(_) => None,
        }
    }
    fn description(&self) -> &str {
        match self {
            Self::NotFound(msg) => msg,
            Self::InvalidPgn(e) => Box::leak(format!("Invalid PGN: {e}").into_boxed_str()),
        }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::InvalidPgn(ref err) => Some(err),
            Self::NotFound(_) => None,
        }
    }
}
impl From<std::io::Error> for ChessError {
    fn from(err: std::io::Error) -> Self {
        Self::InvalidPgn(err)
    }
}
impl std::fmt::Display for ChessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
            Self::InvalidPgn(err) => write!(f, "Invalid PGN: {err}"),
        }
    }
}
