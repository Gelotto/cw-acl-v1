use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub enum Admin {
  Owner(Addr),
  Acl(Addr),
}

#[cw_serde]
pub struct Authorization {
  pub principal: Addr,
  pub actions: Vec<String>,
}
