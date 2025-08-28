use crate::{Align64, CodeConverter, CodeConverterInPlace};

pub(crate) const A: u8 = 0b0000_0001;
pub(crate) const C: u8 = 0b0000_0010;
pub(crate) const G: u8 = 0b0000_0100;
pub(crate) const T: u8 = 0b0000_1000;
pub(crate) const U: u8 = T;
pub(crate) const M: u8 = A | C;
pub(crate) const R: u8 = A | G;
pub(crate) const S: u8 = C | G;
pub(crate) const V: u8 = A | C | G;
pub(crate) const W: u8 = A | T;
pub(crate) const Y: u8 = C | T;
pub(crate) const H: u8 = A | C | T;
pub(crate) const K: u8 = G | T;
pub(crate) const D: u8 = A | G | T;
pub(crate) const B: u8 = C | G | T;
pub(crate) const N: u8 = A | C | G | T;

/// LUT for converting an ASCII character to a mask.
pub(crate) static BASE_CHAR_TO_MASK: Align64<[u8; 256]> = Align64(
    const {
        let mut table = [N; 256];
        table[b'A' as usize] = A;
        table[b'C' as usize] = C;
        table[b'G' as usize] = G;
        table[b'T' as usize] = T;
        table[b'U' as usize] = U;
        table[b'M' as usize] = M;
        table[b'R' as usize] = R;
        table[b'S' as usize] = S;
        table[b'V' as usize] = V;
        table[b'W' as usize] = W;
        table[b'Y' as usize] = Y;
        table[b'H' as usize] = H;
        table[b'K' as usize] = K;
        table[b'D' as usize] = D;
        table[b'B' as usize] = B;
        table[b'N' as usize] = N;
        let mut i = b'a';
        while i <= b'z' {
            table[i as usize] = table[i.to_ascii_uppercase() as usize];
            i += 1;
        }
        table
    },
);

/// LUT for converting a mask back to an ASCII character.
pub(crate) static BASE_MASK_TO_CHAR: Align64<[u8; 256]> = Align64(
    const {
        let mut table = [b'N'; 256];
        table[A as usize] = b'A';
        table[C as usize] = b'C';
        table[G as usize] = b'G';
        table[T as usize] = b'T';
        table[M as usize] = b'M';
        table[R as usize] = b'R';
        table[S as usize] = b'S';
        table[V as usize] = b'V';
        table[W as usize] = b'W';
        table[Y as usize] = b'Y';
        table[H as usize] = b'H';
        table[K as usize] = b'K';
        table[D as usize] = b'D';
        table[B as usize] = b'B';
        table[N as usize] = b'N';
        table
    },
);

#[repr(align(64))]
pub struct LUTPacker {
    lut: [u8; 256],
}

impl Default for LUTPacker {
    fn default() -> Self {
        Self {
            lut: BASE_CHAR_TO_MASK.0,
        }
    }
}

impl LUTPacker {
    pub const fn new(lut: [u8; 256]) -> Self {
        Self { lut }
    }

    pub const fn new_alphabet(lut: [u8; 32]) -> Self {
        let mut full = [N; 256];
        let mut idx = 0;
        while idx < 32 {
            full[(b'A' + idx as u8) as usize] = lut[idx];
            full[(b'a' + idx as u8) as usize] = lut[idx];
            idx += 1;
        }
        Self { lut: full }
    }
}

impl CodeConverter for LUTPacker {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        assert!(out.len() >= (code.len() + 1) / 2);

        for i in 0..(code.len() / 2) {
            let low = self.lut[code[i * 2] as usize];
            let high = self.lut[code[i * 2 + 1] as usize];
            let value = low | high << 4;
            out[i] = value;
        }
        if code.len() % 2 == 1 {
            out[code.len() / 2] = self.lut[code[code.len() - 1] as usize];
        }
    }
}

pub struct SSE41Packer {
    scalar: LUTPacker,
    lut: [u8; 32],
}

impl Default for SSE41Packer {
    fn default() -> Self {
        Self {
            scalar: LUTPacker::default(),
            lut: core::array::from_fn(|i| BASE_CHAR_TO_MASK.0[(b'A' + i as u8) as usize]),
        }
    }
}

