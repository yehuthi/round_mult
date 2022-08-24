use core::{ops::Shl, simd::{Simd, SimdElement, Mask, MaskElement, LaneCount}};

use crate::{NonZeroPow2, traits::{NonZeroable, Number}};

mod private {
    pub trait Sealed {}
}

pub trait LanesMult: private::Sealed {
    fn lanes_mult<N: NonZeroable + Number + Shl<u32, Output=N>>() -> NonZeroPow2<N>;
}

macro_rules! impl_lanes_mult {
    ($($n:expr => $v:expr),* $(,)?) => {
        $(
            impl<T: SimdElement> private::Sealed for Simd<T, $n> {}
            impl<T: MaskElement> private::Sealed for Mask<T, $n> {}
            impl private::Sealed for LaneCount<$n> {}
            impl<T: SimdElement> LanesMult for Simd<T, $n> {
                #[inline(always)]
                fn lanes_mult<N: NonZeroable + Number + Shl<u32, Output=N>>() -> NonZeroPow2<N> {
                    $v()
                }
            }
            impl<T: MaskElement> LanesMult for Mask<T, $n> {
                #[inline(always)]
                fn lanes_mult<N: NonZeroable + Number + Shl<u32, Output=N>>() -> NonZeroPow2<N> {
                    $v()
                }
            }
            impl LanesMult for LaneCount<$n> {
                #[inline(always)]
                fn lanes_mult<N: NonZeroable + Number + Shl<u32, Output=N>>() -> NonZeroPow2<N> {
                    $v()
                }
            }
        )*
    };
}

impl_lanes_mult!(
    2  => NonZeroPow2::v2 ,
    4  => NonZeroPow2::v4 ,
    8  => NonZeroPow2::v8 ,
    16 => NonZeroPow2::v16,
    32 => NonZeroPow2::v32,
    64 => NonZeroPow2::v64,
);


impl<N: NonZeroable + Shl<u32, Output = N> + Number> NonZeroPow2<N> {
    /// Gets the multiplier for the given SIMD type.
    #[inline(always)]
    pub fn of<T: LanesMult>() -> Self { T::lanes_mult() }
}
