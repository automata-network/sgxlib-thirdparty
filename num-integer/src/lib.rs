#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use num_integer::*;
#[cfg(feature = "tstd")]
pub use num_integer_sgx::*;
