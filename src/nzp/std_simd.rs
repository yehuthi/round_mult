use core::simd::{Simd, SimdElement, Mask, MaskElement, LaneCount};

use crate::{NonZeroPow2, traits::NonZeroable};

mod private {
    use core::ops::Shl;

    use crate::{traits::{NonZeroable, Number}, NonZeroPow2};

    pub trait LanesMultNum : NonZeroable + Number + Shl<u32, Output=Self> {}

    impl<T> LanesMultNum for T where T: NonZeroable + Number + Shl<u32, Output=Self> {}

    pub trait LanesMult {
        fn lanes_mult<N : LanesMultNum>() -> NonZeroPow2<N>;
    }
}

/// Gets the lanes multiplier from a SIMD type.
pub trait LanesMult: private::LanesMult {
    /// Gets the lanes multiplier from a SIMD type.
    #[inline(always)]
    fn lanes_mult<N: private::LanesMultNum>() -> NonZeroPow2<N> {
        <Self as private::LanesMult>::lanes_mult()
    }
}

macro_rules! impl_lanes_mult {
    ($($n:expr => $v:expr),* $(,)?) => {
        $(
            impl<T: SimdElement> LanesMult for Simd<T, $n> {}
            impl<T: MaskElement> LanesMult for Mask<T, $n> {}
            impl LanesMult for LaneCount<$n> {}
            impl<T: SimdElement> private::LanesMult for Simd<T, $n> {
                #[inline(always)]
                fn lanes_mult<N: private::LanesMultNum>() -> NonZeroPow2<N> {
                    $v()
                }
            }
            impl<T: MaskElement> private::LanesMult for Mask<T, $n> {
                #[inline(always)]
                fn lanes_mult<N: private::LanesMultNum>() -> NonZeroPow2<N> {
                    $v()
                }
            }
            impl private::LanesMult for LaneCount<$n> {
                #[inline(always)]
                fn lanes_mult<N: private::LanesMultNum>() -> NonZeroPow2<N> {
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


impl<N: NonZeroable> NonZeroPow2<N> {
    /// Gets the multiplier for the given SIMD type.
    #[inline(always)]
    pub fn of<T: LanesMult>() -> Self where N: private::LanesMultNum { <T as LanesMult>::lanes_mult() }
}
