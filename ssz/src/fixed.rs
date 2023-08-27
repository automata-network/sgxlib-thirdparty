use std::prelude::v1::*;

use super::{Encode, Error, Decode, Compact, CompactRef, Add, Mul, Div, Codec};
use super::utils::{encode_list, decode_list};
use generic_array::{GenericArray, ArrayLength};
use vecarray::VecArray;
use primitive_types::H256;
use typenum::Unsigned;
use core::convert::TryFrom;
use std::vec::Vec;

macro_rules! impl_builtin_fixed_uint_vector {
	( $( $t:ty ),* ) => { $(
		impl<L: ArrayLength<$t> + Unsigned> Codec for Compact<GenericArray<$t, L>>
		{
			type Size = Mul<<$t as Codec>::Size, L>;
		}

		impl<'a, L: ArrayLength<$t> + Unsigned> Codec for CompactRef<'a, GenericArray<$t, L>> where
			Compact<GenericArray<$t, L>>: Codec,
		{
			type Size = <Compact<GenericArray<$t, L>> as Codec>::Size;
		}

		impl<'a, L: ArrayLength<$t>> Encode for CompactRef<'a, GenericArray<$t, L>> where
			CompactRef<'a, GenericArray<$t, L>>: Codec
		{
			fn encode(&self) -> Vec<u8> {
				encode_list(self.0)
			}
		}

		impl<L: ArrayLength<$t>> Encode for Compact<GenericArray<$t, L>> where
			Compact<GenericArray<$t, L>>: Codec,
			for<'a> CompactRef<'a, GenericArray<$t, L>>: Encode
		{
			fn encode(&self) -> Vec<u8> {
				CompactRef(&self.0).encode()
			}
		}

		impl<L: ArrayLength<$t>> Decode for Compact<GenericArray<$t, L>> where
			Compact<GenericArray<$t, L>>: Codec,
		{
			fn decode(value: &[u8]) -> Result<Self, Error> {
				let decoded = decode_list::<$t>(value)?;
				if decoded.len() != L::to_usize() {
					return Err(Error::InvalidLength)
				}
				let mut ret = GenericArray::default();
				for i in 0..decoded.len() {
					ret[i] = decoded[i];
				}
				Ok(Compact(ret))
			}
		}

		impl<L: Unsigned> Codec for Compact<VecArray<$t, L>>
		{
			type Size = Mul<<$t as Codec>::Size, L>;
		}

		impl<'a, L: Unsigned> Codec for CompactRef<'a, VecArray<$t, L>> where
			Compact<VecArray<$t, L>>: Codec,
		{
			type Size = <Compact<VecArray<$t, L>> as Codec>::Size;
		}

		impl<'a, L: Unsigned> Encode for CompactRef<'a, VecArray<$t, L>> where
			CompactRef<'a, VecArray<$t, L>>: Codec
		{
			fn encode(&self) -> Vec<u8> {
				encode_list(self.0)
			}
		}

		impl<L: Unsigned> Encode for Compact<VecArray<$t, L>> where
			Compact<VecArray<$t, L>>: Codec,
			for<'a> CompactRef<'a, GenericArray<$t, L>>: Encode
		{
			fn encode(&self) -> Vec<u8> {
				CompactRef(&self.0).encode()
			}
		}

		impl<L: Unsigned> Decode for Compact<VecArray<$t, L>> where
			Compact<VecArray<$t, L>>: Codec,
		{
			fn decode(value: &[u8]) -> Result<Self, Error> {
				let decoded = decode_list::<$t>(value)?;
				if decoded.len() != L::to_usize() {
					return Err(Error::InvalidLength)
				}
				let mut ret = VecArray::default();
				for i in 0..decoded.len() {
					ret[i] = decoded[i];
				}
				Ok(Compact(ret))
			}
		}
	)* }
}

impl_builtin_fixed_uint_vector!(u8, u16, u32, u64, u128);

impl<L: ArrayLength<bool> + Unsigned> Codec for Compact<GenericArray<bool, L>> {
	type Size = Div<Add<L, typenum::U7>, typenum::U8>;
}

impl<'a, L: ArrayLength<bool> + Unsigned> Codec for CompactRef<'a, GenericArray<bool, L>> where
	Compact<GenericArray<bool, L>>: Codec,
{
	type Size = <Compact<GenericArray<bool, L>> as Codec>::Size;
}

impl<'a, L: ArrayLength<bool>> Encode for CompactRef<'a, GenericArray<bool, L>> where
	CompactRef<'a, GenericArray<bool, L>>: Codec
{
	fn encode(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
        bytes.resize((self.0.len() + 7) / 8, 0u8);

        for i in 0..self.0.len() {
            bytes[i / 8] |= (self.0[i] as u8) << (i % 8);
        }
		bytes
	}
}

impl<L: ArrayLength<bool>> Encode for Compact<GenericArray<bool, L>> where
	Compact<GenericArray<bool, L>>: Codec,
	for<'a> CompactRef<'a, GenericArray<bool, L>>: Encode
{
	fn encode(&self) -> Vec<u8> {
		CompactRef(&self.0).encode()
	}
}

impl<L: ArrayLength<bool>> Decode for Compact<GenericArray<bool, L>> where
	Compact<GenericArray<bool, L>>: Codec,
{
	fn decode(value: &[u8]) -> Result<Self, Error> {
		let len = L::to_usize();
		let mut ret = GenericArray::default();
		for i in 0..len {
			if i / 8 >= value.len() {
				return Err(Error::IncorrectSize)
			}
			ret[i] = value[i / 8] & (1 << (i % 8)) != 0;
		}
		Ok(Compact(ret))
	}
}

