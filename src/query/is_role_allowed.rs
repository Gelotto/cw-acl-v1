use cosmwasm_std::Deps;

use crate::{error::ContractError, msg::BooleanResponse, state::ROLE_ACTIONS};

pub fn is_role_allowed(
  deps: Deps,
  role: &String,
  action: &String,
) -> Result<BooleanResponse, ContractError> {
  Ok(BooleanResponse {
    value: match ROLE_ACTIONS.may_load(deps.storage, role.clone())? {
      Some(actions) => actions.contains(action),
      None => false,
    },
  })
}
