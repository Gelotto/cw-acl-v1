use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Storage};
use cw_storage_plus::Map;

pub const ADMINS: Map<Addr, bool> = Map::new("admins");
pub const ACL: Map<(Addr, String), bool> = Map::new("acl");

/// Initialize contract state data.
pub fn initialize(
    deps: DepsMut,
    _env: &Env,
    info: &MessageInfo,
    _msg: &InstantiateMsg,
) -> Result<(), ContractError> {
    ADMINS.save(deps.storage, info.sender.clone(), &true)?;
    Ok(())
}

pub fn is_admin(storage: &mut dyn Storage, addr: &Addr) -> bool {
    ADMINS.has(storage, addr.clone())
}