impl<T: Codec, L: ArrayLength<T>> Codec for GenericArray<T, L> {
	type Size = Mul<<T as Codec>::Size, L>;
}

impl<T: Encode, L: ArrayLength<T>> Encode for GenericArray<T, L> where
	GenericArray<T, L>: Codec
{
	fn encode(&self) -> Vec<u8> {
		encode_list(&self)
	}
}

impl<T: Decode, L: ArrayLength<T>> Decode for GenericArray<T, L> where
	GenericArray<T, L>: Codec
{
	fn decode(value: &[u8]) -> Result<Self, Error> {
		let decoded = decode_list::<T>(value)?;

		GenericArray::from_exact_iter(decoded).ok_or(Error::InvalidLength)
	}
}

impl<L: Unsigned> Codec for Compact<VecArray<bool, L>> {
	type Size = Div<Add<L, typenum::U7>, typenum::U8>;
}

impl<'a, L: Unsigned> Codec for CompactRef<'a, VecArray<bool, L>> where
	Compact<VecArray<bool, L>>: Codec,
{
	type Size = <Compact<VecArray<bool, L>> as Codec>::Size;
}

impl<'a, L: Unsigned> Encode for CompactRef<'a, VecArray<bool, L>> where
	CompactRef<'a, VecArray<bool, L>>: Codec
{
	fn encode(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
        bytes.resize((self.0.len() + 7) / 8, 0u8);

        for i in 0..self.0.len() {
            bytes[i / 8] |= (self.0[i] as u8) << (i % 8);
        }
		bytes
	}
}

impl<L: Unsigned> Encode for Compact<VecArray<bool, L>> where
	Compact<VecArray<bool, L>>: Codec,
	for<'a> CompactRef<'a, VecArray<bool, L>>: Encode
{
	fn encode(&self) -> Vec<u8> {
		CompactRef(&self.0).encode()
	}
}

impl<L: Unsigned> Decode for Compact<VecArray<bool, L>> where
	Compact<VecArray<bool, L>>: Codec,
{
	fn decode(value: &[u8]) -> Result<Self, Error> {
		let len = L::to_usize();
		let mut ret = VecArray::default();
		for i in 0..len {
			if i / 8 >= value.len() {
				return Err(Error::IncorrectSize)
			}
			ret[i] = value[i / 8] & (1 << (i % 8)) != 0;
		}
		Ok(Compact(ret))
	}
}

impl<T: Codec, L: Unsigned> Codec for VecArray<T, L> {
	type Size = Mul<<T as Codec>::Size, L>;
}

impl<T: Encode, L: Unsigned> Encode for VecArray<T, L> where
	VecArray<T, L>: Codec
{
	fn encode(&self) -> Vec<u8> {
		encode_list(&self)
	}
}

impl<T: Decode, L: Unsigned> Decode for VecArray<T, L> where
	VecArray<T, L>: Codec
{
	fn decode(value: &[u8]) -> Result<Self, Error> {
		let decoded = decode_list::<T>(value)?;

		VecArray::try_from(decoded).map_err(|_| Error::InvalidLength)
	}
}

#[macro_export]
macro_rules! impl_type {
	($t:ty, $base:ty) => {
		impl $crate::Codec for $t {
			type Size = <$base as $crate::Codec>::Size;
		}
		impl $crate::Encode for $t {
			fn encode(&self) -> Vec<u8> {
				let n: $base = self.into();
				$crate::Encode::encode(&n)
			}
		}
		impl $crate::Decode for $t {
			fn decode(value: &[u8]) -> Result<Self, $crate::Error> {
				let n: $base = $crate::Decode::decode(value)?;
				Ok(n.into())
			}
		}
	};
}

macro_rules! ssz_array {
	($t:ty, $base:ty, $count:ty) => {
		impl $crate::Codec for $t {
			type Size = <$crate::Compact<$crate::GenericArray<$base, $count>> as $crate::Codec>::Size;
		}
		impl Encode for $t {
			fn encode(&self) -> Vec<u8> {
				$crate::CompactRef($crate::GenericArray::<$base, $count>::from_slice(&self[..])).encode()
			}
		}
		impl Decode for $t {
			fn decode(value: &[u8]) -> Result<Self, Error> {
				let decoded = Compact::<$crate::GenericArray<$base, $count>>::decode(value)?;
				let mut data: $t = unsafe { std::mem::zeroed() };
				data.copy_from_slice(decoded.0.as_slice());
				Ok(data)
			}
		}
	};
}

ssz_array!([u8; 1], u8, typenum::U1);
ssz_array!([u8; 4], u8, typenum::U4);
ssz_array!([u8; 8], u8, typenum::U8);
ssz_array!([u8; 20], u8, typenum::U20);
ssz_array!([u8; 32], u8, typenum::U32);
ssz_array!([u8; 48], u8, typenum::U48);
ssz_array!([u64; 1], u64, typenum::U1);
ssz_array!([u64; 4], u64, typenum::U4);

impl Codec for H256 {
	type Size = <Compact<GenericArray<u8, typenum::U32>> as Codec>::Size;
}

impl Encode for H256 {
	fn encode(&self) -> Vec<u8> {
		CompactRef(GenericArray::<u8, typenum::U32>::from_slice(&self.0[..])).encode()
	}
}

impl Decode for H256 {
	fn decode(value: &[u8]) -> Result<Self, Error> {
		let decoded = Compact::<GenericArray<u8, typenum::U32>>::decode(value)?;
		Ok(H256::from_slice(decoded.0.as_slice()))
	}
}