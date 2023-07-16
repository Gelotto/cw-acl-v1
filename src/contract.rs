use crate::error::ContractError;
use crate::execute::{allow, allow_role, deny, deny_role, grant_roles, open, restrict, revoke, revoke_roles};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{has_roles, is_allowed, is_role_allowed, select};
use crate::state;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-acl";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  state::initialize(deps, &env, &info, &msg)?;
  Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::Allow { principal, action } => allow(deps, env, info, principal, action),
    ExecuteMsg::Deny { principal, action } => deny(deps, env, info, principal, action),
    ExecuteMsg::Revoke { principal, action } => revoke(deps, env, info, principal, action),
    ExecuteMsg::RevokeRole { role, action } => deny_role(deps, env, info, role, action),
    ExecuteMsg::AllowRole { role, action } => allow_role(deps, env, info, role, action),
    ExecuteMsg::GrantRoles { principal, roles } => grant_roles(deps, env, info, principal, roles),
    ExecuteMsg::RevokeRoles { principal, roles } => revoke_roles(deps, env, info, principal, roles),
    ExecuteMsg::Open { action } => open(deps, env, info, action),
    ExecuteMsg::Restrict { action } => restrict(deps, env, info, action),
  }
}

#[entry_point]
pub fn query(
  deps: Deps,
  _env: Env,
  msg: QueryMsg,
) -> Result<Binary, ContractError> {
  let result = match msg {
    QueryMsg::IsAllowed { principal, action } => to_binary(&is_allowed(deps, &principal, &action)?),
    QueryMsg::IsRoleAllowed { role, action } => to_binary(&is_role_allowed(deps, &role, &action)?),
    QueryMsg::HasRoles { principal, roles } => to_binary(&has_roles(deps, &principal, &roles)?),
    QueryMsg::Select { fields, wallet } => to_binary(&select(deps, fields, wallet)?),
  }?;
  Ok(result)
}

#[entry_point]
pub fn migrate(
  _deps: DepsMut,
  _env: Env,
  _msg: MigrateMsg,
) -> Result<Response, ContractError> {
  Ok(Response::default())
}
