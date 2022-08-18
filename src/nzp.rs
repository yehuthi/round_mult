use core::num::{
	NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
	NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use self::private::{NonZero, NonZeroable};

#[repr(transparent)]
#[derive(Debug, Hash, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct NonZeroPow2<N: NonZeroable>(N::NonZeroType);

impl<N: NonZeroable> NonZeroPow2<N> {
	/// Creates a new [`NonZeroPow2`].
	///
	/// # Safety
	/// Ensure the value is not zero and is a power of two.
	pub unsafe fn new_unchecked(value: N) -> Self {
		Self(N::NonZeroType::new_unchecked(value))
	}

	/// Creates a new [`NonZeroPow2`].
	///
	/// # Safety
	/// Ensure the value is a power of two.
	pub const unsafe fn from_nonzero_unchecked(value: N::NonZeroType) -> Self {
		Self(value)
	}

	/// Creates a new [`NonZeroPow2`].
	///
	/// Returns [`None`] if the given value is zero or not a power of two.
	#[inline]
	pub fn new(value: N) -> Option<Self> {
		(!value.is_zero() && value.is_power_of_two()).then(|| unsafe { Self::new_unchecked(value) })
	}

	#[inline(always)]
	pub fn get(self) -> N {
		self.0.into()
	}

	#[inline(always)]
	pub fn get_nonzero(self) -> N::NonZeroType {
		self.0
	}
}

macro_rules! impl_nonzero_traits {
	($($ty:ty: $nz:ty),* $(,)?) => {
		$(
			impl NonZeroable for $ty {
				type NonZeroType = $nz;

				#[inline(always)]
				fn is_zero(self) -> bool {
					self == 0
				}

				#[allow(unconditional_recursion)] // false positive
				#[inline(always)]
				fn is_power_of_two(self) -> bool {
					<$ty>::is_power_of_two(self)
				}
			}

			impl NonZero for $nz {
				type Number = $ty;

				#[inline(always)]
				unsafe fn new_unchecked(value: Self::Number) -> Self {
					Self::new_unchecked(value)
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
	i8: NonZeroI8,
	i16: NonZeroI16,
	i32: NonZeroI32,
	i64: NonZeroI64,
	i128: NonZeroI128,
	isize: NonZeroIsize,
);

pub(crate) mod private {
	pub trait NonZeroable: Copy {
		type NonZeroType: NonZero<Number = Self>;

		fn is_zero(self) -> bool;
		fn is_power_of_two(self) -> bool;
	}

	pub trait NonZero: Into<Self::Number> {
		type Number;

		unsafe fn new_unchecked(value: Self::Number) -> Self;
	}
}
#[cfg(test)]
mod arbitrary_impl {
	use super::*;
	use core::ops::Shl;
	use quickcheck::Arbitrary;

	impl<N: NonZeroable + crate::Number> Arbitrary for NonZeroPow2<N>
	where
		Self: 'static + Clone,
		N: Shl<u8, Output = N>,
	{
		#[inline]
		fn arbitrary(g: &mut quickcheck::Gen) -> Self {
			Self::new((N::ONE << 1) << (u8::arbitrary(g) % 7)).unwrap()
		}
	}
}
