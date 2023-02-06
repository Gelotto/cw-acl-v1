use cosmwasm_std::{Addr, Empty, QuerierWrapper, StdResult};

use crate::msg::{BooleanResponse, QueryMsg};

pub struct Acl {
  acl_contract_addr: Addr,
}

impl Acl {
  pub fn new(acl_contract_addr: &Addr) -> Self {
    Self {
      acl_contract_addr: acl_contract_addr.clone(),
    }
  }

  pub fn is_allowed(
    &self,
    querier: &QuerierWrapper<Empty>,
    principal: &Addr,
    action: &str,
  ) -> StdResult<bool> {
    Ok(
      querier
        .query_wasm_smart::<BooleanResponse>(
          self.acl_contract_addr.clone(),
          &QueryMsg::IsAllowed {
            principal: principal.clone(),
            action: action.to_owned(),
          },
        )?
        .value,
    )
  }
}
