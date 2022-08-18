//! [`NonZeroPow2`]

use crate::traits::NonZeroable;

/// A number that is non-zero and is a power of two.
#[repr(transparent)]
#[derive(Debug, Hash, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct NonZeroPow2<N: NonZeroable>(N::NonZeroType);

impl<N: NonZeroable> NonZeroPow2<N> {
	/// Creates a new [`NonZeroPow2`].
	///
	/// # Safety
	/// Ensure the value is not zero and is a power of two.
	pub unsafe fn new_unchecked(value: N) -> Self {
		Self(<N::NonZeroType as crate::traits::nonzero::private::NonZero>::new_unchecked(value))
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
	/// ```
	#[inline]
	pub fn new(value: N) -> Option<Self> {
		(!value.is_zero() && value.is_power_of_two()).then(|| unsafe { Self::new_unchecked(value) })
	}

	/// Gets the value of the number in its primitive representation.
	#[inline(always)]
	pub fn get(self) -> N {
		self.0.into()
	}

	/// Gets the value of the number in its nonzero representation.
	#[inline(always)]
	pub fn get_nonzero(self) -> N::NonZeroType {
		self.0
	}
}

#[cfg(test)]
mod arbitrary_impl {
	use crate::traits::Number;

	use super::*;
	use core::ops::Shl;
	use quickcheck::Arbitrary;

	impl<N: NonZeroable + Number> Arbitrary for NonZeroPow2<N>
	where
		Self: 'static + Clone,
		N: Shl<u32, Output = N>,
	{
		#[inline]
		fn arbitrary(g: &mut quickcheck::Gen) -> Self {
			Self::new((N::ONE << 1) << (u32::arbitrary(g) % (N::BITS - 1))).unwrap()
		}
	}
}
