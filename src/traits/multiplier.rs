use std::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

use crate::{private::Number, NonZeroPow2};

use super::NonZeroable;

pub mod public {
	pub trait Multiplier {
		type Number;
	}
}

pub(crate) mod private {
	pub trait Multiplier: super::public::Multiplier {
		fn get(self) -> Self::Number;

		fn down(self, value: Self::Number) -> Self::Number;
		fn up(self, value: Self::Number) -> Option<Self::Number>;
	}
}

pub trait Multiplier: public::Multiplier + private::Multiplier {}

macro_rules! impl_unsigned_number {
	($($ty:ty : $nz:ty),* $(,)?) => {
		$(
			impl public::Multiplier for $nz {
				type Number = $ty;
			}

			impl private::Multiplier for <$ty as crate::traits::nonzero::public::NonZeroable>::NonZeroType {
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

			impl Multiplier for $nz {}
		)*
	};
}

impl_unsigned_number!(
	u8: NonZeroU8,
	u16: NonZeroU16,
	u32: NonZeroU32,
	u64: NonZeroU64,
	u128: NonZeroU128,
	usize: NonZeroUsize
);

impl<N: NonZeroable> public::Multiplier for NonZeroPow2<N> {
	type Number = N;
}

impl<N: NonZeroable + Number> private::Multiplier for NonZeroPow2<N>
where
	Self: Copy,
{
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

impl<N: NonZeroable + Number> Multiplier for NonZeroPow2<N> where Self: Copy {}
