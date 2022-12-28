use crate::{
    error::ContractError,
    state::{is_admin, ACL},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn authorize(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    principal: &Addr,
    action: &String,
) -> Result<Response, ContractError> {
    if !is_admin(deps.storage, &info.sender) {
        return Err(ContractError::NotAuthorized {});
    }

    ACL.save(deps.storage, (info.sender.clone(), action.clone()), &true)?;

    Ok(Response::new().add_attributes(vec![
        attr("principal", principal.to_string()),
        attr("action", action),
    ]))
}
