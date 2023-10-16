use crate::{
    error::ContractError,
    state::{ensure_can_execute, UNRESTRICTED_RESOURCES},
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn open(
    ctx: Context,
    paths: Vec<String>,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;

    ensure_can_execute(&deps, &info.sender, "/acl/resources/open")?;

    for path in paths.iter() {
        // Indicate that the given path is open to the public
        UNRESTRICTED_RESOURCES.save(deps.storage, path, &true)?;
    }

    Ok(Response::new().add_attributes(vec![attr("action", "open")]))
}
