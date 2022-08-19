//! [`NonZeroable`]

use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

pub mod public {
	pub trait NonZeroable: Copy {
		type NonZeroType: super::private::NonZero<Number = Self>;
	}
}

pub(crate) mod private {
	pub trait NonZeroable: super::public::NonZeroable
	where
		Self::NonZeroType: super::private::NonZero,
	{
		fn is_zero(self) -> bool;
		fn is_power_of_two(self) -> bool;
	}

	pub trait NonZero: Into<Self::Number> {
		type Number;

		unsafe fn new_unchecked(value: Self::Number) -> Self;
	}
}

macro_rules! impl_nonzero_traits {
	($($ty:ty: $nz:ty),* $(,)?) => {
		$(
			impl public::NonZeroable for $ty {
				type NonZeroType = $nz;
			}

			impl private::NonZeroable for $ty {
				#[inline(always)]
				fn is_zero(self) -> bool {
					self == 0
				}

				#[inline(always)]
				fn is_power_of_two(self) -> bool {
					<$ty>::is_power_of_two(self)
				}
			}
			impl NonZeroable for $ty {}

			impl private::NonZero for $nz {
				type Number = $ty;

				#[inline(always)]
				unsafe fn new_unchecked(value: Self::Number) -> Self {
					<$nz>::new_unchecked(value)
				}
			}
		)*
	};
}

impl_nonzero_traits!(
	u8: NonZeroU8,
	u16: NonZeroU16,
	u32: NonZeroU32,
	u64: NonZeroU64,
	u128: NonZeroU128,
	usize: NonZeroUsize,
	// need own impl as they don't have is_power_of_two
	// i8: NonZeroI8,
	// i16: NonZeroI16,
	// i32: NonZeroI32,
	// i64: NonZeroI64,
	// i128: NonZeroI128,
	// isize: NonZeroIsize,
);
pub trait NonZeroable: public::NonZeroable + private::NonZeroable {}
