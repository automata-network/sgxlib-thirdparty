#![cfg_attr(feature = "tstd", no_std)]

#[cfg(feature = "std")]
pub use webpki::*;
#[cfg(feature = "tstd")]
pub use webpki_sgx::*;

pub mod roots {
    #[cfg(feature = "std")]
    pub use webpki_roots::*;
    #[cfg(feature = "tstd")]
    pub use webpki_roots_sgx::*;
}
