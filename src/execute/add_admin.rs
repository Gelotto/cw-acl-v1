use crate::{
  error::ContractError,
  models::Admin,
  state::{is_superuser, ADMINS},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn add_admin(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  admin_address: &Addr,
  as_superuser: Option<bool>,
) -> Result<Response, ContractError> {
  if !is_superuser(deps.storage, &info.sender) {
    return Err(ContractError::NotAuthorized {});
  }

  deps.api.debug(&format!("ACL Adding admin {}", admin_address));

  ADMINS.save(
    deps.storage,
    admin_address.clone(),
    &Admin {
      is_superuser: as_superuser.unwrap_or(false),
    },
  )?;

  Ok(Response::new().add_attributes(vec![
    attr("action", "add_admin"),
    attr("admin_address", admin_address.to_string()),
  ]))
}
