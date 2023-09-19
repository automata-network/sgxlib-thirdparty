#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use thiserror::*;
#[cfg(feature = "tstd")]
pub use thiserror_sgx::*;
