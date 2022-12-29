use crate::{
  error::ContractError,
  state::{is_admin, ACL},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn allow(
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
    .debug(&format!("ACL allow address {} to {}", principal, action));

  ACL.save(deps.storage, (principal.clone(), action.clone()), &true)?;

  Ok(Response::new().add_attributes(vec![
    attr("action", "allow"),
    attr("principal", principal.to_string()),
    attr("allow_action", action),
  ]))
}
