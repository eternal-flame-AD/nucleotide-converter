#![no_std]
#![warn(clippy::all)]
use core::arch::x86_64::*;

pub trait CodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]);
}

#[cold]
fn unlikely() {}
#[derive(Default)]
pub struct NaiveCodeConverter {
    _private: (),
}

impl CodeConverter for NaiveCodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        for (a, b) in code.iter().zip(out.iter_mut()) {
            *b = match *a & (!0x20) {
                b'A' => 0,
                b'T' => 1,
                b'C' => 2,
                b'G' => 3,
                _ => !0,
            }
        }
    }
}

#[derive(Default)]
pub struct NaiveToLowerCodeConverter {
    _private: (),
}

impl CodeConverter for NaiveToLowerCodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        for (a, b) in code.iter().zip(out.iter_mut()) {
            *b = match a.to_ascii_uppercase() {
                b'A' => 0,
                b'T' => 1,
                b'C' => 2,
                b'G' => 3,
                _ => !0,
            }
        }
    }
}

#[derive(Default)]
pub struct LUTCodeConverter {
    _private: (),
}

#[repr(align(64))]
struct Align64<T>(T);

impl CodeConverter for LUTCodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        static LUT: Align64<[u8; 256]> = Align64(
            const {
                let mut lut = [!0u8; 256];
                lut[b'a' as usize] = 0;
                lut[b'A' as usize] = 0;
                lut[b't' as usize] = 1;
                lut[b'T' as usize] = 1;
                lut[b'c' as usize] = 2;
                lut[b'C' as usize] = 2;
                lut[b'g' as usize] = 3;
                lut[b'G' as usize] = 3;
                lut
            },
        );
        for (a, b) in code.iter().zip(out.iter_mut()) {
            *b = LUT.0[*a as usize];
        }
    }
}

pub struct SSE2CodeConverter {
    scalar: NaiveCodeConverter,
    a: __m128i,
    t: __m128i,
    c: __m128i,
    g: __m128i,
    one: __m128i,
    two: __m128i,
    three: __m128i,
    tolower: __m128i,
}

impl Default for SSE2CodeConverter {
    fn default() -> Self {
        let (a, t, c, g, one, two, three, tolower) = unsafe {
            (
                _mm_set1_epi8(b'a' as _),
                _mm_set1_epi8(b't' as _),
                _mm_set1_epi8(b'c' as _),
                _mm_set1_epi8(b'g' as _),
                _mm_set1_epi8(1 as _),
                _mm_set1_epi8(2 as _),
                _mm_set1_epi8(3 as _),
                _mm_set1_epi8(0x20 as _),
            )
        };
        Self {
            scalar: NaiveCodeConverter::default(),
            a,
            t,
            c,
            g,
            one,
            two,
            three,
            tolower,
        }
    }
}

macro_rules! mm_blendv_epi8x {
    ($a:expr, $b:expr, $mask:expr) => {
        _mm_xor_si128(_mm_and_si128($b, $mask), _mm_andnot_si128($mask, $a))
    };
}

impl CodeConverter for SSE2CodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        assert!(out.len() >= code.len());

        let mut chunks = code.chunks_exact(16);
        let mut out_chunks = out.chunks_exact_mut(16);
        unsafe {
            for (chunk_in, chunk_out) in (&mut chunks).zip(&mut out_chunks) {
                let chunk_xmm =
                    _mm_or_si128(self.tolower, _mm_loadu_si128(chunk_in.as_ptr().cast()));

                let is_a = _mm_cmpeq_epi8(chunk_xmm, self.a);
                let is_t = _mm_cmpeq_epi8(chunk_xmm, self.t);
                let is_c = _mm_cmpeq_epi8(chunk_xmm, self.c);
                let is_g = _mm_cmpeq_epi8(chunk_xmm, self.g);
                let mut result = _mm_andnot_si128(is_a, _mm_set1_epi32(!0));
                result = mm_blendv_epi8x!(result, self.one, is_t);
                result = mm_blendv_epi8x!(result, self.two, is_c);
                result = mm_blendv_epi8x!(result, self.three, is_g);
                _mm_storeu_si128(chunk_out.as_mut_ptr().cast(), result);
            }
        }
        let remainder = chunks.remainder();
        let out_remainder = out_chunks.into_remainder();

        self.scalar.convert(remainder, out_remainder);
    }
}

cpufeatures::new!(x86_ssse3, "ssse3");
cpufeatures::new!(x86_avx2, "avx2");
cpufeatures::new!(x86_avx512vbmi, "avx512vbmi");

