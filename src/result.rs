extern crate csv;
extern crate std;

pub type TTResult<T> = std::result::Result<T, TTError>;

#[derive(Debug)]
pub enum TTError {
    CSVError(csv::Error),
    Uncategorized(String),
}

impl std::fmt::Display for TTError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TTError::CSVError(ref err) => {
                return write!(f, "CSV Error: {}", err);
            },
            TTError::Uncategorized(ref e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for TTError {
    fn description(&self) -> &str {
        match *self {
            TTError::CSVError(_) => "CSVError",
            TTError::Uncategorized(ref str) => str,
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        return None
    }
}

impl From<std::io::Error> for TTError {
    fn from(err: std::io::Error) -> TTError {
        return TTError::Uncategorized(format!("{:?}", err));
    }
}

impl From<csv::Error> for TTError {
    fn from(err: csv::Error) -> TTError {
        return TTError::CSVError(err);
    }
}
