#![cfg_attr(feature = "tstd", no_std)]
sgxlib::lib!();

#[macro_use]
extern crate alloc;

// This `extern crate` invocation tells `rustc` that we actually need the symbols from `blst`.
// Without it, the compiler won't link to `blst` when compiling this crate.
// See: https://kornel.ski/rust-sys-crate#linking
extern crate blst;

mod bindings;

use core::mem::MaybeUninit;

use bindings::NUM_G2_POINTS;
// Expose relevant types with idiomatic names.
pub use bindings::{
    KZGCommitment as KzgCommitment, KZGProof as KzgProof, KZGSettings as KzgSettings,
    C_KZG_RET as CkzgError,
};
// Expose the constants.
pub use bindings::{
    BYTES_PER_BLOB, BYTES_PER_COMMITMENT, BYTES_PER_FIELD_ELEMENT, BYTES_PER_G1_POINT,
    BYTES_PER_G2_POINT, BYTES_PER_PROOF, FIELD_ELEMENTS_PER_BLOB,
};
// Expose the remaining relevant types.
pub use bindings::{Blob, Bytes32, Bytes48, Error};
use serde::Deserialize;

pub const BUILDIN_TRUSTED_SETUP: &[u8] = include_bytes!("trusted_setup.json");

#[derive(Deserialize)]
struct TrustedSetup {
    g1_lagrange: Vec<String>,
    g2_monomial: Vec<String>,
}

pub fn build_setting(json_bytes: &[u8]) -> Result<KzgSettings, String> {
    let setup: TrustedSetup =
        serde_json::from_slice(json_bytes).map_err(|err| format!("{:?}", err))?;

    if setup.g1_lagrange.len() != FIELD_ELEMENTS_PER_BLOB {
        return Err(format!(
            "Invalid number of g1 points in trusted setup. Expected {} got {}",
            FIELD_ELEMENTS_PER_BLOB,
            setup.g1_lagrange.len(),
        ));
    }
    if setup.g2_monomial.len() != NUM_G2_POINTS {
        return Err(format!(
            "Invalid number of g2 points in trusted setup. Expected {} got {}",
            NUM_G2_POINTS,
            setup.g2_monomial.len(),
        ));
    }

    let mut g1 = vec![0_u8; setup.g1_lagrange.len() * BYTES_PER_G1_POINT];
    let mut g2 = vec![0_u8; setup.g2_monomial.len() * BYTES_PER_G2_POINT];
    for (idx, item) in setup.g1_lagrange.iter().enumerate() {
        let mut item = item.trim_start_matches("0x");
        let g1_bytes = hex::decode(item).unwrap();
        if g1_bytes.len() != BYTES_PER_G1_POINT {
            return Err(format!(
                "invalid g1 bytes={}, expected {}",
                item, BYTES_PER_G1_POINT
            ));
        }
        g1[idx * BYTES_PER_G1_POINT..(idx + 1) * BYTES_PER_G1_POINT].copy_from_slice(&g1_bytes);
    }
    for (idx, item) in setup.g2_monomial.iter().enumerate() {
        let mut item = item.trim_start_matches("0x");
        let g2_bytes = hex::decode(item).unwrap();
        if g2_bytes.len() != BYTES_PER_G2_POINT {
            return Err(format!(
                "invalid g2 bytes={}, expected {}",
                item, BYTES_PER_G2_POINT
            ));
        }
        g2[idx * BYTES_PER_G2_POINT..(idx + 1) * BYTES_PER_G2_POINT].copy_from_slice(&g2_bytes);
    }

    let mut kzg_settings = MaybeUninit::<KzgSettings>::uninit();
    unsafe {
        let res = crate::bindings::load_trusted_setup(
            kzg_settings.as_mut_ptr(),
            g1.as_ptr(),
            setup.g1_lagrange.len(),
            g2.as_ptr(),
            setup.g2_monomial.len(),
        );
        if let crate::bindings::C_KZG_RET::C_KZG_OK = res {
            Ok(kzg_settings.assume_init())
        } else {
            Err(format!("Invalid trusted setup: {:?}", res))
        }
    }
}

lazy_static::lazy_static! {
    pub static ref BUILDIN_TRUSTED_SETTING: KzgSettings = build_setting(&BUILDIN_TRUSTED_SETUP).unwrap();
}
