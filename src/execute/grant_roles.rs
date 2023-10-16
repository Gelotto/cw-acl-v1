use crate::{
    error::ContractError,
    msg::Principal,
    state::{ensure_can_execute, IX_PRINCIPAL_ROLE},
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn grant_roles(
    ctx: Context,
    principal: Principal,
    roles: Vec<String>,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    let principal_id = &principal.to_string();

    ensure_can_execute(&deps, &info.sender, "/acl/roles/grant")?;

    for role in roles.iter() {
        // Update lookup table for testing if a principal has a role
        IX_PRINCIPAL_ROLE.save(
            deps.storage,
            (principal.as_u8(), principal_id, &role),
            &true,
        )?;
    }

    Ok(Response::new().add_attributes(vec![
        attr("action", "grant_roles"),
        attr("principal", principal.to_string()),
    ]))
}
