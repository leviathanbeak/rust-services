use ethabi::{Function, Token, Uint};

use crate::utils::contract_builder::ContractBuilder;

#[derive(Debug, Clone)]
pub struct ExampleContract {
    send_fn: Function,
    request_fn: Function,
}
#[derive(Debug, Clone)]
pub enum ExampleContractError {
    InvalidArgument(String),
    FailedConversion(String),
    UnknownError(String),
}

impl From<ethabi::Error> for ExampleContractError {
    fn from(error: ethabi::Error) -> Self {
        match error {
            ethabi::Error::InvalidData => Self::InvalidArgument("Invalid argument".into()),
            _ => Self::UnknownError("Error not known to the universe".to_owned()),
        }
    }
}

impl From<web3::ethabi::Error> for ExampleContractError {
    fn from(_error: web3::ethabi::Error) -> Self {
        Self::FailedConversion("Failed to convert".to_owned())
    }
}

impl From<hex::FromHexError> for ExampleContractError {
    fn from(_error: hex::FromHexError) -> Self {
        Self::FailedConversion("Failed to convert from Hex".to_owned())
    }
}

impl ExampleContract {
    pub fn new() -> Self {
        let send_fn = ContractBuilder::create_send_fn();
        let request_fn = ContractBuilder::create_request_fn();

        Self {
            send_fn,
            request_fn,
        }
    }

    pub fn encode_send_with_args(
        &self,
        to: &str,
        amount: &str,
    ) -> Result<Vec<u8>, ExampleContractError> {
        let to = &to[2..];

        let bytes = hex::decode(to)?;
        let to = Token::FixedBytes(bytes);

        if let Ok(amount) = Uint::from_dec_str(amount) {
            let amount = Token::Uint(amount);
            let res = self.send_fn.encode_input(&[to, amount])?;
            Ok(res)
        } else {
            Err(ExampleContractError::FailedConversion(
                "Failed to convert from decimal".to_owned(),
            ))
        }
    }

    pub fn encode_request_with_args(
        &self,
        from: &str,
        amount: &str,
    ) -> Result<Vec<u8>, ExampleContractError> {
        let from = &from[2..];
        let bytes = hex::decode(from)?;
        let from = Token::FixedBytes(bytes);

        if let Ok(amount) = Uint::from_dec_str(amount) {
            let amount = Token::Uint(amount);
            let res = self.request_fn.encode_input(&[from, amount])?;
            Ok(res)
        } else {
            Err(ExampleContractError::FailedConversion(
                "Failed to convert from decimal".to_owned(),
            ))
        }
    }
}
