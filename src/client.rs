use cosmwasm_std::{Addr, Empty, QuerierWrapper, StdResult};

use crate::msg::{Principal, PrincipalQueryMsg, QueryMsg};

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
        addresss: &Addr,
        path: &str,
    ) -> StdResult<bool> {
        Ok(querier.query_wasm_smart::<bool>(
            self.acl_contract_addr.clone(),
            &QueryMsg::Principal(PrincipalQueryMsg::IsAllowed {
                principal: Principal::Address(addresss.clone()),
                paths: vec![path.to_string()],
            }),
        )?)
    }
}
