//! [`Number`]

pub(crate) mod private {
	use core::ops::{Add, BitAnd, Not, Sub};

	pub trait Number:
		Copy
		+ PartialEq
		+ Add<Output = Self>
		+ Sub<Output = Self>
		+ Not<Output = Self>
		+ BitAnd<Output = Self>
	{
		const BITS: u32;
		const ONE: Self;
		fn checked_add(self, rhs: Self) -> Option<Self>;
	}
}

/// A primitive number.
///
/// This trait is an implementation detail that provides some operations.
/// For users of the library, the only things that matter are when it's required and the set of types that implement it.
pub trait Number: private::Number {}

macro_rules! impl_number {
	($($ty:ty),* $(,)?) => {
		$(
			impl private::Number for $ty {
				const BITS: u32 = <$ty>::BITS;
				const ONE: Self = 1;

				#[inline(always)]
				fn checked_add(self, rhs: Self) -> Option<Self> {
					<$ty>::checked_add(self, rhs)
				}
			}

			impl Number for $ty {}
		)*
	}
}

impl_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
