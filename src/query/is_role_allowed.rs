use cosmwasm_std::{Deps, StdResult};

use crate::{msg::BooleanResponse, state::ROLE_ACTIONS};

pub fn is_role_allowed(
  deps: Deps,
  role: u32,
  action: &String,
) -> StdResult<BooleanResponse> {
  Ok(BooleanResponse {
    value: match ROLE_ACTIONS.may_load(deps.storage, role)? {
      Some(actions) => actions.contains(action),
      None => false,
    },
  })
}
