use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Admin {
  pub is_superuser: bool,
}

#[cw_serde]
pub struct Authorization {
  pub principal: Addr,
  pub actions: Vec<String>,
}
