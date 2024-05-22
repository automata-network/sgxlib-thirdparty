#![cfg_attr(feature = "tstd", no_std)]
sgxlib::lib!();

#[cfg(feature = "std")]
pub use libflate::*;
#[cfg(feature = "tstd")]
pub use libflate_sgx::*;