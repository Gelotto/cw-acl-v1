use cosmwasm_std::{Addr, Deps, StdResult};

use crate::{msg::BooleanResponse, state::ADMINS};

pub fn is_admin(
  deps: Deps,
  address: &Addr,
) -> StdResult<BooleanResponse> {
  Ok(BooleanResponse {
    value: ADMINS.has(deps.storage, address.clone()),
  })
}
