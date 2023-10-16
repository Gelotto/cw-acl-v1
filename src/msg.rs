use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_lib::models::Owner;

#[cw_serde]
pub struct Authorization {
    pub principal: Principal,
    pub resources: Vec<String>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Owner,
    pub authorizations: Option<Vec<Authorization>>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[cw_serde]
pub enum PrincipalMsg {
    Allow {
        principal: Principal,
        resources: Vec<String>,
    },
    Deny {
        principal: Principal,
        resources: Vec<String>,
        clear: Option<bool>,
    },
    GrantRole {
        principal: Principal,
        roles: Vec<String>,
    },
    RevokeRole {
        principal: Principal,
        roles: Vec<String>,
    },
    Ban {
        principal: Principal,
        reason: Option<String>,
    },
    Unban {
        principal: Principal,
    },
}

#[cw_serde]
pub enum ResourceMsg {
    Open { resources: Vec<String> },
    Close { resources: Vec<String> },
}

#[cw_serde]
pub enum AdminMsg {
    SetOwner(Owner),
}

#[cw_serde]
pub enum ExecuteMsg {
    Resources(ResourceMsg),
    Principals(PrincipalMsg),
    Admin(AdminMsg),
}

#[cw_serde]
pub enum Principal {
    Address(Addr),
    Role(String),
}

impl Principal {
    pub fn as_u8(&self) -> u8 {
        match self {
            Principal::Address(..) => 0,
            Principal::Role(..) => 1,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Principal::Address(addr) => addr.to_string(),
            Principal::Role(role) => role.clone(),
        }
    }
}

#[cw_serde]
pub enum QueryMsg {
    Principal(PrincipalQueryMsg),
    Resources(ResourceQueryMsg),
}

#[cw_serde]
pub enum PrincipalQueryMsg {
    IsAllowed {
        principal: Principal,
        paths: Vec<String>,
    },
    HasRoles {
        principal: Principal,
        roles: Vec<String>,
    },
    Resources {
        principal: Principal,
        cursor: Option<String>,
    },
    Roles {
        principal: Principal,
        cursor: Option<String>,
    },
}

#[cw_serde]
pub enum ResourceQueryMsg {
    Get {
        path: String,
        cursor: Option<String>,
        principal: Option<Principal>,
    },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct BlacklistEntry {
    pub reason: Option<String>,
}

#[cw_serde]
pub struct ResourceNode {
    pub path: String,
    pub children: Vec<ResourceNode>,
    pub is_allowed: Option<bool>,
}

#[cw_serde]
pub struct LsResponse {
    pub resource: ResourceNode,
    pub cursor: Option<String>,
}
