use std::collections::HashSet;

use crate::client::Acl;
use crate::msg::InstantiateMsg;
use crate::{error::ContractError, models::Admin};
use cosmwasm_std::{Addr, Deps, DepsMut, Env, MessageInfo, Storage};
use cw_storage_plus::{Item, Map};

pub const ADMIN: Item<Admin> = Item::new("admin");
pub const ACL: Map<(Addr, String), bool> = Map::new("acl");
pub const ROLE_ACTIONS: Map<String, HashSet<String>> = Map::new("role_actions");
pub const ROLES: Map<Addr, HashSet<String>> = Map::new("roles");
pub const ACTIONS: Map<String, u32> = Map::new("actions");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  _info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  ADMIN.save(deps.storage, &msg.admin)?;

  // perform initial ACL authorizations
  if let Some(authorizations) = msg.authorizations.clone() {
    for auth in authorizations.iter() {
      for action in auth.actions.iter() {
        ACL.save(deps.storage, (auth.principal.clone(), action.clone()), &true)?;
        increment_action_counter(deps.storage, action)?;
      }
    }
  }

  Ok(())
}

pub fn is_allowed(
  deps: &Deps,
  principal: &Addr,
  action: &str,
) -> Result<bool, ContractError> {
  Ok(match ADMIN.load(deps.storage)? {
    Admin::Owner(owner_addr) => *principal == owner_addr,
    Admin::Acl(acl_addr) => {
      let acl = Acl::new(&acl_addr);
      acl.is_allowed(&deps.querier, principal, action)?
    },
  })
}

pub fn increment_action_counter(
  storage: &mut dyn Storage,
  action: &String,
) -> Result<u32, ContractError> {
  ACTIONS.update(storage, action.clone(), |maybe_counter| -> Result<_, ContractError> {
    Ok(maybe_counter.unwrap_or_default() + 1)
  })
}

pub fn decrement_action_counter(
  storage: &mut dyn Storage,
  action: &String,
) -> Result<u32, ContractError> {
  if let Some(n) = ACTIONS.may_load(storage, action.clone())? {
    if n > 1 {
      ACTIONS.save(storage, action.clone(), &(n - 1))?;
      Ok(n - 1)
    } else {
      ACTIONS.remove(storage, action.clone());
      Ok(0)
    }
  } else {
    Ok(0)
  }
}
