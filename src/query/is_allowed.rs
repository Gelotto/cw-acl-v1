use cosmwasm_std::Deps;

use crate::{
    error::ContractError, msg::Principal, state::is_principal_allowed, util::validate_path_string,
};

pub fn is_allowed(
    deps: Deps,
    principal: Principal,
    paths: Vec<String>,
) -> Result<bool, ContractError> {
    let principal_id = principal.to_string();
    for path in paths.iter() {
        validate_path_string(path)?;
        if !is_principal_allowed(deps.storage, principal.as_u8(), &principal_id, &path)? {
            return Ok(false);
        }
    }
    Ok(true)
}