impl SSE41Packer {
    #[target_feature(enable = "sse4.1")]
    fn pack_impl(&self, code: &[u8], out: &mut [u8]) {
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;

        assert!(out.len() >= code.len() / 2);

        let mut chunks = code.chunks_exact(16);
        let mut out_chunks = out.chunks_exact_mut(8);

        unsafe {
            let lut0 = _mm_load_si128(self.lut.as_ptr().cast());
            let lut1 = _mm_load_si128(self.lut.as_ptr().add(16).cast());
            let offset0 = _mm_set1_epi8(b'A' as _);
            let offset1 = _mm_set1_epi8((b'A' + 16) as _);
            let to_upper = _mm_set1_epi8(!0x20);
            for (chunk, out_chunk) in (&mut chunks).zip(&mut out_chunks) {
                let mut chunk = _mm_loadu_si128(chunk.as_ptr().cast());

                let mut invalid_mask;

                {
                    chunk = _mm_and_si128(chunk, to_upper);

                    invalid_mask = _mm_cmplt_epi8(chunk, _mm_set1_epi8((b'A') as _));
                    invalid_mask = _mm_and_si128(invalid_mask, offset0); // offset0 is just a constant that happens to work for all possible bytes so don't bother using another constant

                    let indices0 = _mm_sub_epi8(chunk, offset0);
                    let indices1 = _mm_sub_epi8(chunk, offset1);

                    let result0 = _mm_shuffle_epi8(lut0, indices0);
                    let result1 = _mm_shuffle_epi8(lut1, indices1);

                    let mask = _mm_cmpgt_epi8(result1, _mm_setzero_si128());

                    chunk = _mm_blendv_epi8(result0, result1, mask);
                }

                let shifted = _mm_srli_epi16(chunk, 4);
                chunk = _mm_and_si128(chunk, _mm_set1_epi16(0b0000_1111));
                let mut mixed = _mm_or_si128(shifted, chunk);
                mixed = _mm_or_si128(mixed, invalid_mask);

                let result = _mm_cvtsi128_si64(_mm_packus_epi16(mixed, mixed));

                out_chunk.as_mut_ptr().cast::<i64>().write_unaligned(result);
            }
        }

        self.scalar
            .convert(chunks.remainder(), out_chunks.into_remainder());
    }
}

cpufeatures::new!(x86_sse4_1, "sse4.1");
cpufeatures::new!(x86_ssse3, "ssse3");

impl CodeConverter for SSE41Packer {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        if x86_sse4_1::get() {
            unsafe { self.pack_impl(code, out) }
        } else {
            self.scalar.convert(code, out);
        }
    }
}

#[repr(align(64))]
pub struct LUTInPlacePacker {
    lut: [u8; 256],
}

impl LUTInPlacePacker {
    pub const fn new(lut: [u8; 256]) -> Self {
        Self { lut }
    }

    pub const fn new_alphabet(lut: [u8; 32]) -> Self {
        let mut full = [N; 256];
        let mut idx = 0;
        while idx < 32 {
            full[(b'A' + idx as u8) as usize] = lut[idx];
            full[(b'a' + idx as u8) as usize] = lut[idx];
            idx += 1;
        }
        Self { lut: full }
    }
}

impl Default for LUTInPlacePacker {
    fn default() -> Self {
        Self {
            lut: BASE_CHAR_TO_MASK.0,
        }
    }
}

impl CodeConverterInPlace for LUTInPlacePacker {
    fn convert_in_place<'a>(&self, buf: &'a mut [u8]) -> &'a mut [u8] {
        let seq_len = buf.len();
        let packed_len = (seq_len + 1) / 2;

        let mut input_pos = seq_len;
        let mut output_pos = seq_len;
        if seq_len % 2 == 1 {
            buf[buf.len() - 1] = self.lut[buf[buf.len() - 1] as usize];
            input_pos -= 1;
            output_pos -= 1;
        }

        unsafe {
            while input_pos as isize >= 2 {
                input_pos -= 2;
                output_pos -= 1;
                let mut lsb = *buf.get_unchecked(input_pos);
                let mut msb = *buf.get_unchecked(input_pos + 1);

                lsb = self.lut[lsb as usize];
                msb = self.lut[msb as usize];

                *buf.get_unchecked_mut(output_pos) = lsb | msb << 4;
            }
        }

        &mut buf[seq_len - packed_len..]
    }
}

#[repr(align(16))]
pub struct SSE41InPlacePacker {
    lut: [u8; 32],
}

impl SSE41InPlacePacker {
    pub const fn new(lut: [u8; 32]) -> Self {
        Self { lut }
    }
}

impl Default for SSE41InPlacePacker {
    fn default() -> Self {
        Self {
            lut: core::array::from_fn(|i| BASE_CHAR_TO_MASK.0[(b'A' + i as u8) as usize]),
        }
    }
}

