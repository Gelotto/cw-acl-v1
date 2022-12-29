use std::{collections::HashSet, iter::FromIterator};

use crate::{
  error::ContractError,
  state::{is_admin, ROLES},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn grant_roles(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  principal: &Addr,
  roles: &Vec<u32>,
) -> Result<Response, ContractError> {
  if !is_admin(deps.storage, &info.sender) {
    return Err(ContractError::NotAuthorized {});
  }

  deps
    .api
    .debug(&format!("ACL grant address {} roles {:?}", principal, roles));

  ROLES.update(
    deps.storage,
    principal.clone(),
    |some_roles| -> Result<HashSet<u32>, ContractError> {
      if let Some(mut stored_roles) = some_roles {
        roles.iter().for_each(|role| {
          stored_roles.insert(*role);
        });
        Ok(stored_roles)
      } else {
        Ok(HashSet::from_iter(roles.clone()))
      }
    },
  )?;

  Ok(Response::new().add_attributes(vec![
    attr("action", "grant_roles"),
    attr("principal", principal.to_string()),
    attr("roles", format!("{:?}", roles)),
  ]))
}
