#[cfg(not(feature = "library"))]
use crate::error::ContractError;
use crate::execute::{
  add_admin, allow, allow_role, disallow, disallow_role, grant_roles, remove_admin, revoke_roles, set_superuser,
  unset_superuser,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{has_roles, is_admin, is_allowed, is_role_allowed, is_superuser};
use crate::state;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-acl";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::Allow { principal, action } => allow(deps, env, info, &principal, &action),
    ExecuteMsg::Disallow { principal, action } => disallow(deps, env, info, &principal, &action),
    ExecuteMsg::DisallowRole { role, action } => disallow_role(deps, env, info, role, &action),
    ExecuteMsg::AllowRole { role, action } => allow_role(deps, env, info, role, &action),
    ExecuteMsg::RemoveAdmin { admin_address } => remove_admin(deps, env, info, &admin_address),
    ExecuteMsg::AddAdmin { address, as_superuser } => add_admin(deps, env, info, &address, as_superuser),
    ExecuteMsg::SetSuperuser { admin_address } => set_superuser(deps, env, info, &admin_address),
    ExecuteMsg::UnsetSuperuser { admin_address } => unset_superuser(deps, env, info, &admin_address),
    ExecuteMsg::GrantRoles { principal, roles } => grant_roles(deps, env, info, &principal, &roles),
    ExecuteMsg::RevokeRoles { principal, roles } => revoke_roles(deps, env, info, &principal, &roles),
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
  deps: Deps,
  _env: Env,
  msg: QueryMsg,
) -> StdResult<Binary> {
  let result = match msg {
    QueryMsg::IsAllowed { principal, action } => to_binary(&is_allowed(deps, &principal, &action)?),
    QueryMsg::IsRoleAllowed { role, action } => to_binary(&is_role_allowed(deps, role, &action)?),
    QueryMsg::IsAdmin { address } => to_binary(&is_admin(deps, &address)?),
    QueryMsg::IsSuperuser { address } => to_binary(&is_superuser(deps, &address)?),
    QueryMsg::HasRoles { principal, roles } => to_binary(&has_roles(deps, &principal, &roles)?),
  }?;
  Ok(result)
}
