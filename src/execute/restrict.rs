use crate::{
  error::ContractError,
  state::{ensure_sender_is_allowed, PUBLIC_ACTIONS},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn restrict(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  action: String,
) -> Result<Response, ContractError> {
  ensure_sender_is_allowed(&deps.as_ref(), &info.sender, "restrict")?;
  PUBLIC_ACTIONS.remove(deps.storage, action.clone());
  Ok(Response::new().add_attributes(vec![attr("action", "restrict"), attr("closed_action", action)]))
}
