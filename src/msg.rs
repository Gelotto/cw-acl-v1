use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::models::Authorization;

#[cw_serde]
pub struct InstantiateMsg {
  pub admins: Option<Vec<Addr>>,
  pub authorizations: Option<Vec<Authorization>>,
}

#[cw_serde]
pub enum ExecuteMsg {
  Allow { principal: Addr, action: String },
  Disallow { principal: Addr, action: String },
  AllowRole { role: u32, action: String },
  DisallowRole { role: u32, action: String },
  SetSuperuser { admin_address: Addr },
  UnsetSuperuser { admin_address: Addr },
  AddAdmin { address: Addr, as_superuser: Option<bool> },
  RemoveAdmin { admin_address: Addr },
  GrantRoles { principal: Addr, roles: Vec<u32> },
  RevokeRoles { principal: Addr, roles: Vec<u32> },
}

#[cw_serde]
pub enum QueryMsg {
  IsAllowed { principal: Addr, action: String },
  IsRoleAllowed { role: u32, action: String },
  IsAdmin { address: Addr },
  IsSuperuser { address: Addr },
  HasRoles { principal: Addr, roles: Vec<u32> },
}

#[cw_serde]
pub struct BooleanResponse {
  pub value: bool,
}
