use cosmwasm_std::{Addr, Deps, StdResult};

use crate::{
  msg::BooleanResponse,
  state::{ACL, ROLES, ROLE_ACTIONS},
};

pub fn is_allowed(
  deps: Deps,
  principal: &Addr,
  action: &String,
) -> StdResult<BooleanResponse> {
  // first check if the action has been allowed to prinipal directly
  let mut is_allowed = ACL.has(deps.storage, (principal.clone(), action.clone()));

  // if not, check if the action is authorized via one of principal's roles.
  if !is_allowed {
    if let Some(roles) = ROLES.may_load(deps.storage, principal.clone())? {
      for role in roles.iter() {
        if let Some(actions) = ROLE_ACTIONS.may_load(deps.storage, role.clone())? {
          if actions.contains(action) {
            is_allowed = true;
            break;
          }
        }
      }
    }
  }
  Ok(BooleanResponse { value: is_allowed })
}
