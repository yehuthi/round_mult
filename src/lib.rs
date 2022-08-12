use std::ops::{Add, BitAnd, Not, Sub};

/// The trait for numbers in this library.
///
/// This allows the rounding functions to work with any primitive integer type.
pub trait Num: Sub<Output = Self> + Not<Output = Self> + BitAnd<Output = Self> + Sized {
	/// Returns the value for the number one in this type's representation.
	///
	/// # Examples
	/// ```
	/// # use round_mult::Num;
	/// assert_eq!(
	///     <u8 as Num>::one(),
	///    1u8
	/// );
	/// assert_eq!(
	///     <i32 as Num>::one(),
	///    1i32
	/// );
	/// assert_eq!(
	///     <usize as Num>::one(),
	///    1usize
	/// );
	/// ```
	fn one() -> Self;

	// Note: we don't use `num_traits::identities::One` because it requires std::ops::Mul which we don't need.
}

impl<T: num_traits::One + Sub<Output = Self> + Not<Output = Self> + BitAnd<Output = Self>> Num
	for T
{
	fn one() -> Self {
		<T as num_traits::One>::one()
	}
}

/// Rounds the `value` down to the nearest multiplier of `mult`.
///
/// # Examples
/// ```
/// assert_eq!(
///     round_mult::down(109, 10),
///     100
/// );
/// ```
#[inline(always)]
pub fn down<N: Num>(value: N, mult: N) -> N {
	value & !(mult - N::one())
}

/// Rounds the `value` up to the nearest multiplier of `mult`.
///
/// # Examples
/// ```
/// assert_eq!(
///     round_mult::up(101, 10),
///     110
/// );
/// ```
#[inline(always)]
pub fn up<N: Num + Add<Output = N> + Copy>(value: N, mult: N) -> N {
	// TODO: specialized implementation
	down(value, mult) + mult
}
