use crate::{
    error::ContractError,
    msg::{BlacklistEntry, Principal},
    state::{ensure_can_execute, IX_BLACKLIST},
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn ban(
    ctx: Context,
    principal: Principal,
    maybe_reason: Option<String>,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;

    ensure_can_execute(&deps, &info.sender, "/acl/resources/ban")?;

    IX_BLACKLIST.save(
        deps.storage,
        (principal.as_u8(), &principal.to_string()),
        &BlacklistEntry {
            reason: maybe_reason,
        },
    )?;

    Ok(Response::new().add_attributes(vec![attr("action", "ban")]))
}

pub fn unban(
    ctx: Context,
    principal: Principal,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;

    ensure_can_execute(&deps, &info.sender, "/acl/admin/unban")?;

    IX_BLACKLIST.remove(deps.storage, (principal.as_u8(), &principal.to_string()));

    Ok(Response::new().add_attributes(vec![attr("action", "unban")]))
}
