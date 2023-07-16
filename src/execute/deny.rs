use crate::{
  error::ContractError,
  state::{ensure_sender_is_allowed, increment_action_counter, ALLOWED_ACTIONS},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn deny(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  principal: Addr,
  action: String,
) -> Result<Response, ContractError> {
  ensure_sender_is_allowed(&deps.as_ref(), &info.sender, "deny")?;

  deps.api.debug(&format!("ACL deny address {} to {}", principal, action));

  let mut do_increment = false;

  ALLOWED_ACTIONS.update(
    deps.storage,
    (principal.clone(), action.clone()),
    |maybe_value| -> Result<_, ContractError> {
      if maybe_value.is_none() {
        do_increment = true;
      }
      Ok(false)
    },
  )?;

  if do_increment {
    increment_action_counter(deps.storage, &action)?;
  }

  Ok(Response::new().add_attributes(vec![
    attr("action", "deny"),
    attr("principal", principal.to_string()),
    attr("deny_action", action),
  ]))
}
