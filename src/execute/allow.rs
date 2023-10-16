use crate::{
    error::ContractError,
    msg::Principal,
    state::{
        create_resource_if_not_exists, ensure_can_execute, IX_PRINCIPAL_RES, IX_RES_PRINCIPAL,
    },
    util::validate_path_string,
};
use cosmwasm_std::{attr, Response, Storage};

use super::Context;

pub fn allow(
    ctx: Context,
    principal: Principal,
    paths: Vec<String>,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;

    ensure_can_execute(&deps, &info.sender, "/acl/resources/allow")?;

    let principal_type = principal.as_u8();
    let principal_id = principal.to_string();

    for path in paths.iter() {
        validate_path_string(path)?;
        create_resource_if_not_exists(deps.storage, path)?;
        allow_resource_to_principal(deps.storage, path, principal_type, &principal_id)?;
    }

    Ok(Response::new().add_attributes(vec![
        attr("action", "allow"),
        attr("principal", principal.to_string()),
    ]))
}

fn allow_resource_to_principal(
    storage: &mut dyn Storage,
    path: &String,
    principal_type: u8,
    principal_id: &String,
) -> Result<(), ContractError> {
    IX_RES_PRINCIPAL.save(storage, (principal_type, &path, principal_id), &true)?;
    IX_PRINCIPAL_RES.save(storage, (principal_type, principal_id, &path), &true)?;
    Ok(())
}
