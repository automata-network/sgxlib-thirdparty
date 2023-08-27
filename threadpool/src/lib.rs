#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use threadpool::*;
#[cfg(feature = "tstd")]
pub use threadpool_sgx::*;
