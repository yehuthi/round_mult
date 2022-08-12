use std::ops::{BitAnd, Not, Sub};

pub trait Num: Sub<Output = Self> + Not<Output = Self> + BitAnd<Output = Self> + Sized {
	fn one() -> Self;
}

impl<
		T: num_traits::One + Sub<Output = Self> + Not<Output = Self> + BitAnd<Output = Self> + Sized,
	> Num for T
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
