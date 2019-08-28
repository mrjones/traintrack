// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate csv;
extern crate frank_jwt;
extern crate prost;
extern crate reqwest;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate std;

pub type TTResult<T> = std::result::Result<T, TTError>;

#[derive(Debug)]
pub enum TTError {
    CSVError(csv::Error),
    IOError(std::io::Error),
    JWTError(frank_jwt::Error),
    ProstDecodeError(prost::DecodeError),
    ProstEncodeError(prost::EncodeError),
    SerializationError(serde_json::Error),
    ParseIntError(std::num::ParseIntError),
    HttpClientError(reqwest::Error),
    Utf8Error(std::str::Utf8Error),
    XmlError(serde_xml_rs::Error),
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
            TTError::IOError(ref err) => {
                return write!(f, "IO Error: {}", err);
            },
            TTError::JWTError(ref err) => {
                return write!(f, "JWT Error: {:?}", err);
            },
            TTError::ProstDecodeError(ref err) => {
                return write!(f, "ProstDecode Error: {}", err);
            },
            TTError::ProstEncodeError(ref err) => {
                return write!(f, "ProstEncode Error: {}", err);
            },
            TTError::SerializationError(ref err) => {
                return write!(f, "Serialization Error: {}", err);
            },
            TTError::ParseIntError(ref err) => {
                return write!(f, "ParseInt Error: {}", err);
            },
            TTError::HttpClientError(ref err) => {
                return write!(f, "Http Client Error: {}", err);
            },
            TTError::Utf8Error(ref err) => {
                return write!(f, "UTF8 Error: {}", err);
            },
            TTError::XmlError(ref err) => {
                return write!(f, "XML Error: {}", err);
            },
            TTError::Uncategorized(ref e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for TTError {
    fn description(&self) -> &str {
        match *self {
            TTError::CSVError(_) => "CSVError",
            TTError::IOError(_) => "IOError",
            TTError::JWTError(_) => "JWTError",
            TTError::ProstDecodeError(_) => "ProstDecodeError",
            TTError::ProstEncodeError(_) => "ProstEncodeError",
            TTError::SerializationError(_) => "SerializationError",
            TTError::ParseIntError(_) => "ParseIntError",
            TTError::HttpClientError(_) => "HttpClientError",
            TTError::Utf8Error(_) => "Utf8Errorr",
            TTError::XmlError(_) => "XmlErrorr",
            TTError::Uncategorized(ref str) => str,
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
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

impl From<prost::DecodeError> for TTError {
    fn from(err: prost::DecodeError) -> TTError {
        return TTError::ProstDecodeError(err);
    }
}

impl From<prost::EncodeError> for TTError {
    fn from(err: prost::EncodeError) -> TTError {
        return TTError::ProstEncodeError(err);
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

impl From<reqwest::Error> for TTError {
    fn from(err: reqwest::Error) -> TTError {
        return TTError::HttpClientError(err);
    }
}


impl From<frank_jwt::Error> for TTError {
    fn from(err: frank_jwt::Error) -> TTError {
        return TTError::JWTError(err);
    }
}

impl From<std::str::Utf8Error> for TTError {
    fn from(err: std::str::Utf8Error) -> TTError {
        return TTError::Utf8Error(err);
    }
}

impl From<serde_xml_rs::Error> for TTError {
    fn from(err: serde_xml_rs::Error) -> TTError {
        return TTError::XmlError(err);
    }
}
