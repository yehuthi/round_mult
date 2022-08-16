#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]

use core::ops::{Add, BitAnd, Not, Sub};

/// The trait for numbers in this library.
///
/// This allows the rounding functions to work with any primitive integer type.
pub trait Num: Sub<Output = Self> + Not<Output = Self> + BitAnd<Output = Self> + Sized {
	/// Returns the value for the number one in this type's representation.
	///
	/// # Examples
	/// ```
	/// # use round_mult::Num;
	/// assert_eq!(
	///     <u8 as Num>::one(),
	///     1u8
	/// );
	/// assert_eq!(
	///     <i32 as Num>::one(),
	///     1i32
	/// );
	/// assert_eq!(
	///     <usize as Num>::one(),
	///     1usize
	/// );
	/// ```
	fn one() -> Self;
}

#[cfg(not(feature = "num-traits"))]
mod num_impl {
	use crate::Num;
	use core::ops::{BitAnd, Not, Sub};

	macro_rules! impl_num {
	($($t:ty),+) => {
		$(
		impl Num for $t { fn one() -> Self { 1 } }
		)+
	}
}

	impl_num!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

	impl<T: Num> Num for core::num::Wrapping<T>
	where
		Self: Sub<Output = Self> + Not<Output = Self> + BitAnd<Output = Self>,
	{
		fn one() -> Self {
			Self(T::one())
		}
	}
}

#[cfg(feature = "num-traits")]
impl<T: num_traits::One + Sub<Output = Self> + Not<Output = Self> + BitAnd<Output = Self>> Num
	for T
{
	fn one() -> Self {
		<T as num_traits::One>::one()
	}
}

/// Rounds the `value` down to the nearest multiplier of `mult`.
///
/// # Examples
/// ```
/// assert_eq!(
///     round_mult::down(109, 10),
///     100
/// );
/// ```
#[inline(always)]
pub fn down<N: Num>(value: N, mult: N) -> N {
	value & !(mult - N::one())
}

/// Rounds the `value` up to the nearest multiplier of `mult`.
///
/// # Examples
/// ```
/// assert_eq!(
///     round_mult::up(101, 10),
///     110
/// );
/// ```
#[inline(always)]
pub fn up<N: Num + Add<Output = N> + Copy>(value: N, mult: N) -> N {
	// TODO: specialized implementation
	down(value, mult) + mult
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
		let mult = mult.get();
		TestResult::from_bool(down(value, mult) == (value / mult) * mult)
	}

	#[quickcheck]
	fn round_up_is_correct(value: usize, mult: NonZeroUsize) -> TestResult {
		if !is_power_of_2(mult) {
			return TestResult::discard();
		}
		let mult = mult.get();
		TestResult::from_bool(up(value, mult) == ((value / mult) * mult) + mult)
	}
}
