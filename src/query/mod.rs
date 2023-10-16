mod get_principal_resources;
mod get_principal_roles;
mod get_resource;
mod has_roles;
mod is_allowed;

use cosmwasm_std::{Deps, Env};
pub use get_principal_resources::get_principal_resources;
pub use get_principal_roles::get_principal_roles;
pub use get_resource::get_resource;
pub use has_roles::has_roles;
pub use is_allowed::is_allowed;

pub struct ReadonlyContext<'a> {
    pub deps: Deps<'a>,
    pub env: Env,
}
