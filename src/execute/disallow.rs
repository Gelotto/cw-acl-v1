use crate::{
  error::ContractError,
  state::{is_admin, ACL},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn disallow(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  principal: &Addr,
  action: &String,
) -> Result<Response, ContractError> {
  if !is_admin(deps.storage, &info.sender) {
    return Err(ContractError::NotAuthorized {});
  }

  deps
    .api
    .debug(&format!("ACL disallow address {} to {}", principal, action));

  ACL.remove(deps.storage, (principal.clone(), action.clone()));

  Ok(Response::new().add_attributes(vec![
    attr("action", "disallow"),
    attr("principal", principal.to_string()),
    attr("disallow_action", action),
  ]))
}
