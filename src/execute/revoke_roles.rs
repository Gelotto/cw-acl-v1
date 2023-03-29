use crate::{
  error::ContractError,
  state::{is_allowed, ROLES},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn revoke_roles(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  principal: &Addr,
  roles: &Vec<String>,
) -> Result<Response, ContractError> {
  if !is_allowed(&deps.as_ref(), &info.sender, "revoke_roles")? {
    return Err(ContractError::NotAuthorized {});
  }

  deps
    .api
    .debug(&format!("ACL revoke {:?} roles from {}", roles, principal));

  if let Some(mut stored_roles) = ROLES.may_load(deps.storage, principal.clone())? {
    roles.iter().for_each(|role| {
      stored_roles.remove(role);
    });
    ROLES.save(deps.storage, principal.clone(), &stored_roles)?;
  }

  Ok(Response::new().add_attributes(vec![
    attr("action", "revoke_roles"),
    attr("principal", principal.to_string()),
    attr("roles", format!("{:?}", roles)),
  ]))
}
