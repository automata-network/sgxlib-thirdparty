#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use libc::*;
#[cfg(feature = "tstd")]
pub use sgxlib::sgx_libc::*;
