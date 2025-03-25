#![doc = include_str!("../README.md")]
#![no_std]
#![deny(clippy::missing_inline_in_public_items, missing_docs)]
#![cfg_attr(nightly, feature(portable_simd))]

mod nzp;
#[cfg(nightly)]
pub use nzp::LanesMult;
pub use nzp::NonZeroPow2;

pub mod traits;
use traits::Multiplier;

/// Rounds the number down.
#[inline(always)]
pub fn down<M: Multiplier>(
	value: M::Number, multiplier: M
) -> M::Number {
	multiplier.down(value)
}

/// Rounds the number up.
///
/// Returns [`None`] if the result overflows.
#[inline(always)]
pub fn up<M: Multiplier>(
	value: M::Number, multiplier: M
) -> Option<M::Number> {
	multiplier.up(value)
}

#[cfg(test)]
mod test {
	use core::num::NonZeroU8;

	use quickcheck::TestResult;
	use quickcheck_macros::quickcheck;

	use super::*;

	#[quickcheck]
	fn mult2_down_round_mult_is_identity(value: NonZeroPow2<u8>) -> bool {
		down(value.get(), value) == value.get()
	}

	#[quickcheck]
	fn mult2_up_round_mult_is_identity(value: NonZeroPow2<u8>) -> bool {
		up(value.get(), value) == Some(value.get())
	}

	#[quickcheck]
	fn mult_down_round_mult_is_identity(value: NonZeroU8) -> bool {
		down(value.get(), value) == value.get()
	}

	#[quickcheck]
	fn mult_up_round_mult_is_identity(value: NonZeroU8) -> bool {
		up(value.get(), value) == Some(value.get())
	}

	#[quickcheck]
	fn mult_up_overflow_is_none(value: u8, mult: NonZeroU8) -> TestResult {
		if
			value % mult.get() != 0 &&
			u8::MAX - ((value / mult) * mult.get()) < mult.get()
		{
			TestResult::from_bool(up(value, mult).is_none())
		} else {
			TestResult::discard()
		}
	}

	#[quickcheck]
	fn mult2_up_overflow_is_none(
		value: u8, mult: NonZeroPow2<u8>
	) -> TestResult {
		if
			value % mult.get() != 0 &&
			u8::MAX - ((value / mult.get()) * mult.get()) < mult.get()
		{
			TestResult::from_bool(up(value, mult).is_none())
		} else {
			TestResult::discard()
		}
	}

	#[quickcheck]
	fn mult2_down_is_correct(value: u8, mult: NonZeroPow2<u8>) -> bool {
		down(value, mult) == (value / mult.get()) * mult.get()
	}

	#[quickcheck]
	fn mult_down_is_correct(value: u8, mult: NonZeroU8) -> bool {
		down(value, mult) == (value / mult.get()) * mult.get()
	}

	#[test]
	fn up_12_4_identity() {
		assert_eq!(
			up(12_usize, NonZeroPow2::new(4).unwrap()).unwrap(),
			12
		);
	}
}
