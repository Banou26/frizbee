use std::arch::wasm32::*;

use super::WASM256Vector;

#[derive(Debug, Clone, Copy)]
pub struct WASMVector(v128);

impl super::Vector for WASMVector {
    #[inline]
    fn is_available() -> bool {
        true
    }

    #[inline(always)]
    unsafe fn zero() -> Self {
        Self(u8x16_splat(0))
    }

    #[inline(always)]
    unsafe fn splat_u8(value: u8) -> Self {
        Self(u8x16_splat(value))
    }

    #[inline(always)]
    unsafe fn splat_u16(value: u16) -> Self {
        Self(u16x8_splat(value))
    }

    #[inline(always)]
    unsafe fn eq_u8(self, other: Self) -> Self {
        Self(u8x16_eq(self.0, other.0))
    }

    #[inline(always)]
    unsafe fn gt_u8(self, other: Self) -> Self {
        Self(u8x16_gt(self.0, other.0))
    }

    #[inline(always)]
    unsafe fn lt_u8(self, other: Self) -> Self {
        Self(u8x16_lt(self.0, other.0))
    }

    #[inline(always)]
    unsafe fn max_u16(self, other: Self) -> Self {
        Self(u16x8_max(self.0, other.0))
    }

    #[inline(always)]
    unsafe fn smax_u16(self) -> u16 {
        // Horizontal max via reduction: swap halves and take max at each step
        // 8 lanes -> 4
        let swapped = i8x16_shuffle::<8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7>(
            self.0, self.0,
        );
        let max1 = u16x8_max(self.0, swapped);
        // 4 -> 2
        let swapped =
            i8x16_shuffle::<4, 5, 6, 7, 0, 1, 2, 3, 8, 9, 10, 11, 12, 13, 14, 15>(max1, max1);
        let max2 = u16x8_max(max1, swapped);
        // 2 -> 1
        let swapped =
            i8x16_shuffle::<2, 3, 0, 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15>(max2, max2);
        let max3 = u16x8_max(max2, swapped);
        u16x8_extract_lane::<0>(max3)
    }

    #[inline(always)]
    unsafe fn add_u16(self, other: Self) -> Self {
        Self(u16x8_add(self.0, other.0))
    }

    #[inline(always)]
    unsafe fn subs_u16(self, other: Self) -> Self {
        Self(u16x8_sub_sat(self.0, other.0))
    }

    #[inline(always)]
    unsafe fn and(self, other: Self) -> Self {
        Self(v128_and(self.0, other.0))
    }

    #[inline(always)]
    unsafe fn or(self, other: Self) -> Self {
        Self(v128_or(self.0, other.0))
    }

    #[inline(always)]
    unsafe fn not(self) -> Self {
        Self(v128_not(self.0))
    }

    #[inline(always)]
    unsafe fn shift_right_padded_u16<const N: i32>(self, other: Self) -> Self {
        match N {
            0 => self,
            1 => Self(i8x16_shuffle::<
                14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            >(other.0, self.0)),
            2 => Self(i8x16_shuffle::<
                12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
            >(other.0, self.0)),
            3 => Self(i8x16_shuffle::<
                10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            >(other.0, self.0)),
            4 => Self(i8x16_shuffle::<
                8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            >(other.0, self.0)),
            5 => Self(i8x16_shuffle::<
                6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
            >(other.0, self.0)),
            6 => Self(i8x16_shuffle::<
                4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
            >(other.0, self.0)),
            7 => Self(i8x16_shuffle::<
                2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
            >(other.0, self.0)),
            8 => Self(other.0),
            _ => unreachable!(),
        }
    }

    #[cfg(test)]
    fn from_array(arr: [u8; 16]) -> Self {
        Self(v128_load(arr.as_ptr() as *const v128))
    }
    #[cfg(test)]
    fn to_array(self) -> [u8; 16] {
        let mut arr = [0u8; 16];
        v128_store(arr.as_mut_ptr() as *mut v128, self.0);
        arr
    }
    #[cfg(test)]
    fn from_array_u16(arr: [u16; 8]) -> Self {
        Self(v128_load(arr.as_ptr() as *const v128))
    }
    #[cfg(test)]
    fn to_array_u16(self) -> [u16; 8] {
        let mut arr = [0u16; 8];
        v128_store(arr.as_mut_ptr() as *mut v128, self.0);
        arr
    }
}

impl super::Vector128 for WASMVector {
    #[inline(always)]
    unsafe fn load_partial(data: *const u8, start: usize, len: usize) -> Self {
        if len == 0 {
            return Self(u8x16_splat(0));
        }
        if start + 16 <= len {
            return Self(unsafe { v128_load(data.add(start) as *const v128) });
        }
        if start >= len {
            return Self(u8x16_splat(0));
        }
        // Partial load: copy available bytes to a stack buffer
        let mut buf = [0u8; 16];
        let available = len - start;
        unsafe { core::ptr::copy_nonoverlapping(data.add(start), buf.as_mut_ptr(), available) };
        Self(unsafe { v128_load(buf.as_ptr() as *const v128) })
    }

    #[inline(always)]
    unsafe fn shift_right_padded_u8<const L: i32>(self, other: Self) -> Self {
        match L {
            0 => self,
            1 => Self(i8x16_shuffle::<
                15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            >(other.0, self.0)),
            2 => Self(i8x16_shuffle::<
                14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            >(other.0, self.0)),
            3 => Self(i8x16_shuffle::<
                13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            >(other.0, self.0)),
            4 => Self(i8x16_shuffle::<
                12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
            >(other.0, self.0)),
            5 => Self(i8x16_shuffle::<
                11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
            >(other.0, self.0)),
            6 => Self(i8x16_shuffle::<
                10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            >(other.0, self.0)),
            7 => Self(i8x16_shuffle::<
                9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            >(other.0, self.0)),
            8 => Self(i8x16_shuffle::<
                8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            >(other.0, self.0)),
            9 => Self(i8x16_shuffle::<
                7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            >(other.0, self.0)),
            10 => Self(i8x16_shuffle::<
                6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
            >(other.0, self.0)),
            11 => Self(i8x16_shuffle::<
                5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            >(other.0, self.0)),
            12 => Self(i8x16_shuffle::<
                4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
            >(other.0, self.0)),
            13 => Self(i8x16_shuffle::<
                3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            >(other.0, self.0)),
            14 => Self(i8x16_shuffle::<
                2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
            >(other.0, self.0)),
            15 => Self(i8x16_shuffle::<
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            >(other.0, self.0)),
            _ => unreachable!(),
        }
    }
}

impl super::Vector128Expansion<WASM256Vector> for WASMVector {
    #[inline(always)]
    unsafe fn cast_i8_to_i16(self) -> WASM256Vector {
        WASM256Vector((
            i16x8_extend_low_i8x16(self.0),
            i16x8_extend_high_i8x16(self.0),
        ))
    }
}
