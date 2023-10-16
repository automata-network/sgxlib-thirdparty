#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use byteorder::*;
#[cfg(feature = "tstd")]
pub use byteorder_sgx::*;
