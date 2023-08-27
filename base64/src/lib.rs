#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use base64::*;
#[cfg(feature = "tstd")]
pub use base64_sgx::*;