#[derive(Default)]
pub struct SSSE3CodeConverter {
    scalar: NaiveCodeConverter,
}

impl CodeConverter for SSSE3CodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        if x86_ssse3::get() {
            unsafe { self.convert_impl(code, out) }
        } else {
            unlikely();
            let sse = SSE2CodeConverter::default();
            sse.convert(code, out);
        }
    }
}

impl SSSE3CodeConverter {
    #[target_feature(enable = "ssse3")]
    fn convert_impl(&self, code: &[u8], out: &mut [u8]) {
        assert!(out.len() >= code.len());

        let mut chunks = code.chunks_exact(16);
        let mut out_chunks = out.chunks_exact_mut(16);

        struct ComputeLut<const SHIFT: usize>;

        impl<const SHIFT: usize> ComputeLut<SHIFT> {
            const TABLE: Align64<[u8; 16]> = Align64(
                const {
                    let mut lut = [0u8; 16];
                    lut[((b'a' >> SHIFT) & 0b1111) as usize] = 1;
                    lut[((b't' >> SHIFT) & 0b1111) as usize] = 2;
                    lut[((b'c' >> SHIFT) & 0b1111) as usize] = 3;
                    lut[((b'g' >> SHIFT) & 0b1111) as usize] = 4;
                    lut[((b'A' >> SHIFT) & 0b1111) as usize] = 1;
                    lut[((b'T' >> SHIFT) & 0b1111) as usize] = 2;
                    lut[((b'C' >> SHIFT) & 0b1111) as usize] = 3;
                    lut[((b'G' >> SHIFT) & 0b1111) as usize] = 4;
                    lut
                },
            );
        }
        unsafe {
            let lut_0 = _mm_load_si128(ComputeLut::<0>::TABLE.0.as_ptr().cast());
            let lut_1 = _mm_load_si128(ComputeLut::<1>::TABLE.0.as_ptr().cast());

            for (chunk_in, chunk_out) in (&mut chunks).zip(&mut out_chunks) {
                let chunk_xmm = _mm_subs_epi8(
                    _mm_loadu_si128(chunk_in.as_ptr().cast()),
                    _mm_set1_epi8(0b100_0000), // check the 6-th bit, if it is zero this will set the sign bit
                );

                let shifted_chunk = _mm_srli_epi16(chunk_xmm, 1);

                let result0 = _mm_shuffle_epi8(lut_0, chunk_xmm); // this clears non-ascii characters
                let result1 =
                    _mm_shuffle_epi8(lut_1, _mm_and_si128(shifted_chunk, _mm_set1_epi8(0b1111)));

                let results_mask = _mm_cmpeq_epi8(result0, result1);

                _mm_storeu_si128(
                    chunk_out.as_mut_ptr().cast(),
                    _mm_sub_epi8(_mm_and_si128(results_mask, result0), _mm_set1_epi8(1)),
                );
            }
        }
        let remainder = chunks.remainder();
        let out_remainder = out_chunks.into_remainder();

        self.scalar.convert(remainder, out_remainder);
    }
}

#[derive(Default)]
pub struct AVX2CodeConverter {
    _private: (),
}

impl CodeConverter for AVX2CodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        if x86_avx2::get() {
            unsafe { self.convert_impl(code, out) }
        } else {
            let sse = SSSE3CodeConverter::default();
            sse.convert(code, out);
        }
    }
}

impl AVX2CodeConverter {
    #[target_feature(enable = "avx2")]
    fn convert_impl(&self, code: &[u8], out: &mut [u8]) {
        assert!(out.len() >= code.len());

        let (a, t, c, g, one, two, three, tolower, nil) = (
            _mm256_set1_epi8(b'a' as _),
            _mm256_set1_epi8(b't' as _),
            _mm256_set1_epi8(b'c' as _),
            _mm256_set1_epi8(b'g' as _),
            _mm256_set1_epi8(1 as _),
            _mm256_set1_epi8(2 as _),
            _mm256_set1_epi8(3 as _),
            _mm256_set1_epi8(0x20 as _),
            _mm256_set1_epi8(!0 as _),
        );

        let mut chunks = code.chunks_exact(32);
        let mut out_chunks = out.chunks_exact_mut(32);

        unsafe {
            for (chunk_in, chunk_out) in (&mut chunks).zip(&mut out_chunks) {
                let chunk_ymm =
                    _mm256_or_si256(tolower, _mm256_loadu_si256(chunk_in.as_ptr().cast()));
                let is_a = _mm256_cmpeq_epi8(chunk_ymm, a);
                let is_t = _mm256_cmpeq_epi8(chunk_ymm, t);
                let is_c = _mm256_cmpeq_epi8(chunk_ymm, c);
                let is_g = _mm256_cmpeq_epi8(chunk_ymm, g);
                let mut result = _mm256_andnot_si256(is_a, nil);
                result = _mm256_blendv_epi8(result, one, is_t);
                result = _mm256_blendv_epi8(result, two, is_c);
                result = _mm256_blendv_epi8(result, three, is_g);
                _mm256_storeu_si256(chunk_out.as_mut_ptr().cast(), result);
            }
        }
        let remainder = chunks.remainder();
        let out_remainder = out_chunks.into_remainder();

        NaiveCodeConverter::default().convert(remainder, out_remainder);
    }
}

