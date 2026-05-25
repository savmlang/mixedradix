#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

//! # Mixed Radix
//!
//! This crate heavily simplifies the process of Mixed Radix Crate Generation
//!
//! This crate provides two things
//! 1. `mixedradix!` macro
//! 2. `Compact<T>` Container for serde

#[cfg(feature = "serde")]
use core::{
  convert::From,
  ops::{Deref, DerefMut},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize};

pub use mixedradix_macros::*;

pub trait MixedRadixStructure
where
  Self: Sized,
{
  type BitsType;

  const STORAGE_BITS: u8;
  const MAXIMUM_VALUE: Self::BitsType;

  /// # Panic
  /// This function panics on **DEBUG MODE ONLY** due to overflow of fields.
  fn bits(&self) -> Self::BitsType;

  /// # None
  /// This returns `None` only in the case of overflow of its fields
  fn try_bits(&self) -> Option<Self::BitsType>;

  /// # Panic
  /// This function panics on **DEBUG MODE ONLY** in case of overflow of "total" from the hypothetical maximum value
  fn from_bits(total: Self::BitsType) -> Self;

  /// # None
  /// This returns `None` only in the case of overflow of "total" from the hypothetical maximum value
  fn try_from_bits(total: Self::BitsType) -> Option<Self>;
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
#[repr(transparent)]
/// This transparent structure implements [serde::Serialize] for anything that implements [MixedRadixStructure]
///
/// ## Why not directly apply Serialize to the structure
///
/// 1. The main reason is that rust does not allow this level of blanket trait implementation and that it is bad for the macro parser to add thousands of #[cfg_attr(...)] blocks.
/// 2. Another reason is that sometimes you do want the `MyStructure` to have a different `Serialize` type than the compacted bits.
///
/// ## Usage in struct
/// ```rust
/// #[derive(serde::Serialize)]
/// struct MySerdeStructure {
///   my_compact_structure: Compact<MyStructure>
/// }
/// ```
pub struct Compact<T: MixedRadixStructure>(pub T);

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<T: MixedRadixStructure> Deref for Compact<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<T: MixedRadixStructure> DerefMut for Compact<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<T: MixedRadixStructure> From<T> for Compact<T> {
  #[inline]
  fn from(value: T) -> Self {
    Compact(value)
  }
}
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<T: MixedRadixStructure> Compact<T> {
  /// Extract the inner mixed-radix structure.
  #[inline]
  pub fn into_inner(self) -> T {
    self.0
  }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<T: MixedRadixStructure> Serialize for Compact<T>
where
  T::BitsType: Serialize,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let data = self
      .0
      .try_bits()
      .ok_or_else(|| serde::ser::Error::custom("invalid mixed-radix value : bit overflowed"))?;
    Serialize::serialize(&data, serializer)
  }
}
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de, T: MixedRadixStructure> Deserialize<'de> for Compact<T>
where
  T::BitsType: Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let bits = T::BitsType::deserialize(deserializer)?;

    T::try_from_bits(bits)
      .map(Compact)
      .ok_or_else(|| serde::de::Error::custom("invalid mixed-radix value: bit overflow"))
  }
}
