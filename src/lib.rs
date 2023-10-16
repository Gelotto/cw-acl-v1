#[cfg(not(feature = "library"))]
pub mod contract;

mod error;

pub mod client;
pub mod execute;
pub mod msg;
pub mod query;
pub mod state;

mod util;
