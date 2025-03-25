//! [`Multiplier`]

use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

use crate::NonZeroPow2;

use super::{NonZeroable, Number};

pub mod public {
	/// See [`Multiplier`](crate::traits::Multiplier).
	pub trait Multiplier {
		/// The type of the multiplied number of this multiplier.
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

/// A numeric type that may act as a multiplier for another number.
///
/// This helps add safety and optimization:
/// - NonZeroT is a multiplier where T is not, so we don't divide by zero by accident.
/// - [`NonZeroPow2`](crate::NonZeroPow2) takes advanage of its invariant to calculate results faster.
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
		let downed = self.down(value);
		if downed == value {
			return Some(value);
		}
		downed.checked_add(self.get())
	}
}

impl<N: NonZeroable + Number> Multiplier for NonZeroPow2<N> where Self: Copy {}
