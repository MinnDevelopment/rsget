use hls_m3u8::Error as HlsError;
use reqwest::Error as ReqwestError;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};
use tokio::io::Error as TokioIoError;
use url::ParseError;

#[derive(Debug)]
pub enum Error {
    /// M3U8 error
    Hls(HlsError),
    /// Http error.
    Reqwest(ReqwestError),
    /// Url error.
    Url(ParseError),
    /// Tokio IO error
    TIO(TokioIoError),
}

impl From<HlsError> for Error {
    fn from(err: HlsError) -> Self {
        Error::Hls(err)
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Error::Reqwest(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::Url(err)
    }
}

impl From<TokioIoError> for Error {
    fn from(err: TokioIoError) -> Self {
        Error::TIO(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", &self)
    }
}

impl StdError for Error {}
