#[cfg(feature = "library")]
pub mod client;

#[cfg(not(feature = "library"))]
pub mod contract;

mod error;

pub mod execute;
pub mod models;
pub mod msg;
pub mod query;
pub mod state;
