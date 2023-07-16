use crate::{
  error::ContractError,
  state::{ensure_sender_is_allowed, PUBLIC_ACTIONS},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn open(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  action: String,
) -> Result<Response, ContractError> {
  ensure_sender_is_allowed(&deps.as_ref(), &info.sender, "open")?;
  PUBLIC_ACTIONS.save(deps.storage, action.clone(), &true)?;
  Ok(Response::new().add_attributes(vec![attr("action", "revoke"), attr("opened_action", action)]))
}
