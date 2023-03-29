use cosmwasm_std::{Addr, Deps};

use crate::{error::ContractError, msg::BooleanResponse, state::ACL};

pub fn is_denied(
  deps: Deps,
  principal: &Addr,
  action: &String,
) -> Result<BooleanResponse, ContractError> {
  let mut resp: BooleanResponse = BooleanResponse { value: false };

  if let Some(is_allowed) = ACL.may_load(deps.storage, (principal.clone(), action.clone()))? {
    resp.value = !is_allowed;
  }
  Ok(resp)
}
