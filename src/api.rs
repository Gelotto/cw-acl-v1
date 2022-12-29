use crate::msg::{BooleanResponse, QueryMsg};
use cosmwasm_std::{Addr, Empty, QuerierWrapper, StdResult};

pub fn is_allowed(
  acl_contract_addr: &Addr,
  querier: &QuerierWrapper<Empty>,
  principal: &Addr,
  action: &str,
) -> StdResult<bool> {
  let resp: BooleanResponse = querier.query_wasm_smart(
    acl_contract_addr,
    &QueryMsg::IsAllowed {
      principal: principal.clone(),
      action: action.to_owned(),
    },
  )?;
  Ok(resp.value)
}
