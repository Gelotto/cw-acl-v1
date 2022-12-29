use std::collections::HashSet;

use crate::msg::InstantiateMsg;
use crate::{error::ContractError, models::Admin};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Storage};
use cw_storage_plus::Map;

pub const ADMINS: Map<Addr, Admin> = Map::new("admins");
pub const ACL: Map<(Addr, String), bool> = Map::new("acl");
pub const ROLE_ACTIONS: Map<u32, HashSet<String>> = Map::new("role_actions");
pub const ROLES: Map<Addr, HashSet<u32>> = Map::new("roles");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  // allways save tx sender as initial superuser admin
  ADMINS.save(deps.storage, info.sender.clone(), &Admin { is_superuser: true })?;

  // initialize admin authorities
  if let Some(admins) = msg.admins.clone() {
    for addr in admins.iter() {
      if *addr != info.sender {
        ADMINS.save(deps.storage, addr.clone(), &Admin { is_superuser: false })?;
      }
    }
  }

  // perform initial ACL authorizations
  if let Some(authorizations) = msg.authorizations.clone() {
    for auth in authorizations.iter() {
      for action in auth.actions.iter() {
        ACL.save(deps.storage, (auth.principal.clone(), action.clone()), &true)?;
      }
    }
  }

  Ok(())
}

pub fn is_admin(
  storage: &mut dyn Storage,
  addr: &Addr,
) -> bool {
  ADMINS.has(storage, addr.clone())
}

pub fn is_superuser(
  storage: &mut dyn Storage,
  addr: &Addr,
) -> bool {
  if let Some(some_admin) = ADMINS.may_load(storage, addr.clone()).ok() {
    if let Some(admin) = some_admin {
      if admin.is_superuser {
        return true;
      }
    }
  }
  false
}
