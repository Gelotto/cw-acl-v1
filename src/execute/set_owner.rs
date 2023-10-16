use crate::{
    error::ContractError,
    state::{ensure_can_execute, OWNER},
};
use cosmwasm_std::{attr, Response};
use cw_lib::models::Owner;

use super::Context;

pub fn set_owner(
    ctx: Context,
    new_owner: Owner,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    ensure_can_execute(&deps, &info.sender, "/acl/admin/set-owner")?;

    OWNER.save(deps.storage, &new_owner)?;

    Ok(Response::new().add_attributes(vec![attr("action", "set_owner")]))
}
