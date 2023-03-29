use cosmwasm_std::{Addr, Deps};

use crate::{
  error::ContractError,
  msg::BooleanResponse,
  state::{ACL, ROLES, ROLE_ACTIONS},
};

pub fn is_allowed(
  deps: Deps,
  principal: &Addr,
  action: &String,
) -> Result<BooleanResponse, ContractError> {
  // first check if the action has been allowed to prinipal directly
  let mut resp: BooleanResponse = BooleanResponse { value: false };

  if let Some(is_allowed) = ACL.may_load(deps.storage, (principal.clone(), action.clone()))? {
    resp.value = is_allowed;
  } else {
    // if not, check if the action is authorized via one of principal's roles.
    if let Some(roles) = ROLES.may_load(deps.storage, principal.clone())? {
      for role in roles.iter() {
        if let Some(actions) = ROLE_ACTIONS.may_load(deps.storage, role.clone())? {
          if actions.contains(action) {
            resp.value = true;
            break;
          }
        }
      }
    }
  }
  Ok(resp)
}
