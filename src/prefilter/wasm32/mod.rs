use std::arch::wasm32::*;

use crate::prefilter::{case_needle, scalar};

/// Loads a chunk of 16 bytes from the haystack, with overlap when remaining bytes < 16.
///
/// # Safety
/// Caller must ensure that haystack length >= 8
#[inline(always)]
unsafe fn overlapping_load(haystack: &[u8], start: usize, len: usize) -> v128 {
    unsafe {
        match len {
            0..=7 => unreachable!(),
            8..=15 => {
                // Copy to a stack buffer for small haystacks
                let mut buf = [0u8; 16];
                core::ptr::copy_nonoverlapping(haystack.as_ptr(), buf.as_mut_ptr(), len);
                v128_load(buf.as_ptr() as *const v128)
            }
            16 => v128_load(haystack.as_ptr() as *const v128),
            // Re-read from the last 16 bytes to avoid reading past the end
            _ => v128_load(haystack.as_ptr().add(start.min(len - 16)) as *const v128),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrefilterWASM {
    needle: Vec<(u8, u8)>,
}

impl PrefilterWASM {
    #[inline]
    pub fn new(needle: &[u8]) -> Self {
        Self {
            needle: case_needle(needle),
        }
    }

    #[inline]
    #[target_feature(enable = "simd128")]
    pub unsafe fn match_haystack(&self, haystack: &[u8]) -> (bool, usize) {
        let len = haystack.len();

        match len {
            0 => return (true, 0),
            1..=7 => {
                return (scalar::match_haystack(&self.needle, haystack), 0);
            }
            _ => {}
        };

        let mut can_skip_chunks = true;
        let mut skipped_chunks = 0;

        let mut needle_iter = self
            .needle
            .iter()
            .map(|&(c1, c2)| (u8x16_splat(c1), u8x16_splat(c2)));
        let mut needle_char = needle_iter.next().unwrap();

        for start in (0..len).step_by(16) {
            let haystack_chunk = unsafe { overlapping_load(haystack, start, len) };

            loop {
                let cmp = v128_or(
                    u8x16_eq(needle_char.1, haystack_chunk),
                    u8x16_eq(needle_char.0, haystack_chunk),
                );
                if !v128_any_true(cmp) {
                    break;
                }

                if let Some(next_needle_char) = needle_iter.next() {
                    if can_skip_chunks {
                        skipped_chunks = start / 16;
                    }
                    can_skip_chunks = false;
                    needle_char = next_needle_char;
                } else {
                    return (true, skipped_chunks);
                }
            }
        }

        (false, skipped_chunks)
    }

    #[inline]
    #[target_feature(enable = "simd128")]
    pub unsafe fn match_haystack_typos(&self, haystack: &[u8], max_typos: u16) -> (bool, usize) {
        let len = haystack.len();

        match len {
            0 => return (true, 0),
            1..=7 => {
                return (
                    scalar::match_haystack_typos(&self.needle, haystack, max_typos),
                    0,
                );
            }
            _ => {}
        };

        if max_typos >= 3 {
            return (true, 0);
        }

        let mut needle_iter = self
            .needle
            .iter()
            .map(|&(c1, c2)| (u8x16_splat(c1), u8x16_splat(c2)));
        let mut needle_char = needle_iter.next().unwrap();

        let mut typos = 0;
        loop {
            for start in (0..len).step_by(16) {
                let haystack_chunk = unsafe { overlapping_load(haystack, start, len) };

                loop {
                    let cmp = v128_or(
                        u8x16_eq(needle_char.1, haystack_chunk),
                        u8x16_eq(needle_char.0, haystack_chunk),
                    );
                    if !v128_any_true(cmp) {
                        break;
                    }

                    if let Some(next_needle_char) = needle_iter.next() {
                        needle_char = next_needle_char;
                    } else {
                        return (true, 0);
                    }
                }
            }

            typos += 1;
            if typos > max_typos as usize {
                return (false, 0);
            }

            if let Some(next_needle_char) = needle_iter.next() {
                needle_char = next_needle_char;
            } else {
                return (true, 0);
            }
        }
    }
}