impl SSE41InPlacePacker {
    #[target_feature(enable = "sse4.1")]
    fn convert_impl<'a>(&self, buf: &'a mut [u8]) -> &'a mut [u8] {
        if buf.is_empty() {
            return buf;
        }

        let seq_len = buf.len();
        let packed_len = (seq_len + 1) / 2;

        let mut input_pos = seq_len;
        let mut output_pos = seq_len;
        if seq_len % 2 == 1 {
            let mut last_word = buf[buf.len() - 1] & !0x20;
            last_word = last_word.wrapping_sub(b'A' as u8);
            if last_word < 32 {
                buf[buf.len() - 1] = self.lut[(last_word) as usize];
            } else {
                buf[buf.len() - 1] = N;
            }
            input_pos -= 1;
            output_pos -= 1;
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe {
            #[cfg(target_arch = "x86_64")]
            use core::arch::x86_64::*;

            #[cfg(target_arch = "x86")]
            use core::arch::x86::*;

            let lut0 = _mm_load_si128(self.lut.as_ptr().cast());
            let lut1 = _mm_load_si128(self.lut.as_ptr().add(16).cast());
            let offset0 = _mm_set1_epi8(b'A' as _);
            let offset1 = _mm_set1_epi8((b'A' + 16) as _);
            let to_upper = _mm_set1_epi8(!0x20);
            while input_pos >= 16 {
                input_pos -= 16;
                output_pos -= 8;
                let mut chunk = _mm_loadu_si128(buf.as_ptr().add(input_pos).cast());

                let mut invalid_mask;

                {
                    chunk = _mm_and_si128(chunk, to_upper);

                    invalid_mask = _mm_cmplt_epi8(chunk, _mm_set1_epi8((b'A') as _));
                    invalid_mask = _mm_and_si128(invalid_mask, offset0); // offset0 is just a constant that happens to work for all possible bytes so don't bother using another constant

                    let indices0 = _mm_sub_epi8(chunk, offset0);
                    let indices1 = _mm_sub_epi8(chunk, offset1);

                    let result0 = _mm_shuffle_epi8(lut0, indices0);
                    let result1 = _mm_shuffle_epi8(lut1, indices1);

                    let mask = _mm_cmpgt_epi8(result1, _mm_setzero_si128());

                    chunk = _mm_blendv_epi8(result0, result1, mask);
                }

                let shifted = _mm_srli_epi16(chunk, 4);
                chunk = _mm_and_si128(chunk, _mm_set1_epi16(0b0000_1111));
                let mut mixed = _mm_or_si128(shifted, chunk);
                mixed = _mm_or_si128(mixed, invalid_mask);

                let result = _mm_cvtsi128_si64(_mm_packus_epi16(mixed, mixed));

                buf.as_mut_ptr()
                    .add(output_pos)
                    .cast::<i64>()
                    .write_unaligned(result);
            }
        }

        unsafe {
            while input_pos as isize >= 2 {
                input_pos -= 2;
                output_pos -= 1;
                let mut lsb = *buf.get_unchecked(input_pos);
                let mut msb = *buf.get_unchecked(input_pos + 1);

                lsb &= !0x20;
                msb &= !0x20;
                lsb = lsb.wrapping_sub(b'A' as u8);
                msb = msb.wrapping_sub(b'A' as u8);
                if lsb < 32 {
                    lsb = self.lut[lsb as usize]
                } else {
                    lsb = N;
                }

                if msb < 32 {
                    msb = self.lut[msb as usize];
                } else {
                    msb = N;
                }

                *buf.get_unchecked_mut(output_pos) = lsb | msb << 4;
            }
        }

        &mut buf[seq_len - packed_len..]
    }
}

impl CodeConverterInPlace for SSE41InPlacePacker {
    fn convert_in_place<'a>(&self, buf: &'a mut [u8]) -> &'a mut [u8] {
        if x86_sse4_1::get() {
            unsafe { self.convert_impl(buf) }
        } else {
            LUTInPlacePacker::default().convert_in_place(buf)
        }
    }
}

#[repr(align(64))]
pub struct LUTUnpacker {
    lut: [u8; 16],
}

impl Default for LUTUnpacker {
    fn default() -> Self {
        Self {
            lut: core::array::from_fn(|i| BASE_MASK_TO_CHAR.0[i as usize]),
        }
    }
}

