use crate::{error::ContractError, msg::Principal, state::IX_PRINCIPAL_ROLE};

use std::marker::PhantomData;

use cosmwasm_std::Order;
use cw_storage_plus::Bound;

use super::ReadonlyContext;

pub fn get_principal_roles(
    ctx: ReadonlyContext,
    principal: Principal,
    maybe_cursor: Option<String>,
) -> Result<Vec<String>, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let principal_id = &principal.to_string();
    let prefix = (principal.as_u8(), principal_id);
    let cursor = maybe_cursor.unwrap_or_default();
    let mut resources: Vec<String> = Vec::with_capacity(8);

    for maybe_resource in IX_PRINCIPAL_ROLE.prefix(prefix).keys(
        deps.storage,
        if cursor.is_empty() {
            None
        } else {
            Some(Bound::Exclusive((&cursor, PhantomData)))
        },
        None,
        Order::Ascending,
    ) {
        resources.push(maybe_resource?);
    }

    Ok(resources)
}
