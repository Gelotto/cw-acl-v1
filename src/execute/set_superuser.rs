use crate::{
  error::ContractError,
  models::Admin,
  state::{is_superuser, ADMINS},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn set_superuser(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  admin_address: &Addr,
) -> Result<Response, ContractError> {
  if !is_superuser(deps.storage, &info.sender) {
    return Err(ContractError::NotAuthorized {});
  }

  deps.api.debug(&format!("ACL set {} as superuser", admin_address));

  ADMINS.update(
    deps.storage,
    admin_address.clone(),
    |some_admin| -> Result<Admin, ContractError> {
      if let Some(mut admin) = some_admin {
        admin.is_superuser = true;
        Ok(admin)
      } else {
        Err(ContractError::NotAuthorized {})
      }
    },
  )?;

  Ok(Response::new().add_attributes(vec![
    attr("action", "set_superuser"),
    attr("admin_address", admin_address.to_string()),
  ]))
}
