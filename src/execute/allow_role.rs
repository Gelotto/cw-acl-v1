use std::collections::HashSet;

use crate::{
  error::ContractError,
  state::{is_admin, ROLES},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn allow_role(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  role: u32,
  action: &String,
) -> Result<Response, ContractError> {
  if !is_admin(deps.storage, &info.sender) {
    return Err(ContractError::NotAuthorized {});
  }

  deps.api.debug(&format!("ACL allow role {} to {}", role, action));

  ROLES.update(
    deps.storage,
    role,
    |some_actions| -> Result<HashSet<String>, ContractError> {
      if let Some(mut actions) = some_actions {
        actions.insert(action.clone());
        Ok(actions)
      } else {
        let mut actions = HashSet::with_capacity(1);
        actions.insert(action.clone());
        Ok(actions)
      }
    },
  )?;

  Ok(Response::new().add_attributes(vec![
    attr("action", "allow_role"),
    attr("role", role.to_string()),
    attr("allow_action", action),
  ]))
}
