#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use num_bigint::*;
#[cfg(feature = "tstd")]
pub use num_bigint_sgx::*;
