use std::{collections::HashSet, iter::FromIterator};

use crate::{
  error::ContractError,
  state::{ensure_sender_is_allowed, ROLES, ROLE_ACTIONS},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn grant_roles(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  principal: Addr,
  roles: Vec<String>,
) -> Result<Response, ContractError> {
  ensure_sender_is_allowed(&deps.as_ref(), &info.sender, "grant_roles")?;

  deps.api.debug(&format!("ACL grant roles {:?} to {}", roles, principal));

  ROLES.update(
    deps.storage,
    principal.clone(),
    |maybe_roles| -> Result<HashSet<String>, ContractError> {
      if let Some(mut stored_roles) = maybe_roles {
        roles.iter().for_each(|role| {
          stored_roles.insert(role.clone());
        });
        Ok(stored_roles)
      } else {
        Ok(HashSet::from_iter(roles.clone()))
      }
    },
  )?;

  for role in roles.iter() {
    ROLE_ACTIONS.update(
      deps.storage,
      role.clone(),
      |maybe_action_set| -> Result<_, ContractError> { Ok(maybe_action_set.unwrap_or(HashSet::new())) },
    )?;
  }

  Ok(Response::new().add_attributes(vec![
    attr("action", "grant_roles"),
    attr("principal", principal.to_string()),
    attr("roles", format!("{:?}", roles)),
  ]))
}
