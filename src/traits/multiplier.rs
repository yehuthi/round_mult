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
	($($ty:ty),* $(,)?) => {
		$(
			impl public::Multiplier for <$ty as crate::traits::nonzero::public::NonZeroable>::NonZeroType {
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

			impl Multiplier for <$ty as crate::traits::nonzero::public::NonZeroable>::NonZeroType {}
		)*
	};
}

impl_unsigned_number!(u8, u16, u32, u64, u128, usize);

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
