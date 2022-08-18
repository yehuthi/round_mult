mod nzp;

use core::num::NonZeroU8;

use nzp::private::NonZeroable;
pub use nzp::NonZeroPow2;
use private::{Multiplier, Number};

mod private {
	use std::ops::{Add, BitAnd, Not, Sub};

	pub trait Multiplier {
		type Number;

		fn get(self) -> Self::Number;

		fn down(self, value: Self::Number) -> Self::Number;
		fn up(self, value: Self::Number) -> Option<Self::Number>;
	}

	pub trait Number:
		Copy
		+ PartialEq
		+ Add<Output = Self>
		+ Sub<Output = Self>
		+ Not<Output = Self>
		+ BitAnd<Output = Self>
	{
		const ONE: Self;
		fn checked_add(self, rhs: Self) -> Option<Self>;
	}
}

impl<N> Multiplier for NonZeroPow2<N>
where
	N: NonZeroable + Number,
	Self: Copy,
{
	type Number = N;

	#[inline(always)]
	fn get(self) -> Self::Number {
		self.get()
	}

	#[inline(always)]
	fn down(self, value: Self::Number) -> Self::Number {
		value & !(self.get() - N::ONE)
	}

	#[inline(always)]
	fn up(self, value: Self::Number) -> Option<Self::Number> {
		if self.get() == value {
			return Some(value);
		}
		self.down(value).checked_add(self.get())
	}
}

impl Multiplier for NonZeroU8 {
	type Number = u8;

	#[inline(always)]
	fn get(self) -> Self::Number {
		self.get()
	}

	#[inline]
	fn down(self, value: Self::Number) -> Self::Number {
		if value % self.get() != 0 {
			value / self.get() * self.get()
		} else {
			value
		}
	}

	#[inline]
	fn up(self, value: Self::Number) -> Option<Self::Number> {
		let r = value % self;
		if r == 0 {
			Some(value)
		} else {
			value.checked_add(self.get() - r)
		}
	}
}

macro_rules! impl_number {
	($($ty:ty),* $(,)?) => {
		$(
			impl Number for $ty {
				const ONE: Self = 1;

				#[inline(always)]
				fn checked_add(self, rhs: Self) -> Option<Self> {
					<$ty>::checked_add(self, rhs)
				}
			}
		)*
	}
}

impl_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[inline(always)]
pub fn down<M: Multiplier>(value: M::Number, multiplier: M) -> M::Number {
	multiplier.down(value)
}

#[inline(always)]
pub fn up<M: Multiplier>(value: M::Number, multiplier: M) -> Option<M::Number> {
	multiplier.up(value)
}

#[cfg(test)]
mod test {
	use super::*;
	use quickcheck::TestResult;
	use quickcheck_macros::quickcheck;

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
		if value % mult.get() != 0 && u8::MAX - ((value / mult) * mult.get()) < mult.get() {
			TestResult::from_bool(up(value, mult).is_none())
		} else {
			TestResult::discard()
		}
	}

	#[quickcheck]
	fn mult2_up_overflow_is_none(value: u8, mult: NonZeroPow2<u8>) -> TestResult {
		if value % mult.get() != 0 && u8::MAX - ((value / mult.get()) * mult.get()) < mult.get() {
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
}
