use crate::{
    error::ContractError,
    state::{ensure_can_execute, UNRESTRICTED_RESOURCES},
    util::validate_path_string,
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn close(
    ctx: Context,
    paths: Vec<String>,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;

    ensure_can_execute(&deps, &info.sender, "/acl/resources/close")?;

    for path in paths.iter() {
        validate_path_string(path)?;
        UNRESTRICTED_RESOURCES.remove(deps.storage, path);
    }

    Ok(Response::new().add_attributes(vec![attr("action", "close")]))
}
