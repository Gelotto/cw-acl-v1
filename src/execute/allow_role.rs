use std::collections::HashSet;

use crate::{
  error::ContractError,
  state::{ensure_sender_is_allowed, ROLE_ACTIONS},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn allow_role(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  role: String,
  action: String,
) -> Result<Response, ContractError> {
  ensure_sender_is_allowed(&deps.as_ref(), &info.sender, "allow_role")?;

  deps.api.debug(&format!("ACL allow role {} to {}", role, action));

  ROLE_ACTIONS.update(
    deps.storage,
    role.clone(),
    |some_actions| -> Result<HashSet<String>, ContractError> {
      if let Some(mut actions) = some_actions {
        if !actions.contains(&action) {
          actions.insert(action.clone());
        }
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
