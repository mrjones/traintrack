extern crate csv;
extern crate hyper;
extern crate protobuf;
extern crate std;

pub type TTResult<T> = std::result::Result<T, TTError>;

#[derive(Debug)]
pub enum TTError {
    CSVError(csv::Error),
    HTTPError(hyper::Error),
    IOError(std::io::Error),
    ProtobufError(protobuf::ProtobufError),
    Uncategorized(String),
}

impl std::fmt::Display for TTError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TTError::CSVError(ref err) => {
                return write!(f, "CSV Error: {}", err);
            },
            TTError::HTTPError(ref err) => {
                return write!(f, "HTTP Error: {}", err);
            },
            TTError::IOError(ref err) => {
                return write!(f, "IO Error: {}", err);
            },
            TTError::ProtobufError(ref err) => {
                return write!(f, "Protobuf Error: {}", err);
            },
            TTError::Uncategorized(ref e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for TTError {
    fn description(&self) -> &str {
        match *self {
            TTError::CSVError(_) => "CSVError",
            TTError::HTTPError(_) => "HTTPError",
            TTError::IOError(_) => "IOError",
            TTError::ProtobufError(_) => "ProtobufError",
            TTError::Uncategorized(ref str) => str,
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        return None
    }
}

impl From<std::io::Error> for TTError {
    fn from(err: std::io::Error) -> TTError {
        return TTError::IOError(err);
    }
}

impl From<csv::Error> for TTError {
    fn from(err: csv::Error) -> TTError {
        return TTError::CSVError(err);
    }
}

impl From<hyper::Error> for TTError {
    fn from(err: hyper::Error) -> TTError {
        return TTError::HTTPError(err);
    }
}

impl From<protobuf::ProtobufError> for TTError {
    fn from(err: protobuf::ProtobufError) -> TTError {
        return TTError::ProtobufError(err);
    }
}
