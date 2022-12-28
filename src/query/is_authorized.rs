use cosmwasm_std::{Addr, Deps, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::ACL;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IsAuthorizedResponse {
    pub is_authorized: bool,
}

pub fn is_authorized(
    deps: Deps,
    principal: &Addr,
    action: &String,
) -> StdResult<IsAuthorizedResponse> {
    Ok(IsAuthorizedResponse {
        is_authorized: ACL.has(deps.storage, (principal.clone(), action.clone())),
    })
}
