use crate::error::ContractError;
use crate::msg::{BlacklistEntry, InstantiateMsg};
use crate::{client::Acl, util::split_path_str};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Storage};
use cw_lib::models::Owner;
use cw_storage_plus::{Item, Map};

pub const MAX_PATH_LEN: usize = 1000;

pub const OWNER: Item<Owner> = Item::new("owner");
pub const CONFIG_NAME: Item<Option<String>> = Item::new("name");
pub const CONFIG_DESCRIPTION: Item<Option<String>> = Item::new("description");
pub const UNRESTRICTED_RESOURCES: Map<&String, bool> = Map::new("unrestricted_resources");
pub const IX_TREE: Map<(&String, &String), bool> = Map::new("ix_tree");
pub const IX_BLACKLIST: Map<(u8, &String), BlacklistEntry> = Map::new("ix_blacklist");
pub const IX_PRINCIPAL_RES: Map<(u8, &String, &String), bool> = Map::new("ix_principal_resource");
pub const IX_RES_PRINCIPAL: Map<(u8, &String, &String), bool> = Map::new("ix_resource_principal");
pub const IX_PRINCIPAL_ROLE: Map<(u8, &String, &String), bool> = Map::new("ix_principal_role");

/// Initialize contract state data.
pub fn initialize(
    deps: DepsMut,
    _env: &Env,
    _info: &MessageInfo,
    msg: &InstantiateMsg,
) -> Result<(), ContractError> {
    OWNER.save(deps.storage, &msg.owner)?;
    CONFIG_NAME.save(deps.storage, &msg.name)?;
    CONFIG_DESCRIPTION.save(deps.storage, &msg.description)?;

    // perform initial authorizations
    if let Some(authorizations) = msg.authorizations.clone() {
        for auth in authorizations.iter() {
            let principal_type = auth.principal.as_u8();
            let principal_id = auth.principal.to_string();
            for res in auth.resources.iter() {
                IX_PRINCIPAL_RES.save(deps.storage, (principal_type, &principal_id, res), &true)?;
            }
        }
    }

    Ok(())
}

pub fn ensure_can_execute(
    deps: &DepsMut,
    principal: &Addr,
    path: &str,
) -> Result<(), ContractError> {
    if !match OWNER.load(deps.storage)? {
        Owner::Address(addr) => *principal == addr,
        Owner::Acl(acl_addr) => {
            let acl = Acl::new(&acl_addr);
            acl.is_allowed(&deps.querier, principal, path)?
        },
    } {
        Err(ContractError::NotAuthorized {})
    } else {
        Ok(())
    }
}

pub fn is_principal_allowed(
    storage: &dyn Storage,
    principal_type_code: u8,
    principal_id: &String,
    cannonical_path: &String,
) -> Result<bool, ContractError> {
    if
    // NOTE: The order of checks matters.
    // Is principal blacklisted?
    IX_BLACKLIST.has(storage, (principal_type_code, principal_id))
    // Is resource specifically denied to principal?
    || !is_principal_allowed_by_ancestors(storage, principal_type_code, principal_id, cannonical_path)?
    // If not specifically denied, is the resource permitted?
    || !UNRESTRICTED_RESOURCES.has(storage, &cannonical_path)
    {
        return Ok(false);
    }
    return Ok(true);
}

fn is_principal_allowed_by_ancestors(
    storage: &dyn Storage,
    principal_type_code: u8,
    principal_id: &String,
    cannonical_path: &String,
) -> Result<bool, ContractError> {
    let mut path: String = cannonical_path.clone();
    loop {
        let is_allowed = IX_PRINCIPAL_RES
            .may_load(
                storage,
                (principal_type_code, principal_id, &cannonical_path),
            )?
            .unwrap_or(true);

        if !is_allowed {
            return Ok(false);
        }
        if path == "/" {
            break;
        } else {
            path = split_path_str(&path).0;
        }
    }
    return Ok(true);
}

pub fn create_resource_if_not_exists(
    storage: &mut dyn Storage,
    child_path_str: &String,
) -> Result<(), ContractError> {
    let (parent_path, maybe_child) = split_path_str(child_path_str);
    if let Some(child) = maybe_child {
        // Save a mapping from parent path to the child resource
        if !IX_TREE.has(storage, (&parent_path, &child)) {
            IX_TREE.save(storage, (&parent_path, &child), &true)?;
        }
    }
    Ok(())
}
