use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("The reply ID is unrecognized")]
    UnrecognizedReply {},

    #[error("Unauthorized")]
    Unauthorized {},
}