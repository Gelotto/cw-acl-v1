use std::collections::HashSet;

use crate::client::Acl;
use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use cosmwasm_std::{Addr, Deps, DepsMut, Env, MessageInfo, Storage};
use cw_lib::models::Owner;
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<Owner> = Item::new("owner");
pub const ALLOWED_ACTIONS: Map<(Addr, String), bool> = Map::new("allowed_actions");
pub const PUBLIC_ACTIONS: Map<String, bool> = Map::new("public_actions");
pub const ACTIONS: Map<String, u32> = Map::new("actions");
pub const ROLE_ACTIONS: Map<String, HashSet<String>> = Map::new("role_actions");
pub const ROLES: Map<Addr, HashSet<String>> = Map::new("roles");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  _info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  OWNER.save(deps.storage, &msg.owner)?;

  // perform initial ACL authorizations
  if let Some(authorizations) = msg.authorizations.clone() {
    for auth in authorizations.iter() {
      for action in auth.actions.iter() {
        ALLOWED_ACTIONS.save(deps.storage, (auth.principal.clone(), action.clone()), &true)?;
        increment_action_counter(deps.storage, action)?;
      }
    }
  }

  Ok(())
}

/// Helper function that returns true if given wallet (principal) is authorized
/// by ACL to the given action.
pub fn ensure_sender_is_allowed(
  deps: &Deps,
  principal: &Addr,
  action: &str,
) -> Result<(), ContractError> {
  if !match OWNER.load(deps.storage)? {
    Owner::Address(addr) => *principal == addr,
    Owner::Acl(acl_addr) => {
      let acl = Acl::new(&acl_addr);
      acl.is_allowed(&deps.querier, principal, action)?
    },
  } {
    Err(ContractError::NotAuthorized {})
  } else {
    Ok(())
  }
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
