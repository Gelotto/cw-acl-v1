use cosmwasm_std::{Addr, Deps, Order, StdResult};
use cw_lib::{loader::StateLoader, pagination::paginate_map};

use crate::{
  error::ContractError,
  msg::{Account, SelectResponse},
  state::{ACTIONS, ALLOWED_ACTIONS, OWNER, ROLES, ROLE_ACTIONS},
};

pub fn select(
  deps: Deps,
  fields: Option<Vec<String>>,
  wallet: Option<Addr>,
) -> Result<SelectResponse, ContractError> {
  let loader = StateLoader::new(deps.storage, &fields, &wallet);

  Ok(SelectResponse {
    owner: loader.get("admin", &OWNER)?,

    actions: loader.view("actions", |_| {
      Ok(Some(paginate_map(
        &ACTIONS,
        deps.storage,
        None,
        None,
        Order::Ascending,
        100,
        |k, v| -> StdResult<(String, u32)> { Ok((k, v)) },
      )?))
    })?,

    roles: loader.view("roles", |_| {
      Ok(Some(paginate_map(
        &ROLE_ACTIONS,
        deps.storage,
        None,
        None,
        Order::Ascending,
        100,
        |k, v| -> StdResult<(String, Vec<String>)> { Ok((k, v.iter().map(|v| v.clone()).collect())) },
      )?))
    })?,

    account: loader.view("account", |maybe_wallet| {
      Ok(if let Some(wallet) = maybe_wallet {
        Some(Account {
          roles: if let Some(roles) = ROLES.may_load(deps.storage, wallet.clone())? {
            roles.iter().map(|x| -> String { x.clone() }).collect()
          } else {
            vec![]
          },

          actions: ALLOWED_ACTIONS
            .prefix(wallet.clone())
            .range(deps.storage, None, None, Order::Ascending)
            .filter_map(|result| {
              if let Ok((action, is_allowed)) = result {
                if is_allowed {
                  return Some(action);
                }
              }
              None
            })
            .collect(),
        })
      } else {
        None
      })
    })?,
  })
}
