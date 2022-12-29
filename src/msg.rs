use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::Authorization;

/// Initial contract state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
  pub admins: Option<Vec<Addr>>,
  pub authorizations: Option<Vec<Authorization>>,
}

/// Executable contract endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

/// Custom contract query endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  IsAllowed { principal: Addr, action: String },
  IsRoleAllowed { role: u32, action: String },
  IsAdmin { address: Addr },
  IsSuperuser { address: Addr },
  HasRoles { principal: Addr, roles: Vec<u32> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BooleanResponse {
  pub value: bool,
}
