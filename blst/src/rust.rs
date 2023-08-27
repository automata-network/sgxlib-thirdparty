use super::bindings::*;

use core::ptr::null_mut;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct PublicKey(blst_p1_affine);

#[derive(Default, Clone, PartialEq, Eq)]
pub struct SecretKey(blst_scalar);

impl core::fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{}...{}", hex::encode(&self.to_bytes()[..2]), hex::encode(&self.to_bytes()[30..]))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Signature(blst_p2_affine);

const PUBLIC_KEY_LENGTH: usize = 48;
const SECRET_KEY_LENGTH: usize = 32;
// const SIGNATURE_LENGTH: usize = 96;
const BLST_FP_BYTES: usize = 384 / 8;
const BLST_P1_COMPRESS_BYTES: usize = BLST_FP_BYTES;
const BLST_P2_COMPRESS_BYTES: usize = BLST_FP_BYTES * 2;
const BLS_SIG: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_POP_";

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    DeserializeSecretKey,
    InvalidPubkey,
    InvalidPubkeyLength,
    InvalidSecretKeyLength,
    InvalidSignature,
    InvalidSignatureLength,
    UncompressPubkey,
    UncompressSignature,
}

pub fn sign(sk: &SecretKey, msg: &[u8]) -> Signature {
    let mut sig = Signature::default();
    sig.sign(sk, msg, BLS_SIG);
    sig
}

pub fn keygen_v3(ikm: [u8; 32]) -> SecretKey {
    let mut seckey = SecretKey::default();
    let info = &[];
    unsafe {
        blst_keygen_v3(
            &mut seckey.0 as *mut _,
            ikm.as_ptr(),
            ikm.len(),
            info.as_ptr(),
            info.len(),
        )
    }
    seckey
}

impl Signature {
    fn raw(&self) -> *const blst_p2_affine {
        return &self.0 as *const blst_p2_affine;
    }

    fn raw_mut(&mut self) -> *mut blst_p2_affine {
        &mut self.0 as *mut blst_p2_affine
    }

    fn hash_to_g2(msg: &[u8], dst: &[u8]) -> blst_p2 {
        let mut q = blst_p2::default();
        let aug = [0_u8; 0];
        unsafe {
            blst_hash_to_g2(
                &mut q as *mut blst_p2,
                msg.as_ptr(),
                msg.len(),
                dst.as_ptr(),
                dst.len(),
                aug.as_ptr(),
                aug.len(),
            );
        }
        q
    }

    pub fn sign(&mut self, sk: &SecretKey, msg: &[u8], dst: &[u8]) {
        unsafe {
            let hash = Self::hash_to_g2(msg, dst);
            blst_sign_pk2_in_g1(
                null_mut(),
                self.raw_mut(),
                &hash as *const blst_p2,
                sk.raw(),
            );
        }
    }

    pub fn compress(&self) -> [u8; BLST_P2_COMPRESS_BYTES] {
        let mut out = [0_u8; BLST_P2_COMPRESS_BYTES];
        unsafe { blst_p2_affine_compress(out.as_mut_ptr(), self.raw()) }
        out
    }
}

impl PublicKey {
    fn raw(&self) -> *const blst_p1_affine {
        return &self.0 as *const blst_p1_affine;
    }

    fn raw_mut(&mut self) -> *mut blst_p1_affine {
        &mut self.0 as *mut blst_p1_affine
    }

    pub fn validate(&self) -> bool {
        unsafe { !blst_p1_affine_is_inf(self.raw()) && blst_p1_affine_in_g1(self.raw()) }
    }

    pub fn from_hex(hex_str: &[u8]) -> Result<Self, Error> {
        match hex::decode(hex_str) {
            Ok(data) => Self::from_bytes(&data),
            Err(_) => Err(Error::InvalidPubkey),
        }
    }

    pub fn compress(&self) -> [u8; BLST_P1_COMPRESS_BYTES] {
        let mut out = [0_u8; BLST_P1_COMPRESS_BYTES];
        unsafe { blst_p1_affine_compress(out.as_mut_ptr(), self.raw()) };
        out
    }

    pub fn from_bytes(pk_bytes: &[u8]) -> Result<Self, Error> {
        if pk_bytes.len() != PUBLIC_KEY_LENGTH {
            return Err(Error::InvalidPubkeyLength);
        }
        let mut pk = Self::default();
        unsafe {
            let err = blst_p1_uncompress(&mut pk.0 as *mut blst_p1_affine, pk_bytes.as_ptr());
            if err != BLST_ERROR::BLST_SUCCESS {
                return Err(Error::UncompressPubkey);
            }
        }
        if !pk.validate() {
            return Err(Error::InvalidPubkey);
        }
        Ok(pk)
    }
}

impl SecretKey {
    fn raw(&self) -> *const blst_scalar {
        &self.0 as *const blst_scalar
    }

    pub fn raw_mut(&mut self) -> *mut blst_scalar {
        &mut self.0 as *mut blst_scalar
    }

    pub fn public(&self) -> PublicKey {
        let mut pk = PublicKey::default();
        unsafe { blst_sk_to_pk2_in_g1(null_mut(), pk.raw_mut(), self.raw()) };
        pk
    }

    pub fn from_bytes(sk_bytes: &[u8]) -> Result<Self, Error> {
        if sk_bytes.len() != SECRET_KEY_LENGTH {
            return Err(Error::InvalidSecretKeyLength);
        }
        let mut sk = Self::default();
        unsafe {
            blst_scalar_from_bendian(&mut sk.0 as *mut blst_scalar, sk_bytes.as_ptr());
        }
        if sk.is_empty() {
            return Err(Error::InvalidSecretKeyLength);
        }
        Ok(sk)
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    pub fn to_bytes(&self) -> [u8; SECRET_KEY_LENGTH] {
        let mut bytes = [0_u8; SECRET_KEY_LENGTH];
        unsafe {
            blst_bendian_from_scalar(bytes.as_mut_ptr(), self.raw());
        }
        bytes
    }
}

pub fn sha256_sum(data: &[u8]) -> [u8; 32] {
    let mut out = [0_u8; 32];
    unsafe { blst_sha256(out.as_mut_ptr(), data.as_ptr(), data.len()) }
    out
}
