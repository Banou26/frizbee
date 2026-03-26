use std::arch::wasm32::*;

/// 256-bit vector simulated with two 128-bit WASM SIMD registers
#[derive(Debug, Clone, Copy)]
pub struct WASM256Vector(pub(crate) (v128, v128));

/// Helper macro: equivalent to ARM vextq / SSE palignr.
/// Extracts 16 bytes starting at byte offset N from the concatenation [a, b].
macro_rules! wasm_ext {
    ($a:expr, $b:expr, 2) => {
        i8x16_shuffle::<2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17>($a, $b)
    };
    ($a:expr, $b:expr, 4) => {
        i8x16_shuffle::<4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19>($a, $b)
    };
    ($a:expr, $b:expr, 6) => {
        i8x16_shuffle::<6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21>($a, $b)
    };
    ($a:expr, $b:expr, 8) => {
        i8x16_shuffle::<8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23>($a, $b)
    };
    ($a:expr, $b:expr, 10) => {
        i8x16_shuffle::<10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25>(
            $a, $b,
        )
    };
    ($a:expr, $b:expr, 12) => {
        i8x16_shuffle::<12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27>(
            $a, $b,
        )
    };
    ($a:expr, $b:expr, 14) => {
        i8x16_shuffle::<14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29>(
            $a, $b,
        )
    };
}

impl super::Vector for WASM256Vector {
    #[inline]
    fn is_available() -> bool {
        true
    }

    #[inline(always)]
    unsafe fn zero() -> Self {
        let z = u8x16_splat(0);
        Self((z, z))
    }

    #[inline(always)]
    unsafe fn splat_u8(value: u8) -> Self {
        let v = u8x16_splat(value);
        Self((v, v))
    }

    #[inline(always)]
    unsafe fn splat_u16(value: u16) -> Self {
        let v = u16x8_splat(value);
        Self((v, v))
    }

    #[inline(always)]
    unsafe fn eq_u8(self, other: Self) -> Self {
        Self((u8x16_eq(self.0 .0, other.0 .0), u8x16_eq(self.0 .1, other.0 .1)))
    }

    #[inline(always)]
    unsafe fn gt_u8(self, other: Self) -> Self {
        Self((u8x16_gt(self.0 .0, other.0 .0), u8x16_gt(self.0 .1, other.0 .1)))
    }

    #[inline(always)]
    unsafe fn lt_u8(self, other: Self) -> Self {
        Self((u8x16_lt(self.0 .0, other.0 .0), u8x16_lt(self.0 .1, other.0 .1)))
    }

    #[inline(always)]
    unsafe fn max_u16(self, other: Self) -> Self {
        Self((
            u16x8_max(self.0 .0, other.0 .0),
            u16x8_max(self.0 .1, other.0 .1),
        ))
    }

    #[inline(always)]
    unsafe fn smax_u16(self) -> u16 {
        // Max across both halves, then horizontal reduce
        let combined = u16x8_max(self.0 .0, self.0 .1);
        let swapped = i8x16_shuffle::<8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7>(
            combined, combined,
        );
        let max1 = u16x8_max(combined, swapped);
        let swapped =
            i8x16_shuffle::<4, 5, 6, 7, 0, 1, 2, 3, 8, 9, 10, 11, 12, 13, 14, 15>(max1, max1);
        let max2 = u16x8_max(max1, swapped);
        let swapped =
            i8x16_shuffle::<2, 3, 0, 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15>(max2, max2);
        let max3 = u16x8_max(max2, swapped);
        u16x8_extract_lane::<0>(max3)
    }

    #[inline(always)]
    unsafe fn add_u16(self, other: Self) -> Self {
        Self((
            u16x8_add(self.0 .0, other.0 .0),
            u16x8_add(self.0 .1, other.0 .1),
        ))
    }

    #[inline(always)]
    unsafe fn subs_u16(self, other: Self) -> Self {
        Self((
            u16x8_sub_sat(self.0 .0, other.0 .0),
            u16x8_sub_sat(self.0 .1, other.0 .1),
        ))
    }

    #[inline(always)]
    unsafe fn and(self, other: Self) -> Self {
        Self((
            v128_and(self.0 .0, other.0 .0),
            v128_and(self.0 .1, other.0 .1),
        ))
    }

    #[inline(always)]
    unsafe fn or(self, other: Self) -> Self {
        Self((
            v128_or(self.0 .0, other.0 .0),
            v128_or(self.0 .1, other.0 .1),
        ))
    }

    #[inline(always)]
    unsafe fn not(self) -> Self {
        Self((v128_not(self.0 .0), v128_not(self.0 .1)))
    }

