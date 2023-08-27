#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use env_logger::*;
#[cfg(feature = "tstd")]
pub use env_logger_sgx::*;
