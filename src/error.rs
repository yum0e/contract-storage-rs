use ethers::providers::ProviderError;
use rustc_hex::FromHexError;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    ProviderError,
    ParseHexError,
}

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::ProviderError => write!(f, "Provider Error"),
            CustomError::ParseHexError => write!(f, "Error while parsing hex number"),
        }
    }
}

impl From<ProviderError> for CustomError {
    fn from(_: ProviderError) -> Self {
        CustomError::ProviderError
    }
}

impl From<FromHexError> for CustomError {
    fn from(_: FromHexError) -> Self {
        CustomError::ParseHexError
    }
}