    #[inline(always)]
    unsafe fn shift_right_padded_u16<const L: i32>(self, other: Self) -> Self {
        const { assert!(L >= 0 && L <= 15) };
        match L {
            0 => self,
            1 => Self((
                wasm_ext!(other.0 .1, self.0 .0, 14),
                wasm_ext!(self.0 .0, self.0 .1, 14),
            )),
            2 => Self((
                wasm_ext!(other.0 .1, self.0 .0, 12),
                wasm_ext!(self.0 .0, self.0 .1, 12),
            )),
            3 => Self((
                wasm_ext!(other.0 .1, self.0 .0, 10),
                wasm_ext!(self.0 .0, self.0 .1, 10),
            )),
            4 => Self((
                wasm_ext!(other.0 .1, self.0 .0, 8),
                wasm_ext!(self.0 .0, self.0 .1, 8),
            )),
            5 => Self((
                wasm_ext!(other.0 .1, self.0 .0, 6),
                wasm_ext!(self.0 .0, self.0 .1, 6),
            )),
            6 => Self((
                wasm_ext!(other.0 .1, self.0 .0, 4),
                wasm_ext!(self.0 .0, self.0 .1, 4),
            )),
            7 => Self((
                wasm_ext!(other.0 .1, self.0 .0, 2),
                wasm_ext!(self.0 .0, self.0 .1, 2),
            )),
            8 => Self((other.0 .1, self.0 .0)),
            9 => Self((
                wasm_ext!(other.0 .0, other.0 .1, 14),
                wasm_ext!(other.0 .1, self.0 .0, 14),
            )),
            10 => Self((
                wasm_ext!(other.0 .0, other.0 .1, 12),
                wasm_ext!(other.0 .1, self.0 .0, 12),
            )),
            11 => Self((
                wasm_ext!(other.0 .0, other.0 .1, 10),
                wasm_ext!(other.0 .1, self.0 .0, 10),
            )),
            12 => Self((
                wasm_ext!(other.0 .0, other.0 .1, 8),
                wasm_ext!(other.0 .1, self.0 .0, 8),
            )),
            13 => Self((
                wasm_ext!(other.0 .0, other.0 .1, 6),
                wasm_ext!(other.0 .1, self.0 .0, 6),
            )),
            14 => Self((
                wasm_ext!(other.0 .0, other.0 .1, 4),
                wasm_ext!(other.0 .1, self.0 .0, 4),
            )),
            15 => Self((
                wasm_ext!(other.0 .0, other.0 .1, 2),
                wasm_ext!(other.0 .1, self.0 .0, 2),
            )),
            _ => unreachable!(),
        }
    }

    #[cfg(test)]
    fn from_array(arr: [u8; 16]) -> Self {
        let v = v128_load(arr.as_ptr() as *const v128);
        Self((v, v))
    }
    #[cfg(test)]
    fn to_array(self) -> [u8; 16] {
        let mut arr = [0u8; 16];
        v128_store(arr.as_mut_ptr() as *mut v128, self.0 .0);
        arr
    }
    #[cfg(test)]
    fn from_array_u16(arr: [u16; 8]) -> Self {
        let v = v128_load(arr.as_ptr() as *const v128);
        Self((v, v))
    }
    #[cfg(test)]
    fn to_array_u16(self) -> [u16; 8] {
        let mut arr = [0u16; 8];
        v128_store(arr.as_mut_ptr() as *mut v128, self.0 .0);
        arr
    }
}

impl super::Vector256 for WASM256Vector {
    #[cfg(test)]
    fn from_array_256_u16(arr: [u16; 16]) -> Self {
        Self((
            v128_load(arr.as_ptr() as *const v128),
            v128_load(arr.as_ptr().add(8) as *const v128),
        ))
    }
    #[cfg(test)]
    fn to_array_256_u16(self) -> [u16; 16] {
        let mut arr = [0u16; 16];
        v128_store(arr.as_mut_ptr() as *mut v128, self.0 .0);
        v128_store(arr.as_mut_ptr().add(8) as *mut v128, self.0 .1);
        arr
    }

    #[inline(always)]
    unsafe fn load_unaligned(data: [u8; 32]) -> Self {
        Self(unsafe {
            (
                v128_load(data.as_ptr() as *const v128),
                v128_load(data.as_ptr().add(16) as *const v128),
            )
        })
    }

    #[inline(always)]
    unsafe fn idx_u16(self, search: u16) -> usize {
        let search_vec = u16x8_splat(search);
        let cmp_lo = u16x8_eq(self.0 .0, search_vec);
        let cmp_hi = u16x8_eq(self.0 .1, search_vec);

        // i8x16_bitmask extracts MSB of each byte into a u32 bitmask.
        // Each matching u16 lane produces two consecutive 0xFF bytes -> 2 set bits.
        let mask_lo = i8x16_bitmask(cmp_lo) as u32;
        if mask_lo != 0 {
            return mask_lo.trailing_zeros() as usize / 2;
        }
        let mask_hi = i8x16_bitmask(cmp_hi) as u32;
        if mask_hi != 0 {
            return mask_hi.trailing_zeros() as usize / 2 + 8;
        }
        16 // Not found
    }
}
