#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use rustls::*;
#[cfg(feature = "tstd")]
pub use rustls_sgx::*;
