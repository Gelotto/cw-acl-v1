mod allow;
mod ban;
mod deny;
mod grant_roles;
mod open;
mod restrict;
mod revoke_roles;
mod set_owner;

pub use allow::allow;
pub use ban::{ban, unban};
use cosmwasm_std::{DepsMut, Env, MessageInfo};
pub use deny::deny;
pub use grant_roles::grant_roles;
pub use open::open;
pub use restrict::close;
pub use revoke_roles::revoke_roles;
pub use set_owner::set_owner;

pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}
