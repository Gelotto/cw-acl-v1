use crate::error::ContractError;
use crate::execute::{
    allow, ban, close, deny, grant_roles, open, revoke_roles, set_owner, unban, Context,
};
use crate::msg::{
    AdminMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, PrincipalMsg, PrincipalQueryMsg, QueryMsg,
    ResourceMsg, ResourceQueryMsg,
};
use crate::query::{self, ReadonlyContext};
use crate::state::{self};
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
    let ctx = Context { deps, env, info };
    match msg {
        ExecuteMsg::Admin(msg) => match msg {
            AdminMsg::SetOwner(owner) => set_owner(ctx, owner),
        },
        ExecuteMsg::Resources(msg) => match msg {
            ResourceMsg::Open { resources } => open(ctx, resources),
            ResourceMsg::Close { resources } => close(ctx, resources),
        },
        ExecuteMsg::Principals(msg) => match msg {
            PrincipalMsg::Allow {
                principal,
                resources,
            } => allow(ctx, principal, resources),
            PrincipalMsg::Deny {
                principal,
                resources,
                clear,
            } => deny(ctx, principal, resources, clear),
            PrincipalMsg::GrantRole { principal, roles } => grant_roles(ctx, principal, roles),
            PrincipalMsg::RevokeRole { principal, roles } => revoke_roles(ctx, principal, roles),
            PrincipalMsg::Ban { principal, reason } => ban(ctx, principal, reason),
            PrincipalMsg::Unban { principal } => unban(ctx, principal),
        },
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    let ctx = ReadonlyContext { deps, env };
    let result = match msg {
        QueryMsg::Principal(msg) => match msg {
            PrincipalQueryMsg::IsAllowed { principal, paths } => {
                to_binary(&query::is_allowed(deps, principal, paths)?)
            },
            PrincipalQueryMsg::HasRoles { principal, roles } => {
                to_binary(&query::has_roles(deps, principal, roles)?)
            },
            PrincipalQueryMsg::Resources { principal, cursor } => {
                to_binary(&query::get_principal_resources(ctx, principal, cursor)?)
            },
            PrincipalQueryMsg::Roles { principal, cursor } => {
                to_binary(&query::get_principal_roles(ctx, principal, cursor)?)
            },
        },
        QueryMsg::Resources(msg) => match msg {
            ResourceQueryMsg::Get {
                principal,
                path,
                cursor,
            } => to_binary(&query::get_resource(ctx, path, principal, cursor)?),
        },
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
