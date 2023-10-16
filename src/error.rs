use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("NotAuthorized")]
  NotAuthorized {},

  #[error("UnexpectedError: {reason:?}")]
  UnexpectedError { reason: Option<String> },

  #[error("ValidationError: {reason:?}")]
  ValidationError { reason: Option<String> },
}
