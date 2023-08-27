#![cfg_attr(feature = "tstd", no_std)]
sgxlib::lib!();

#[cfg(feature = "std")]
pub use serde_json::*;
#[cfg(feature = "tstd")]
pub use serde_json_sgx::*;

pub type BoxRawValue = Box<value::RawValue>;
pub type RawValue = value::RawValue;

pub fn from_raw_value<'a, T>(value: &'a BoxRawValue) -> core::result::Result<T, Error>
where
    T: serde::de::Deserialize<'a>,
{
    from_slice(value.get().as_bytes())
}

pub fn to_raw_value<T>(value: &T) -> core::result::Result<BoxRawValue, Error>
where
    T: serde::Serialize,
{
    value::to_raw_value(&value)
}

pub fn raw_from_borrowed(val: &str) -> &RawValue {
    unsafe { std::mem::transmute::<&str, &RawValue>(val) }
}
