#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use serde::*;
#[cfg(feature = "tstd")]
pub use serde_sgx::*;