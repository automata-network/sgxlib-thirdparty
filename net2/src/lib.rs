#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use net2::*;
#[cfg(feature = "tstd")]
pub use net2_sgx::*;
