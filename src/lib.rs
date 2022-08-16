#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]

use core::ops::{Add, BitAnd, Not, Sub};

/// The trait for numbers in this library.
///
/// This allows the rounding functions to work with any primitive integer type.
pub trait Num: Sub<Output = Self> + Not<Output = Self> + BitAnd<Output = Self> + Sized {
	/// The non-zero type for this number.
	/// # Examples
	/// ```
	/// # use round_mult::Num;
	/// use std::any::{Any, TypeId};
	///
	/// assert_eq!(
	///     <usize as Num>::NonZero::new(10).unwrap().type_id(),
	///     TypeId::of::<std::num::NonZeroUsize>(),
	/// );
	/// ```
	type NonZero: Into<Self>;

	/// Returns the value for the number one in this type's representation.
	///
	/// # Examples
	/// ```
	/// # use round_mult::Num;
	/// assert_eq!(
	///     <u8 as Num>::ONE,
	///     1u8
	/// );
	/// assert_eq!(
	///     <i32 as Num>::ONE,
	///     1i32
	/// );
	/// assert_eq!(
	///     <usize as Num>::ONE,
	///     1usize
	/// );
	/// ```
	const ONE: Self;
}

mod num_impl {
	use crate::Num;
	use core::{
		num::{
			NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
			NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
		},
		ops::{BitAnd, Not, Sub},
	};

	macro_rules! impl_num {
	($($t:ty : $nz:ty),+) => {
		$(
		impl Num for $t {
			type NonZero = $nz;
			const ONE: Self = 1;
		}
		)+
	}
}
	impl_num!(
		u8: NonZeroU8,
		u16: NonZeroU16,
		u32: NonZeroU32,
		u64: NonZeroU64,
		u128: NonZeroU128,
		usize: NonZeroUsize,
		i8: NonZeroI8,
		i16: NonZeroI16,
		i32: NonZeroI32,
		i64: NonZeroI64,
		i128: NonZeroI128,
		isize: NonZeroIsize
	);

	impl<T: Num> Num for core::num::Wrapping<T>
	where
		Self: Sub<Output = Self> + Not<Output = Self> + BitAnd<Output = Self>,
		T::NonZero: Into<Self>,
	{
		type NonZero = T::NonZero;
		const ONE: Self = Self(T::ONE);
	}
}

/// Rounds the `value` down to the nearest multiplier of `mult`.
///
/// # Examples
/// ```
/// # use core::num::NonZeroUsize;
/// assert_eq!(
///     round_mult::down(109usize, NonZeroUsize::new(10).unwrap()),
///     100
/// );
/// ```
#[inline(always)]
pub fn down<N: Num>(value: N, mult: N::NonZero) -> N {
	value & !(mult.into() - N::ONE)
}

/// Rounds the `value` up to the nearest multiplier of `mult`.
///
/// # Examples
/// ```
/// # use core::num::NonZeroUsize;
/// assert_eq!(
///     round_mult::up(101usize, NonZeroUsize::new(10).unwrap()),
///     110
/// );
/// ```
#[inline(always)]
pub fn up<N: Num + Add<Output = N>>(value: N, mult: N::NonZero) -> N
where
	N::NonZero: Copy,
{
	// TODO: specialized implementation
	down(value, mult) + mult.into()
}

#[cfg(test)]
mod test {
	use core::num::NonZeroUsize;

	use super::*;
	use quickcheck::TestResult;
	use quickcheck_macros::quickcheck;

	fn is_power_of_2(n: NonZeroUsize) -> bool {
		let n = n.get();
		n & (n - 1) == 0
	}

	#[quickcheck]
	fn round_down_is_correct(value: usize, mult: NonZeroUsize) -> TestResult {
		if !is_power_of_2(mult) {
			return TestResult::discard();
		}
		TestResult::from_bool(down(value, mult) == (value / mult) * mult.get())
	}

	#[quickcheck]
	fn round_up_is_correct(value: usize, mult: NonZeroUsize) -> TestResult {
		if !is_power_of_2(mult) {
			return TestResult::discard();
		}
		TestResult::from_bool(up(value, mult) == ((value / mult) * mult.get()) + mult.get())
	}
}
