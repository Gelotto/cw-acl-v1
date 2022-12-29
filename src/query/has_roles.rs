use cosmwasm_std::{Addr, Deps, StdResult, Storage};

use crate::{msg::BooleanResponse, state::ROLES};

pub fn has_roles(
  deps: Deps,
  principal: &Addr,
  roles: &Vec<u32>,
) -> StdResult<BooleanResponse> {
  Ok(BooleanResponse {
    value: compute_has_roles(deps.storage, principal, roles)?,
  })
}

fn compute_has_roles(
  storage: &dyn Storage,
  principal: &Addr,
  roles: &Vec<u32>,
) -> StdResult<bool> {
  if let Some(stored_roles) = ROLES.may_load(storage, principal.clone())? {
    for role in roles.iter() {
      if !stored_roles.contains(role) {
        return Ok(false);
      }
    }
    Ok(true)
  } else {
    Ok(false)
  }
}
