#![cfg_attr(feature = "tstd", no_std)]
sgxlib::lib!();

#[cfg(feature = "std")]
pub use serde_big_array::*;
#[cfg(feature = "tstd")]
pub use serde_big_array_sgx::*;
