#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use http_req::*;
#[cfg(feature = "tstd")]
pub use http_req_sgx::*;
