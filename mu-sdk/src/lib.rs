pub use candid;
pub use ic_cdk;
pub use mu_sdk_macros::public;

pub mod kv;

#[derive(Debug)]
pub enum MuError {
    NotFound,
    InternalError,
}
