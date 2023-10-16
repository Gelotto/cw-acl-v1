use crate::{
    error::ContractError,
    msg::Principal,
    state::{ensure_can_execute, IX_PRINCIPAL_ROLE},
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn revoke_roles(
    ctx: Context,
    principal: Principal,
    roles: Vec<String>,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    let principal_id = &principal.to_string();

    ensure_can_execute(&deps, &info.sender, "/acl/roles/revoke")?;

    for role in roles.iter() {
        // Update lookup table for testing if a principal has a role
        IX_PRINCIPAL_ROLE.remove(deps.storage, (principal.as_u8(), principal_id, role));
    }

    Ok(Response::new().add_attributes(vec![
        attr("action", "revoke_roles"),
        attr("principal", principal.to_string()),
    ]))
}
