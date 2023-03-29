use crate::{
  error::ContractError,
  state::{decrement_action_counter, is_allowed, ACL},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn revoke(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  principal: &Addr,
  action: &String,
) -> Result<Response, ContractError> {
  if !is_allowed(&deps.as_ref(), &info.sender, "revoke")? {
    return Err(ContractError::NotAuthorized {});
  }

  deps.api.debug(&format!("ACL revoke {} to {}", action, principal));

  let key = (principal.clone(), action.clone());

  if ACL.has(deps.storage, key.clone()) {
    ACL.remove(deps.storage, key);
    decrement_action_counter(deps.storage, action)?;
  }

  Ok(Response::new().add_attributes(vec![
    attr("action", "revoke"),
    attr("principal", principal.to_string()),
    attr("revoke_action", action),
  ]))
}
