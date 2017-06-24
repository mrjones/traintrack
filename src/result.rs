extern crate csv;
extern crate liquid;
extern crate protobuf;
extern crate serde_json;
extern crate std;

pub type TTResult<T> = std::result::Result<T, TTError>;

#[derive(Debug)]
pub enum TTError {
    CSVError(csv::Error),
//    HTTPError(hyper::Error),
    IOError(std::io::Error),
    ProtobufError(protobuf::ProtobufError),
    RenderError(liquid::Error),
    SerializationError(serde_json::Error),
    ParseIntError(std::num::ParseIntError),
    Uncategorized(String),
}

pub fn quick_err(desc: &str) -> TTError {
    return TTError::Uncategorized(desc.to_string());
}

impl std::fmt::Display for TTError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TTError::CSVError(ref err) => {
                return write!(f, "CSV Error: {}", err);
            },
//            TTError::HTTPError(ref err) => {
//                return write!(f, "HTTP Error: {}", err);
//            },
            TTError::IOError(ref err) => {
                return write!(f, "IO Error: {}", err);
            },
            TTError::ProtobufError(ref err) => {
                return write!(f, "Protobuf Error: {}", err);
            },
            TTError::RenderError(ref err) => {
                return write!(f, "Render Error: {}", err);
            },
            TTError::SerializationError(ref err) => {
                return write!(f, "Serialization Error: {}", err);
            },
            TTError::ParseIntError(ref err) => {
                return write!(f, "ParseInt Error: {}", err);
            },
            TTError::Uncategorized(ref e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for TTError {
    fn description(&self) -> &str {
        match *self {
            TTError::CSVError(_) => "CSVError",
//            TTError::HTTPError(_) => "HTTPError",
            TTError::IOError(_) => "IOError",
            TTError::ProtobufError(_) => "ProtobufError",
            TTError::RenderError(_) => "RenderError",
            TTError::SerializationError(_) => "SerializationError",
            TTError::ParseIntError(_) => "ParseIntError",
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

//impl From<hyper::Error> for TTError {
//    fn from(err: hyper::Error) -> TTError {
//        return TTError::HTTPError(err);
//    }
//}

impl From<protobuf::ProtobufError> for TTError {
    fn from(err: protobuf::ProtobufError) -> TTError {
        return TTError::ProtobufError(err);
    }
}

impl From<liquid::Error> for TTError {
    fn from(err: liquid::Error) -> TTError {
        return TTError::RenderError(err);
    }
}

impl From<serde_json::Error> for TTError {
    fn from(err: serde_json::Error) -> TTError {
        return TTError::SerializationError(err);
    }
}

impl From<std::num::ParseIntError> for TTError {
    fn from(err: std::num::ParseIntError) -> TTError {
        return TTError::ParseIntError(err);
    }
}
