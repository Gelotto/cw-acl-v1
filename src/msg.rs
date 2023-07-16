use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_lib::models::Owner;

use crate::models::Authorization;

#[cw_serde]
pub struct InstantiateMsg {
  pub owner: Owner,
  pub authorizations: Option<Vec<Authorization>>,
}

#[cw_serde]
pub enum ExecuteMsg {
  Allow { principal: Addr, action: String },
  Deny { principal: Addr, action: String },
  Revoke { principal: Addr, action: String },
  GrantRoles { principal: Addr, roles: Vec<String> },
  RevokeRoles { principal: Addr, roles: Vec<String> },
  AllowRole { role: String, action: String },
  RevokeRole { role: String, action: String },
  Open { action: String },
  Restrict { action: String },
}

#[cw_serde]
pub enum QueryMsg {
  Select {
    fields: Option<Vec<String>>,
    wallet: Option<Addr>,
  },
  IsAllowed {
    principal: Addr,
    action: String,
  },
  IsRoleAllowed {
    role: String,
    action: String,
  },
  HasRoles {
    principal: Addr,
    roles: Vec<String>,
  },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct BooleanResponse {
  pub value: bool,
}

#[cw_serde]
pub struct Account {
  pub actions: Vec<String>,
  pub roles: Vec<String>,
}

#[cw_serde]
pub struct SelectResponse {
  pub owner: Option<Owner>,
  pub actions: Option<Vec<(String, u32)>>,
  pub roles: Option<Vec<(String, Vec<String>)>>,
  pub account: Option<Account>,
}
