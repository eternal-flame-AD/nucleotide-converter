#![no_std]
use core::arch::x86_64::*;

pub trait CodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]);
}

#[derive(Default)]
pub struct NaiveCodeConverter;

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
pub struct NaiveToLowerCodeConverter;

impl CodeConverter for NaiveToLowerCodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        for (a, b) in code.iter().zip(out.iter_mut()) {
            let mut c = *a;
            if c >= b'a' && c <= b'z' {
                c -= b'a' - b'A';
            }

            *b = match c {
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
pub struct LUTCodeConverter;

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
    lower_masknot: __m128i,
}

impl Default for SSE2CodeConverter {
    fn default() -> Self {
        let (a, t, c, g, one, two, three, lower_masknot) = unsafe {
            (
                _mm_set1_epi8(b'A' as _),
                _mm_set1_epi8(b'T' as _),
                _mm_set1_epi8(b'C' as _),
                _mm_set1_epi8(b'G' as _),
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
            lower_masknot,
        }
    }
}

#[cfg(target_feature = "sse4.1")]
macro_rules! mm_blendv_epi8x {
    ($a:expr, $b:expr, $mask:expr) => {
        _mm_blendv_epi8($a, $b, $mask)
    };
}

#[cfg(not(target_feature = "sse4.1"))]
macro_rules! mm_blendv_epi8x {
    ($a:expr, $b:expr, $mask:expr) => {
        _mm_xor_si128(_mm_and_si128($b, $mask), _mm_andnot_si128($mask, $a))
    };
}

impl CodeConverter for SSE2CodeConverter {
    fn convert(&self, mut code: &[u8], mut out: &mut [u8]) {
        assert!(out.len() >= code.len());

        let align_offset = code.as_ptr().align_offset(16).min(code.len());

        self.scalar
            .convert(&code[..align_offset], &mut out[..align_offset]);
        code = &code[align_offset..];
        out = &mut out[align_offset..];

        let mut chunks = code.chunks_exact(16);
        let mut out_chunks = out.chunks_exact_mut(16);
        unsafe {
            for (chunk_in, chunk_out) in (&mut chunks).zip(&mut out_chunks) {
                let chunk_xmm =
                    _mm_andnot_si128(self.lower_masknot, _mm_load_si128(chunk_in.as_ptr().cast()));

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

pub struct AVX2CodeConverter {
    scalar: NaiveCodeConverter,
    a: __m256i,
    t: __m256i,
    c: __m256i,
    g: __m256i,
    one: __m256i,
    two: __m256i,
    three: __m256i,
    lower_masknot: __m256i,
}

impl Default for AVX2CodeConverter {
    fn default() -> Self {
        let (a, t, c, g, one, two, three, lower_masknot) = unsafe {
            (
                _mm256_set1_epi8(b'A' as _),
                _mm256_set1_epi8(b'T' as _),
                _mm256_set1_epi8(b'C' as _),
                _mm256_set1_epi8(b'G' as _),
                _mm256_set1_epi8(1 as _),
                _mm256_set1_epi8(2 as _),
                _mm256_set1_epi8(3 as _),
                _mm256_set1_epi8(0x20 as _),
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
            lower_masknot,
        }
    }
}

cpufeatures::new!(x86_avx2, "avx2");
cpufeatures::new!(x86_avx512vbmi, "avx512vbmi");

impl CodeConverter for AVX2CodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        if x86_avx2::get() {
            unsafe { self.convert_impl(code, out) }
        } else {
            let sse = SSE2CodeConverter::default();
            sse.convert(code, out);
        }
    }
}

impl AVX2CodeConverter {
    #[target_feature(enable = "avx2")]
    fn convert_impl(&self, mut code: &[u8], mut out: &mut [u8]) {
        assert!(out.len() >= code.len());

        let align_offset = code.as_ptr().align_offset(32).min(code.len());

        self.scalar
            .convert(&code[..align_offset], &mut out[..align_offset]);
        code = &code[align_offset..];
        out = &mut out[align_offset..];

        let mut chunks = code.chunks_exact(32);
        let mut out_chunks = out.chunks_exact_mut(32);

        unsafe {
            for (chunk_in, chunk_out) in (&mut chunks).zip(&mut out_chunks) {
                let chunk_ymm = _mm256_andnot_si256(
                    self.lower_masknot,
                    _mm256_load_si256(chunk_in.as_ptr().cast()),
                );
                let is_a = _mm256_cmpeq_epi8(chunk_ymm, self.a);
                let is_t = _mm256_cmpeq_epi8(chunk_ymm, self.t);
                let is_c = _mm256_cmpeq_epi8(chunk_ymm, self.c);
                let is_g = _mm256_cmpeq_epi8(chunk_ymm, self.g);
                let mut result = _mm256_andnot_si256(is_a, _mm256_set1_epi32(!0));
                result = _mm256_blendv_epi8(result, self.one, is_t);
                result = _mm256_blendv_epi8(result, self.two, is_c);
                result = _mm256_blendv_epi8(result, self.three, is_g);
                _mm256_storeu_si256(chunk_out.as_mut_ptr().cast(), result);
            }
        }
        let remainder = chunks.remainder();
        let out_remainder = out_chunks.into_remainder();

        self.scalar.convert(remainder, out_remainder);
    }
}

pub struct AVX512VbmiCodeConverter {
    scalar: NaiveCodeConverter,
    offset: __m512i,
    range: __m512i,
    lut: __m512i,
}

impl Default for AVX512VbmiCodeConverter {
    fn default() -> Self {
        static LUT: Align64<[u8; 64]> = Align64(
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
        Self {
            scalar: NaiveCodeConverter::default(),
            offset: unsafe { _mm512_set1_epi8(b'A' as _) },
            range: unsafe { _mm512_set1_epi8((b'z' - b'A') as _) },
            lut: unsafe { _mm512_load_si512(LUT.0.as_ptr().cast()) },
        }
    }
}

impl CodeConverter for AVX512VbmiCodeConverter {
    fn convert(&self, code: &[u8], out: &mut [u8]) {
        if x86_avx512vbmi::get() {
            unsafe { self.convert_impl(code, out) }
        } else {
            let sse = AVX2CodeConverter::default();
            sse.convert(code, out);
        }
    }
}

impl AVX512VbmiCodeConverter {
    #[target_feature(enable = "avx512vbmi")]
    fn convert_impl(&self, mut code: &[u8], mut out: &mut [u8]) {
        assert!(out.len() >= code.len());

        let align_offset = code.as_ptr().align_offset(64).min(code.len());

        self.scalar
            .convert(&code[..align_offset], &mut out[..align_offset]);
        code = &code[align_offset..];
        out = &mut out[align_offset..];

        let mut chunks = code.chunks_exact(64);
        let mut out_chunks = out.chunks_exact_mut(64);

        unsafe {
            for (chunk_in, chunk_out) in (&mut chunks).zip(&mut out_chunks) {
                let chunk_zmm =
                    _mm512_sub_epi8(_mm512_load_si512(chunk_in.as_ptr().cast()), self.offset);

                let result =
                    _mm512_permutexvar_epi8(_mm512_min_epu8(chunk_zmm, self.range), self.lut);
                _mm512_storeu_si512(chunk_out.as_mut_ptr().cast(), result);
            }
        }
        let remainder = chunks.remainder();
        let out_remainder = out_chunks.into_remainder();

        self.scalar.convert(remainder, out_remainder);
    }
}
