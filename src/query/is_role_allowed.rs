use cosmwasm_std::{Deps, StdResult};

use crate::{msg::BooleanResponse, state::ROLES};

pub fn is_role_allowed(
  deps: Deps,
  role: u32,
  action: &String,
) -> StdResult<BooleanResponse> {
  Ok(BooleanResponse {
    value: match ROLES.may_load(deps.storage, role)? {
      Some(actions) => actions.contains(action),
      None => false,
    },
  })
}
