use cosmwasm_std::{Addr, Deps, StdResult};

use crate::{msg::BooleanResponse, state::ACL};

pub fn is_allowed(
  deps: Deps,
  principal: &Addr,
  action: &String,
) -> StdResult<BooleanResponse> {
  Ok(BooleanResponse {
    value: ACL.has(deps.storage, (principal.clone(), action.clone())),
  })
}