impl CodeConverter for LUTUnpacker {
    fn convert(&self, mut input: &[u8], out: &mut [u8]) {
        let Some(last) = input.last() else {
            return;
        };

        if last & 0xf0 == 0 {
            let mut result = (*last) & 0b0000_1111;
            result = self.lut[result as usize];
            out[(input.len() - 1) * 2] = result;
            input = &input[..input.len() - 1];
        }

        assert!(out.len() >= input.len() * 2);

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        #[cfg(target_feature = "ssse3")]
        {
            #[cfg(target_arch = "x86")]
            use core::arch::x86::*;
            #[cfg(target_arch = "x86_64")]
            use core::arch::x86_64::*;

            unsafe {
                let align_offset = input.as_ptr().align_offset(8).min(input.len());
                for i in 0..align_offset {
                    let packed = *input.get_unchecked(i);
                    let mut low_nibble = packed & 0b0000_1111;
                    let mut high_nibble = packed >> 4;

                    if OUT_ASCII {
                        low_nibble = self.lut[low_nibble as usize];
                        high_nibble = self.lut[high_nibble as usize];
                    }

                    *out.get_unchecked_mut(i * 2) = low_nibble;
                    *out.get_unchecked_mut(i * 2 + 1) = high_nibble;
                }
                input = &input[align_offset..];
                out = &mut out[align_offset * 2..];

                let lut = _mm_load_si128(self.lut.as_ptr().cast());

                while input.len() >= 8 {
                    let load = input.as_ptr().cast::<u64>().read();
                    let highs = load >> 4;
                    let lows = _mm_cvtsi64_si128(load as _);
                    let highs = _mm_cvtsi64_si128(highs as _);
                    let mut result = _mm_unpacklo_epi8(lows, highs);
                    result = _mm_and_si128(result, _mm_set1_epi8(0b0000_1111));

                    result = _mm_shuffle_epi8(lut, result);

                    _mm_storeu_si128(out.as_mut_ptr().cast(), result);

                    input = &input[8..];
                    out = &mut out[8 * 2..];
                }
            }
        }

        unsafe {
            for i in 0..input.len() {
                let packed = *input.get_unchecked(i);
                let mut low_nibble = packed & 0b0000_1111;
                let mut high_nibble = packed >> 4;

                low_nibble = self.lut[low_nibble as usize];
                high_nibble = self.lut[high_nibble as usize];

                *out.get_unchecked_mut(i * 2) = low_nibble;
                *out.get_unchecked_mut(i * 2 + 1) = high_nibble;
            }
        }
    }
}

#[repr(align(64))]
pub struct SSSE3Unpacker {
    lut: [u8; 16],
}

impl Default for SSSE3Unpacker {
    fn default() -> Self {
        Self {
            lut: core::array::from_fn(|i| BASE_MASK_TO_CHAR.0[i as usize]),
        }
    }
}

impl SSSE3Unpacker {
    #[target_feature(enable = "ssse3")]
    fn convert_impl(&self, mut input: &[u8], mut out: &mut [u8]) {
        let Some(last) = input.last() else {
            return;
        };

        if last & 0xf0 == 0 {
            let mut result = (*last) & 0b0000_1111;
            result = self.lut[result as usize];
            out[(input.len() - 1) * 2] = result;
            input = &input[..input.len() - 1];
        }

        assert!(out.len() >= input.len() * 2);

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            #[cfg(target_arch = "x86")]
            use core::arch::x86::*;
            #[cfg(target_arch = "x86_64")]
            use core::arch::x86_64::*;

            unsafe {
                let align_offset = input.as_ptr().align_offset(8).min(input.len());
                for i in 0..align_offset {
                    let packed = *input.get_unchecked(i);
                    let mut low_nibble = packed & 0b0000_1111;
                    let mut high_nibble = packed >> 4;

                    low_nibble = self.lut[low_nibble as usize];
                    high_nibble = self.lut[high_nibble as usize];

                    *out.get_unchecked_mut(i * 2) = low_nibble;
                    *out.get_unchecked_mut(i * 2 + 1) = high_nibble;
                }
                input = &input[align_offset..];
                out = &mut out[align_offset * 2..];

                let lut = _mm_load_si128(self.lut.as_ptr().cast());

                while input.len() >= 8 {
                    let load = input.as_ptr().cast::<u64>().read();
                    let highs = load >> 4;
                    let lows = _mm_cvtsi64_si128(load as _);
                    let highs = _mm_cvtsi64_si128(highs as _);
                    let mut result = _mm_unpacklo_epi8(lows, highs);
                    result = _mm_and_si128(result, _mm_set1_epi8(0b0000_1111));

                    result = _mm_shuffle_epi8(lut, result);

                    _mm_storeu_si128(out.as_mut_ptr().cast(), result);

                    input = &input[8..];
                    out = &mut out[8 * 2..];
                }
            }
        }

