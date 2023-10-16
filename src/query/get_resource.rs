use std::marker::PhantomData;

use cosmwasm_std::Order;
use cw_storage_plus::Bound;

use crate::{
    error::ContractError,
    msg::{LsResponse, Principal, ResourceNode},
    state::{is_principal_allowed, IX_TREE},
    util::validate_path_string,
};

use super::ReadonlyContext;

const LIMIT: usize = 100;

pub fn get_resource(
    ctx: ReadonlyContext,
    parent_path: String,
    maybe_principal: Option<Principal>,
    maybe_cursor: Option<String>,
) -> Result<LsResponse, ContractError> {
    validate_path_string(&parent_path)?;

    let ReadonlyContext { deps, .. } = ctx;
    let is_parent_root = parent_path == "/";
    let (principal_type, principal_id) = if let Some(principal) = maybe_principal {
        (principal.as_u8(), principal.to_string())
    } else {
        (0, "".to_owned())
    };

    // Returned resource node
    let mut root = ResourceNode {
        path: parent_path.clone(),
        children: vec![],
        is_allowed: if !principal_id.is_empty() {
            Some(is_principal_allowed(
                deps.storage,
                principal_type,
                &principal_id,
                &parent_path,
            )?)
        } else {
            None
        },
    };

    // Prepare arguments for Map::keys()
    let order = Order::Ascending;
    let min_bound = match &maybe_cursor {
        Some(cursor_str) => Some(Bound::Exclusive((cursor_str, PhantomData))),
        None => None,
    };

    // Fetch child resources and add to root node's children vec
    for maybe_child_resource in IX_TREE
        .prefix(&parent_path)
        .keys(deps.storage, min_bound, None, order)
        .take(LIMIT)
    {
        let child_resource = maybe_child_resource?;
        let child_path = if is_parent_root {
            format!("/{}", child_resource)
        } else {
            format!("{}/{}", parent_path, child_resource)
        };
        root.children.push(ResourceNode {
            path: child_path.clone(),
            children: vec![],
            is_allowed: if !principal_id.is_empty() {
                Some(is_principal_allowed(
                    deps.storage,
                    principal_type,
                    &principal_id,
                    &child_path,
                )?)
            } else {
                None
            },
        });
    }

    Ok(LsResponse {
        cursor: if root.children.len() == LIMIT {
            Some(root.children.last().unwrap().path.clone())
        } else {
            None
        },
        resource: root,
    })
}
