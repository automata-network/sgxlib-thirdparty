#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use num_traits::*;
#[cfg(feature = "tstd")]
pub use num_traits_sgx::*;
