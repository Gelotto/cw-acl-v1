use cosmwasm_std::{Addr, Deps, StdResult};

use crate::{msg::BooleanResponse, state::ADMINS};

pub fn is_superuser(
  deps: Deps,
  admin_address: &Addr,
) -> StdResult<BooleanResponse> {
  Ok(BooleanResponse {
    value: if let Some(admin) = ADMINS.may_load(deps.storage, admin_address.clone())? {
      admin.is_superuser
    } else {
      false
    },
  })
}
