//! [`NonZeroPow2`]

#[cfg(nightly)]
mod std_simd;
pub use std_simd::LanesMult;

use core::ops::Shl;

use crate::traits::{NonZeroable, Number};

/// A number that is non-zero and is a power of two.
#[repr(transparent)]
#[derive(Debug, Hash, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct NonZeroPow2<N: NonZeroable>(N::NonZeroType);

impl<N: NonZeroable> NonZeroPow2<N>
where
	N: Number + Shl<u32, Output = N>,
{
	/// The value 2.
	///
	/// # Examples
	/// ```
	/// # use round_mult::NonZeroPow2;
	/// assert_eq!(NonZeroPow2::<u32>::v2().get(), 2);
	/// ```
	#[inline(always)]
	pub fn v2() -> Self {
		unsafe { Self::new_unchecked(N::ONE << 1) }
	}

	/// The value 4.
	///
	/// # Examples
	/// ```
	/// # use round_mult::NonZeroPow2;
	/// assert_eq!(NonZeroPow2::<u32>::v4().get(), 4);
	/// ```
	#[inline(always)]
	pub fn v4() -> Self {
		unsafe { Self::new_unchecked(Self::v2().get() << 1) }
	}

	/// The value 8.
	///
	/// # Examples
	/// ```
	/// # use round_mult::NonZeroPow2;
	/// assert_eq!(NonZeroPow2::<u32>::v8().get(), 8);
	/// ```
	#[inline(always)]
	pub fn v8() -> Self {
		unsafe { Self::new_unchecked(Self::v4().get() << 1) }
	}

	/// The value 16.
	///
	/// # Examples
	/// ```
	/// # use round_mult::NonZeroPow2;
	/// assert_eq!(NonZeroPow2::<u32>::v16().get(), 16);
	/// ```
	#[inline(always)]
	pub fn v16() -> Self {
		unsafe { Self::new_unchecked(Self::v8().get() << 1) }
	}

	/// The value 32.
	///
	/// # Examples
	/// ```
	/// # use round_mult::NonZeroPow2;
	/// assert_eq!(NonZeroPow2::<u32>::v32().get(), 32);
	/// ```
	#[inline(always)]
	pub fn v32() -> Self {
		unsafe { Self::new_unchecked(Self::v16().get() << 1) }
	}

	/// The value 64.
	///
	/// # Examples
	/// ```
	/// # use round_mult::NonZeroPow2;
	/// assert_eq!(NonZeroPow2::<u32>::v64().get(), 64);
	/// ```
	#[inline(always)]
	pub fn v64() -> Self {
		unsafe { Self::new_unchecked(Self::v32().get() << 1) }
	}

	/// The value 128.
	///
	/// # Examples
	/// ```
	/// # use round_mult::NonZeroPow2;
	/// assert_eq!(NonZeroPow2::<u32>::v128().get(), 128);
	/// ```
	#[inline(always)]
	pub fn v128() -> Self {
		unsafe { Self::new_unchecked(Self::v64().get() << 1) }
	}
}

impl<N: NonZeroable> NonZeroPow2<N> {
	/// Creates a new [`NonZeroPow2`].
	///
	/// # Safety
	/// Ensure the value is not zero and is a power of two.
	#[inline(always)]
	pub unsafe fn new_unchecked(value: N) -> Self {
		Self(<N::NonZeroType as crate::traits::nonzero::private::NonZero>::new_unchecked(value))
	}

	/// Creates a new [`NonZeroPow2`].
	///
	/// # Safety
	/// Ensure the value is a power of two.
	#[inline(always)]
	pub const unsafe fn from_nonzero_unchecked(value: N::NonZeroType) -> Self {
		Self(value)
	}

	/// Creates a new [`NonZeroPow2`].
	///
	/// Returns [`None`] if the given value is zero or not a power of two.
	///
	/// # Examples
	/// ```
	/// # use round_mult::NonZeroPow2;
	/// assert!(NonZeroPow2::new(0usize).is_none());
	/// assert!(NonZeroPow2::new(2usize).is_some());
	/// assert!(NonZeroPow2::new(4usize).is_some());
	/// assert!(NonZeroPow2::new(6usize).is_none());
	/// ```
	#[inline]
	pub fn new(value: N) -> Option<Self> {
		(!value.is_zero() && value.is_power_of_two()).then(|| unsafe { Self::new_unchecked(value) })
	}

	/// Gets the value of the number in its primitive representation.
	///
	/// # Examples
	/// ```
	/// # use round_mult::NonZeroPow2;
	/// assert_eq!(
	///     NonZeroPow2::new(32usize).unwrap().get(),
	///     32usize,
	/// );
	/// ```
	#[inline(always)]
	pub fn get(self) -> N {
		self.0.into()
	}

	/// Gets the value of the number in its nonzero representation.
	///
	/// # Examples
	/// ```
	/// # use round_mult::NonZeroPow2;
	/// # use core::num::NonZeroUsize;
	/// assert_eq!(
	///     NonZeroPow2::new(32usize).unwrap().get_nonzero(),
	///     NonZeroUsize::new(32usize).unwrap(),
	/// );
	/// ```
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
