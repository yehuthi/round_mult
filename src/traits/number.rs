pub(crate) mod private {
	use std::ops::{Add, BitAnd, Not, Sub};

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

pub trait Number: private::Number {}

macro_rules! impl_number {
	($($ty:ty),* $(,)?) => {
		$(
			impl private::Number for $ty {
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
