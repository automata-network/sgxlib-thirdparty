#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use log::*;
#[cfg(feature = "tstd")]
pub use log_sgx::*;
