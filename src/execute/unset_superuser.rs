use crate::{
  error::ContractError,
  state::{is_superuser, ADMINS},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn unset_superuser(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  admin_address: &Addr,
) -> Result<Response, ContractError> {
  if !is_superuser(deps.storage, &info.sender) {
    return Err(ContractError::NotAuthorized {});
  }

  deps
    .api
    .debug(&format!("ACL unset admin {} as superuser", admin_address));

  ADMINS.remove(deps.storage, admin_address.clone());

  Ok(Response::new().add_attributes(vec![
    attr("action", "unset_superuser"),
    attr("admin_address", admin_address.to_string()),
  ]))
}
