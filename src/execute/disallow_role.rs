use crate::{
  error::ContractError,
  state::{is_admin, ROLES},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn disallow_role(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  role: u32,
  action: &String,
) -> Result<Response, ContractError> {
  if !is_admin(deps.storage, &info.sender) {
    return Err(ContractError::NotAuthorized {});
  }

  deps.api.debug(&format!("ACL disallow role {} to {}", role, action));

  if let Some(mut actions) = ROLES.may_load(deps.storage, role)? {
    actions.remove(action);
    ROLES.save(deps.storage, role, &actions)?;
  }

  Ok(Response::new().add_attributes(vec![
    attr("action", "disallow_role"),
    attr("role", role.to_string()),
    attr("disallow_action", action),
  ]))
}
