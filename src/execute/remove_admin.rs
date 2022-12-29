use crate::{
  error::ContractError,
  state::{is_superuser, ADMINS},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn remove_admin(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  admin_address: &Addr,
) -> Result<Response, ContractError> {
  if !is_superuser(deps.storage, &info.sender) {
    return Err(ContractError::NotAuthorized {});
  }

  deps.api.debug(&format!("ACL remove admin {}", admin_address));

  ADMINS.remove(deps.storage, admin_address.clone());

  Ok(Response::new().add_attributes(vec![
    attr("action", "remove_admin"),
    attr("admin_address", admin_address.to_string()),
  ]))
}
