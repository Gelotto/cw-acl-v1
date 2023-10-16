use crate::{
    error::ContractError,
    msg::Principal,
    state::{
        create_resource_if_not_exists, ensure_can_execute, IX_PRINCIPAL_RES, IX_RES_PRINCIPAL,
    },
    util::validate_path_string,
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn deny(
    ctx: Context,
    principal: Principal,
    paths: Vec<String>,
    clear: Option<bool>,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;

    ensure_can_execute(&deps, &info.sender, "/acl/resources/deny")?;

    let principal_type = principal.as_u8();
    let principal_id = principal.to_string();

    for path in paths.iter() {
        validate_path_string(path)?;
        create_resource_if_not_exists(deps.storage, path)?;
        if clear.unwrap_or(false) {
            IX_RES_PRINCIPAL.remove(deps.storage, (principal_type, &path, &principal_id));
            IX_PRINCIPAL_RES.remove(deps.storage, (principal_type, &principal_id, &path));
        } else {
            IX_RES_PRINCIPAL.save(deps.storage, (principal_type, &path, &principal_id), &false)?;
            IX_PRINCIPAL_RES.save(deps.storage, (principal_type, &principal_id, &path), &false)?;
        }
    }

    Ok(Response::new().add_attributes(vec![
        attr("action", "deny"),
        attr("principal", principal.to_string()),
    ]))
}