#[derive(Default)]
pub struct AVX512VbmiCodeConverter {
    _private: (),
}

impl CodeConverter for AVX512VbmiCodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        if x86_avx512vbmi::get() {
            unsafe { self.convert_impl(code, out) }
        } else {
            let fallback = AVX2CodeConverter::default();
            fallback.convert(code, out);
        }
    }
}

impl AVX512VbmiCodeConverter {
    #[target_feature(enable = "avx512vbmi")]
    fn convert_impl(&self, code: &[u8], out: &mut [u8]) {
        assert!(out.len() >= code.len());

        static LUT: Align64<[u8; 64]> = Align64(
            #[allow(clippy::eq_op)]
            const {
                let mut lut = [!0u8; 64];
                lut[b'a' as usize - b'A' as usize] = 0;
                lut[b'A' as usize - b'A' as usize] = 0;
                lut[b't' as usize - b'A' as usize] = 1;
                lut[b'T' as usize - b'A' as usize] = 1;
                lut[b'c' as usize - b'A' as usize] = 2;
                lut[b'C' as usize - b'A' as usize] = 2;
                lut[b'g' as usize - b'A' as usize] = 3;
                lut[b'G' as usize - b'A' as usize] = 3;
                lut
            },
        );

        let mut chunks = code.chunks_exact(64);
        let mut out_chunks = out.chunks_exact_mut(64);

        unsafe {
            let lut = _mm512_load_si512(LUT.0.as_ptr().cast());

            let offset = _mm512_set1_epi8(b'A' as _);
            let range = _mm512_set1_epi8((b'z' - b'A') as _);

            for (chunk_in, chunk_out) in (&mut chunks).zip(&mut out_chunks) {
                let chunk_zmm =
                    _mm512_sub_epi8(_mm512_load_si512(chunk_in.as_ptr().cast()), offset);

                let result = _mm512_permutexvar_epi8(_mm512_min_epu8(chunk_zmm, range), lut);
                _mm512_storeu_si512(chunk_out.as_mut_ptr().cast(), result);
            }
        }
        let remainder = chunks.remainder();
        let out_remainder = out_chunks.into_remainder();

        NaiveCodeConverter::default().convert(remainder, out_remainder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{RngCore, SeedableRng};

    fn test_converter<T: CodeConverter + Default>(converter: &T) {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(1);
        let mut buf = [0u8; 100_000];
        let mut out1_buf = [0u8; 100_000];
        let mut out2_buf = [0u8; 100_000];
        for n in [1, 10, 100, 1000, 10_000, 100_000] {
            let mut code = &mut buf[..n];
            rng.fill_bytes(&mut code);
            let mut out1 = &mut out1_buf[..n];
            let mut out2 = &mut out2_buf[..n];
            NaiveCodeConverter::default().convert(&code, &mut out1);
            converter.convert(&code, &mut out2);
            code.iter()
                .zip(out1.iter())
                .zip(out2.iter())
                .for_each(|((a, b), c)| {
                    assert_eq!(
                        b, c,
                        "incorrect response for input byte {}: expected {}, got {}",
                        a, b, c
                    );
                });
        }
    }

    macro_rules! write_test {
        ($name:ident, $converter:ty) => {
            #[test]
            fn $name() {
                let converter = <$converter>::default();
                test_converter(&converter);
            }
        };
    }

    write_test!(test_naive_converter, NaiveCodeConverter);
    write_test!(test_naive_to_lower_converter, NaiveToLowerCodeConverter);
    write_test!(test_lut_converter, LUTCodeConverter);
    write_test!(test_sse2_converter, SSE2CodeConverter);
    write_test!(test_ssse3_converter, SSSE3CodeConverter);
    write_test!(test_avx2_converter, AVX2CodeConverter);
    write_test!(test_avx512vbmi_converter, AVX512VbmiCodeConverter);
}