        unsafe {
            for i in 0..input.len() {
                let packed = *input.get_unchecked(i);
                let mut low_nibble = packed & 0b0000_1111;
                let mut high_nibble = packed >> 4;

                low_nibble = self.lut[low_nibble as usize];
                high_nibble = self.lut[high_nibble as usize];

                *out.get_unchecked_mut(i * 2) = low_nibble;
                *out.get_unchecked_mut(i * 2 + 1) = high_nibble;
            }
        }
    }
}

impl CodeConverter for SSSE3Unpacker {
    fn convert(&self, input: &[u8], out: &mut [u8]) {
        if x86_ssse3::get() {
            unsafe { self.convert_impl(input, out) }
        } else {
            LUTUnpacker::default().convert(input, out)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_sequence_even() {
        // test case from ReferenceSequence.hpp
        const SEQ: [u8; 12] = *b"NNACGTATAGAC";

        let mut buf = SEQ;

        let packed = SSE41InPlacePacker::default().convert_in_place(&mut buf);
        assert_eq!(packed, [0xFF, 0x21, 0x84, 0x81, 0x41, 0x21]);

        let mut unpacked = [0; SEQ.len()];
        SSSE3Unpacker::default().convert(packed, &mut unpacked);
        assert_eq!(unpacked, SEQ);
    }

    #[test]
    fn test_pack_sequence_even_long() {
        // test case from ReferenceSequence.hpp
        const SEQ: [u8; 36] = *b"NNACGTATAGACNNACGTATAGACNNACGTATAGAC";

        let mut buf = SEQ;

        let mut out_of_place_out = [0; 18];
        SSE41Packer::default().convert(&SEQ, &mut out_of_place_out);
        assert_eq!(
            out_of_place_out,
            [
                0xFF, 0x21, 0x84, 0x81, 0x41, 0x21, 0xFF, 0x21, 0x84, 0x81, 0x41, 0x21, 0xFF, 0x21,
                0x84, 0x81, 0x41, 0x21,
            ]
        );

        let packed = SSE41InPlacePacker::default().convert_in_place(&mut buf);
        assert_eq!(
            packed,
            [
                0xFF, 0x21, 0x84, 0x81, 0x41, 0x21, 0xFF, 0x21, 0x84, 0x81, 0x41, 0x21, 0xFF, 0x21,
                0x84, 0x81, 0x41, 0x21,
            ],
            "packed: {:02x?}",
            packed,
        );

        let mut unpacked = [0; SEQ.len()];
        SSSE3Unpacker::default().convert(packed, &mut unpacked);
        assert_eq!(unpacked, SEQ);
    }

    #[test]
    fn test_pack_sequence_any_byte() {
        for val in 0..=u8::MAX {
            let mut buf = [val; 64];
            let expect = if val.is_ascii_alphabetic() {
                BASE_CHAR_TO_MASK[val as usize]
            } else {
                N
            };

            let packed = SSE41InPlacePacker::default().convert_in_place(&mut buf);

            assert_eq!(
                &[expect | expect << 4; 32],
                packed,
                "unexpected output for {:02x}",
                val
            );
        }
    }

    #[test]
    fn test_pack_sequence_odd() {
        const SEQ: [u8; 13] = *b"NNACGTATAGACG";
        let mut buf = SEQ;
        let packed = SSE41InPlacePacker::default().convert_in_place(&mut buf);
        assert_eq!(packed, [0xFF, 0x21, 0x84, 0x81, 0x41, 0x21, 0x04]);

        let mut unpacked = [0; SEQ.len()];
        SSSE3Unpacker::default().convert(packed, &mut unpacked);
        assert_eq!(unpacked, SEQ);
    }

    #[test]
    fn test_pack_sequence_odd_long() {
        const SEQ: [u8; 39] = *b"NNACGTATAGACGNNACGTATAGACGNNACGTATAGACG";
        let mut buf = SEQ;

        let mut out_of_place_out = [0; SEQ.len() / 2 + 1];
        SSE41Packer::default().convert(&SEQ, &mut out_of_place_out);

        let packed = SSE41InPlacePacker::default().convert_in_place(&mut buf);

        assert_eq!(out_of_place_out, packed);

        let mut unpacked = [0; SEQ.len()];
        SSSE3Unpacker::default().convert(packed, &mut unpacked);
        assert_eq!(unpacked, SEQ);
    }
}
