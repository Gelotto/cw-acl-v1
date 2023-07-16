use crate::{
  error::ContractError,
  state::{ensure_sender_is_allowed, ROLE_ACTIONS},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn deny_role(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  role: String,
  action: String,
) -> Result<Response, ContractError> {
  ensure_sender_is_allowed(&deps.as_ref(), &info.sender, "deny_role")?;

  deps.api.debug(&format!("ACL disallow role {} to {}", role, action));

  if let Some(mut actions) = ROLE_ACTIONS.may_load(deps.storage, role.clone())? {
    let was_removed = actions.remove(&action);
    if was_removed {
      ROLE_ACTIONS.save(deps.storage, role.clone(), &actions)?;
    }
  }

  Ok(Response::new().add_attributes(vec![
    attr("action", "deny_role"),
    attr("role", role.to_string()),
    attr("deny_action", action),
  ]))
}
