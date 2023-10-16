use cosmwasm_std::Deps;

use crate::{error::ContractError, msg::Principal, state::IX_PRINCIPAL_ROLE};

pub fn has_roles(
    deps: Deps,
    principal: Principal,
    roles: Vec<String>,
) -> Result<bool, ContractError> {
    for role in roles.iter() {
        if !IX_PRINCIPAL_ROLE.has(
            deps.storage,
            (principal.as_u8(), &principal.to_string(), role),
        ) {
            return Ok(false);
        }
    }
    Ok(true)
}
