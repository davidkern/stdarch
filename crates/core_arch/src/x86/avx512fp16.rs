use crate::arch::asm;
use crate::core_arch::{simd::*, x86::*};
use crate::intrinsics::simd::*;
use crate::ptr;

/// Set packed half-precision (16-bit) floating-point elements in dst with the supplied values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_set_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_set_ph(
    e7: f16,
    e6: f16,
    e5: f16,
    e4: f16,
    e3: f16,
    e2: f16,
    e1: f16,
    e0: f16,
) -> __m128h {
    __m128h(e0, e1, e2, e3, e4, e5, e6, e7)
}

/// Set packed half-precision (16-bit) floating-point elements in dst with the supplied values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_set_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_set_ph(
    e15: f16,
    e14: f16,
    e13: f16,
    e12: f16,
    e11: f16,
    e10: f16,
    e9: f16,
    e8: f16,
    e7: f16,
    e6: f16,
    e5: f16,
    e4: f16,
    e3: f16,
    e2: f16,
    e1: f16,
    e0: f16,
) -> __m256h {
    __m256h(
        e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15,
    )
}

/// Set packed half-precision (16-bit) floating-point elements in dst with the supplied values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_set_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_set_ph(
    e31: f16,
    e30: f16,
    e29: f16,
    e28: f16,
    e27: f16,
    e26: f16,
    e25: f16,
    e24: f16,
    e23: f16,
    e22: f16,
    e21: f16,
    e20: f16,
    e19: f16,
    e18: f16,
    e17: f16,
    e16: f16,
    e15: f16,
    e14: f16,
    e13: f16,
    e12: f16,
    e11: f16,
    e10: f16,
    e9: f16,
    e8: f16,
    e7: f16,
    e6: f16,
    e5: f16,
    e4: f16,
    e3: f16,
    e2: f16,
    e1: f16,
    e0: f16,
) -> __m512h {
    __m512h(
        e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15, e16, e17, e18, e19,
        e20, e21, e22, e23, e24, e25, e26, e27, e28, e29, e30, e31,
    )
}

/// Copy half-precision (16-bit) floating-point elements from a to the lower element of dst and zero
/// the upper 7 elements.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_set_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_set_sh(a: f16) -> __m128h {
    __m128h(a, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
}

/// Broadcast the half-precision (16-bit) floating-point value a to all elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_set1_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_set1_ph(a: f16) -> __m128h {
    transmute(f16x8::splat(a))
}

/// Broadcast the half-precision (16-bit) floating-point value a to all elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_set1_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_set1_ph(a: f16) -> __m256h {
    transmute(f16x16::splat(a))
}

/// Broadcast the half-precision (16-bit) floating-point value a to all elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_set1_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_set1_ph(a: f16) -> __m512h {
    transmute(f16x32::splat(a))
}

/// Set packed half-precision (16-bit) floating-point elements in dst with the supplied values in reverse order.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_setr_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_setr_ph(
    e0: f16,
    e1: f16,
    e2: f16,
    e3: f16,
    e4: f16,
    e5: f16,
    e6: f16,
    e7: f16,
) -> __m128h {
    __m128h(e0, e1, e2, e3, e4, e5, e6, e7)
}

/// Set packed half-precision (16-bit) floating-point elements in dst with the supplied values in reverse order.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_setr_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_setr_ph(
    e0: f16,
    e1: f16,
    e2: f16,
    e3: f16,
    e4: f16,
    e5: f16,
    e6: f16,
    e7: f16,
    e8: f16,
    e9: f16,
    e10: f16,
    e11: f16,
    e12: f16,
    e13: f16,
    e14: f16,
    e15: f16,
) -> __m256h {
    __m256h(
        e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15,
    )
}

/// Set packed half-precision (16-bit) floating-point elements in dst with the supplied values in reverse order.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_setr_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_setr_ph(
    e0: f16,
    e1: f16,
    e2: f16,
    e3: f16,
    e4: f16,
    e5: f16,
    e6: f16,
    e7: f16,
    e8: f16,
    e9: f16,
    e10: f16,
    e11: f16,
    e12: f16,
    e13: f16,
    e14: f16,
    e15: f16,
    e16: f16,
    e17: f16,
    e18: f16,
    e19: f16,
    e20: f16,
    e21: f16,
    e22: f16,
    e23: f16,
    e24: f16,
    e25: f16,
    e26: f16,
    e27: f16,
    e28: f16,
    e29: f16,
    e30: f16,
    e31: f16,
) -> __m512h {
    __m512h(
        e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15, e16, e17, e18, e19,
        e20, e21, e22, e23, e24, e25, e26, e27, e28, e29, e30, e31,
    )
}

/// Return vector of type __m128h with all elements set to zero.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_setzero_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_setzero_ph() -> __m128h {
    transmute(f16x8::splat(0.0))
}

/// Return vector of type __m256h with all elements set to zero.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_setzero_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_setzero_ph() -> __m256h {
    transmute(f16x16::splat(0.0))
}

/// Return vector of type __m512h with all elements set to zero.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_setzero_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_setzero_ph() -> __m512h {
    transmute(f16x32::splat(0.0))
}

/// Return vector of type `__m128h` with undefined elements. In practice, this returns the all-zero
/// vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_undefined_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_undefined_ph() -> __m128h {
    transmute(f16x8::splat(0.0))
}

/// Return vector of type `__m256h` with undefined elements. In practice, this returns the all-zero
/// vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_undefined_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_undefined_ph() -> __m256h {
    transmute(f16x16::splat(0.0))
}

/// Return vector of type `__m512h` with undefined elements. In practice, this returns the all-zero
/// vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_undefined_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_undefined_ph() -> __m512h {
    transmute(f16x32::splat(0.0))
}

/// Cast vector of type `__m128d` to type `__m128h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_castpd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_castpd_ph(a: __m128d) -> __m128h {
    transmute(a)
}

/// Cast vector of type `__m256d` to type `__m256h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_castpd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_castpd_ph(a: __m256d) -> __m256h {
    transmute(a)
}

/// Cast vector of type `__m512d` to type `__m512h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_castpd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_castpd_ph(a: __m512d) -> __m512h {
    transmute(a)
}

/// Cast vector of type `__m128h` to type `__m128d`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_castph_pd)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_castph_pd(a: __m128h) -> __m128d {
    transmute(a)
}

/// Cast vector of type `__m256h` to type `__m256d`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_castph_pd)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_castph_pd(a: __m256h) -> __m256d {
    transmute(a)
}

/// Cast vector of type `__m512h` to type `__m512d`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_castph_pd)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_castph_pd(a: __m512h) -> __m512d {
    transmute(a)
}

/// Cast vector of type `__m128` to type `__m128h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_castps_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_castps_ph(a: __m128) -> __m128h {
    transmute(a)
}

/// Cast vector of type `__m256` to type `__m256h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_castps_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_castps_ph(a: __m256) -> __m256h {
    transmute(a)
}

/// Cast vector of type `__m512` to type `__m512h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_castps_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_castps_ph(a: __m512) -> __m512h {
    transmute(a)
}

/// Cast vector of type `__m128h` to type `__m128`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_castph_ps)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_castph_ps(a: __m128h) -> __m128 {
    transmute(a)
}

/// Cast vector of type `__m256h` to type `__m256`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_castph_ps)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_castph_ps(a: __m256h) -> __m256 {
    transmute(a)
}

/// Cast vector of type `__m512h` to type `__m512`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_castph_ps)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_castph_ps(a: __m512h) -> __m512 {
    transmute(a)
}

/// Cast vector of type `__m128i` to type `__m128h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_castsi128_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_castsi128_ph(a: __m128i) -> __m128h {
    transmute(a)
}

/// Cast vector of type `__m256i` to type `__m256h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_castsi256_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_castsi256_ph(a: __m256i) -> __m256h {
    transmute(a)
}

/// Cast vector of type `__m512i` to type `__m512h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_castsi512_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_castsi512_ph(a: __m512i) -> __m512h {
    transmute(a)
}

/// Cast vector of type `__m128h` to type `__m128i`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_castph_si128)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_castph_si128(a: __m128h) -> __m128i {
    transmute(a)
}

/// Cast vector of type `__m256h` to type `__m256i`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_castph_si256)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_castph_si256(a: __m256h) -> __m256i {
    transmute(a)
}

/// Cast vector of type `__m512h` to type `__m512i`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_castph_si512)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_castph_si512(a: __m512h) -> __m512i {
    transmute(a)
}

/// Cast vector of type `__m256h` to type `__m128h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_castph256_ph128)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_castph256_ph128(a: __m256h) -> __m128h {
    simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7])
}

/// Cast vector of type `__m512h` to type `__m128h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_castph512_ph128)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_castph512_ph128(a: __m512h) -> __m128h {
    simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7])
}

/// Cast vector of type `__m512h` to type `__m256h`. This intrinsic is only used for compilation and
/// does not generate any instructions, thus it has zero latency.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_castph512_ph256)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_castph512_ph256(a: __m512h) -> __m256h {
    simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
}

/// Cast vector of type `__m128h` to type `__m256h`. The upper 8 elements of the result are undefined.
/// In practice, the upper elements are zeroed. This intrinsic can generate the `vzeroupper` instruction,
/// but most of the time it does not generate any instructions.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_castph128_ph256)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_castph128_ph256(a: __m128h) -> __m256h {
    simd_shuffle!(
        a,
        _mm_undefined_ph(),
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8]
    )
}

/// Cast vector of type `__m128h` to type `__m512h`. The upper 24 elements of the result are undefined.
/// In practice, the upper elements are zeroed. This intrinsic can generate the `vzeroupper` instruction,
/// but most of the time it does not generate any instructions.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_castph128_ph512)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_castph128_ph512(a: __m128h) -> __m512h {
    simd_shuffle!(
        a,
        _mm_undefined_ph(),
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
            8, 8, 8
        ]
    )
}

/// Cast vector of type `__m256h` to type `__m512h`. The upper 16 elements of the result are undefined.
/// In practice, the upper elements are zeroed. This intrinsic can generate the `vzeroupper` instruction,
/// but most of the time it does not generate any instructions.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_castph256_ph512)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_castph256_ph512(a: __m256h) -> __m512h {
    simd_shuffle!(
        a,
        _mm256_undefined_ph(),
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16
        ]
    )
}

/// Cast vector of type `__m256h` to type `__m128h`. The upper 8 elements of the result are zeroed.
/// This intrinsic can generate the `vzeroupper` instruction, but most of the time it does not generate
/// any instructions.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_zextph128_ph256)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_zextph128_ph256(a: __m128h) -> __m256h {
    simd_shuffle!(
        a,
        _mm_setzero_ph(),
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8]
    )
}

/// Cast vector of type `__m128h` to type `__m512h`. The upper 24 elements of the result are zeroed.
/// This intrinsic can generate the `vzeroupper` instruction, but most of the time it does not generate
/// any instructions.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_zextph128_ph512)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_zextph128_ph512(a: __m128h) -> __m512h {
    simd_shuffle!(
        a,
        _mm_setzero_ph(),
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
            8, 8, 8
        ]
    )
}

macro_rules! cmp_asm {
    ($mask_type: ty, $reg: ident, $a: expr, $b: expr) => {{
        let dst: $mask_type;
        crate::arch::asm!(
            "vcmpph {k}, {a}, {b}, {imm8}",
            k = lateout(kreg) dst,
            a = in($reg) $a,
            b = in($reg) $b,
            imm8 = const IMM5,
            options(pure, nomem, nostack)
        );
        dst
    }};
    ($mask_type: ty, $mask: expr, $reg: ident, $a: expr, $b: expr) => {{
        let dst: $mask_type;
        crate::arch::asm!(
            "vcmpph {k} {{ {mask} }}, {a}, {b}, {imm8}",
            k = lateout(kreg) dst,
            mask = in(kreg) $mask,
            a = in($reg) $a,
            b = in($reg) $b,
            imm8 = const IMM5,
            options(pure, nomem, nostack)
        );
        dst
    }};
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and store the results in mask vector k.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmp_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl,avx512f,sse")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_cmp_ph_mask<const IMM5: i32>(a: __m128h, b: __m128h) -> __mmask8 {
    static_assert_uimm_bits!(IMM5, 5);
    cmp_asm!(__mmask8, xmm_reg, a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and store the results in mask vector k using zeromask k (elements are
/// zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_cmp_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl,avx512f,sse")]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_cmp_ph_mask<const IMM5: i32>(
    k1: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __mmask8 {
    static_assert_uimm_bits!(IMM5, 5);
    cmp_asm!(__mmask8, k1, xmm_reg, a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and store the results in mask vector k.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_cmp_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl,avx512f,avx")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_cmp_ph_mask<const IMM5: i32>(a: __m256h, b: __m256h) -> __mmask16 {
    static_assert_uimm_bits!(IMM5, 5);
    cmp_asm!(__mmask16, ymm_reg, a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and store the results in mask vector k using zeromask k (elements are
/// zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_cmp_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl,avx512f,avx")]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_cmp_ph_mask<const IMM5: i32>(
    k1: __mmask16,
    a: __m256h,
    b: __m256h,
) -> __mmask16 {
    static_assert_uimm_bits!(IMM5, 5);
    cmp_asm!(__mmask16, k1, ymm_reg, a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and store the results in mask vector k.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_cmp_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512bw,avx512f")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_cmp_ph_mask<const IMM5: i32>(a: __m512h, b: __m512h) -> __mmask32 {
    static_assert_uimm_bits!(IMM5, 5);
    cmp_asm!(__mmask32, zmm_reg, a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and store the results in mask vector k using zeromask k (elements are
/// zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_cmp_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512bw,avx512f")]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_cmp_ph_mask<const IMM5: i32>(
    k1: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __mmask32 {
    static_assert_uimm_bits!(IMM5, 5);
    cmp_asm!(__mmask32, k1, zmm_reg, a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and store the result in mask vector k. Exceptions can be suppressed by
/// passing _MM_FROUND_NO_EXC in the sae parameter.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmp_round_sh_mask)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_cmp_round_sh_mask<const IMM5: i32, const SAE: i32>(
    a: __m128h,
    b: __m128h,
) -> __mmask8 {
    static_assert_uimm_bits!(IMM5, 5);
    static_assert_sae!(SAE);
    _mm_mask_cmp_round_sh_mask::<IMM5, SAE>(0xff, a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and store the result in mask vector k using zeromask k1. Exceptions can be
/// suppressed by passing _MM_FROUND_NO_EXC in the sae parameter.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_cmp_round_sh_mask)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[rustc_legacy_const_generics(3, 4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_cmp_round_sh_mask<const IMM5: i32, const SAE: i32>(
    k1: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __mmask8 {
    static_assert_uimm_bits!(IMM5, 5);
    static_assert_sae!(SAE);
    vcmpsh(a, b, IMM5, k1, SAE)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and store the result in mask vector k.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmp_sh_mask)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_cmp_sh_mask<const IMM5: i32>(a: __m128h, b: __m128h) -> __mmask8 {
    static_assert_uimm_bits!(IMM5, 5);
    _mm_cmp_round_sh_mask::<IMM5, _MM_FROUND_CUR_DIRECTION>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and store the result in mask vector k using zeromask k1.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_cmp_sh_mask)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_cmp_sh_mask<const IMM5: i32>(
    k1: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __mmask8 {
    static_assert_uimm_bits!(IMM5, 5);
    _mm_mask_cmp_round_sh_mask::<IMM5, _MM_FROUND_CUR_DIRECTION>(k1, a, b)
}

/// Cast vector of type `__m256h` to type `__m512h`. The upper 16 elements of the result are zeroed.
/// This intrinsic can generate the `vzeroupper` instruction, but most of the time it does not generate
/// any instructions.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_zextph256_ph512)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_zextph256_ph512(a: __m256h) -> __m512h {
    simd_shuffle!(
        a,
        _mm256_setzero_ph(),
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 16, 16, 16, 16, 16, 16,
            16, 16, 16, 16, 16, 16, 16, 16
        ]
    )
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and return the boolean result (0 or 1).
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_comi_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_comi_round_sh<const IMM5: i32, const SAE: i32>(a: __m128h, b: __m128h) -> i32 {
    static_assert_uimm_bits!(IMM5, 5);
    static_assert_sae!(SAE);
    vcomish(a, b, IMM5, SAE)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b based on the comparison
/// operand specified by imm8, and return the boolean result (0 or 1).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_comi_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_comi_sh<const IMM5: i32>(a: __m128h, b: __m128h) -> i32 {
    static_assert_uimm_bits!(IMM5, 5);
    _mm_comi_round_sh::<IMM5, _MM_FROUND_CUR_DIRECTION>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for equality, and return
/// the boolean result (0 or 1).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_comieq_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_comieq_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_EQ_OS>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for greater-than-or-equal,
/// and return the boolean result (0 or 1).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_comige_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_comige_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_GE_OS>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for greater-than, and return
/// the boolean result (0 or 1).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_comigt_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_comigt_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_GT_OS>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for less-than-or-equal, and
/// return the boolean result (0 or 1).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_comile_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_comile_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_LE_OS>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for less-than, and return
/// the boolean result (0 or 1).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_comilt_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_comilt_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_LT_OS>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for not-equal, and return
/// the boolean result (0 or 1).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_comineq_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_comineq_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_NEQ_OS>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for equality, and
/// return the boolean result (0 or 1). This instruction will not signal an exception for QNaNs.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_ucomieq_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_ucomieq_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_EQ_OQ>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for greater-than-or-equal,
/// and return the boolean result (0 or 1). This instruction will not signal an exception for QNaNs.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_ucomige_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_ucomige_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_GE_OQ>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for greater-than, and return
/// the boolean result (0 or 1). This instruction will not signal an exception for QNaNs.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_ucomigt_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_ucomigt_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_GT_OQ>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for less-than-or-equal, and
/// return the boolean result (0 or 1). This instruction will not signal an exception for QNaNs.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_ucomile_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_ucomile_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_LE_OQ>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for less-than, and return
/// the boolean result (0 or 1). This instruction will not signal an exception for QNaNs.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_ucomilt_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_ucomilt_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_LT_OQ>(a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b for not-equal, and return
/// the boolean result (0 or 1). This instruction will not signal an exception for QNaNs.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_ucomineq_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_ucomineq_sh(a: __m128h, b: __m128h) -> i32 {
    _mm_comi_sh::<_CMP_NEQ_OQ>(a, b)
}

/// Load 128-bits (composed of 8 packed half-precision (16-bit) floating-point elements) from memory into
/// a new vector. The address must be aligned to 16 bytes or a general-protection exception may be generated.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_load_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_load_ph(mem_addr: *const f16) -> __m128h {
    *mem_addr.cast()
}

/// Load 256-bits (composed of 16 packed half-precision (16-bit) floating-point elements) from memory into
/// a new vector. The address must be aligned to 32 bytes or a general-protection exception may be generated.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_load_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_load_ph(mem_addr: *const f16) -> __m256h {
    *mem_addr.cast()
}

/// Load 512-bits (composed of 32 packed half-precision (16-bit) floating-point elements) from memory into
/// a new vector. The address must be aligned to 64 bytes or a general-protection exception may be generated.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_load_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_load_ph(mem_addr: *const f16) -> __m512h {
    *mem_addr.cast()
}

/// Load a half-precision (16-bit) floating-point element from memory into the lower element of a new vector,
/// and zero the upper elements
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_load_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_load_sh(mem_addr: *const f16) -> __m128h {
    _mm_set_sh(*mem_addr)
}

/// Load a half-precision (16-bit) floating-point element from memory into the lower element of a new vector
/// using writemask k (the element is copied from src when mask bit 0 is not set), and zero the upper elements.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_load_sh)
#[inline]
#[target_feature(enable = "avx512fp16,sse,avx512f")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_load_sh(src: __m128h, k: __mmask8, mem_addr: *const f16) -> __m128h {
    let mut dst = src;
    asm!(
        vpl!("vmovsh {dst}{{{k}}}"),
        dst = inout(xmm_reg) dst,
        k = in(kreg) k,
        p = in(reg) mem_addr,
        options(pure, nomem, nostack, preserves_flags)
    );
    dst
}

/// Load a half-precision (16-bit) floating-point element from memory into the lower element of a new vector
/// using zeromask k (the element is zeroed out when mask bit 0 is not set), and zero the upper elements.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_load_sh)
#[inline]
#[target_feature(enable = "avx512fp16,sse,avx512f")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_load_sh(k: __mmask8, mem_addr: *const f16) -> __m128h {
    let mut dst: __m128h;
    asm!(
        vpl!("vmovsh {dst}{{{k}}}{{z}}"),
        dst = out(xmm_reg) dst,
        k = in(kreg) k,
        p = in(reg) mem_addr,
        options(pure, nomem, nostack, preserves_flags)
    );
    dst
}

/// Load 128-bits (composed of 8 packed half-precision (16-bit) floating-point elements) from memory into
/// a new vector. The address does not need to be aligned to any particular boundary.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_loadu_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_loadu_ph(mem_addr: *const f16) -> __m128h {
    ptr::read_unaligned(mem_addr.cast())
}

/// Load 256-bits (composed of 16 packed half-precision (16-bit) floating-point elements) from memory into
/// a new vector. The address does not need to be aligned to any particular boundary.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_loadu_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_loadu_ph(mem_addr: *const f16) -> __m256h {
    ptr::read_unaligned(mem_addr.cast())
}

/// Load 512-bits (composed of 32 packed half-precision (16-bit) floating-point elements) from memory into
/// a new vector. The address does not need to be aligned to any particular boundary.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_loadu_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_loadu_ph(mem_addr: *const f16) -> __m512h {
    ptr::read_unaligned(mem_addr.cast())
}

/// Move the lower half-precision (16-bit) floating-point element from b to the lower element of dst
/// using writemask k (the element is copied from src when mask bit 0 is not set), and copy the upper
/// 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_move_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_move_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    let mut mov: f16 = simd_extract!(src, 0);
    if (k & 1) != 0 {
        mov = simd_extract!(b, 0);
    }
    simd_insert!(a, 0, mov)
}

/// Move the lower half-precision (16-bit) floating-point element from b to the lower element of dst
/// using zeromask k (the element is zeroed out when mask bit 0 is not set), and copy the upper 7 packed
/// elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_move_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_move_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    let mut mov: f16 = 0.;
    if (k & 1) != 0 {
        mov = simd_extract!(b, 0);
    }
    simd_insert!(a, 0, mov)
}

/// Move the lower half-precision (16-bit) floating-point element from b to the lower element of dst,
/// and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_move_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_move_sh(a: __m128h, b: __m128h) -> __m128h {
    let mov: f16 = simd_extract!(b, 0);
    simd_insert!(a, 0, mov)
}

/// Store 128-bits (composed of 8 packed half-precision (16-bit) floating-point elements) from a into memory.
/// The address must be aligned to 16 bytes or a general-protection exception may be generated.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_store_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_store_ph(mem_addr: *mut f16, a: __m128h) {
    *mem_addr.cast() = a;
}

/// Store 256-bits (composed of 16 packed half-precision (16-bit) floating-point elements) from a into memory.
/// The address must be aligned to 32 bytes or a general-protection exception may be generated.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_store_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_store_ph(mem_addr: *mut f16, a: __m256h) {
    *mem_addr.cast() = a;
}

/// Store 512-bits (composed of 32 packed half-precision (16-bit) floating-point elements) from a into memory.
/// The address must be aligned to 64 bytes or a general-protection exception may be generated.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_store_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_store_ph(mem_addr: *mut f16, a: __m512h) {
    *mem_addr.cast() = a;
}

/// Store the lower half-precision (16-bit) floating-point element from a into memory.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_store_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_store_sh(mem_addr: *mut f16, a: __m128h) {
    *mem_addr = simd_extract!(a, 0);
}

/// Store the lower half-precision (16-bit) floating-point element from a into memory using writemask k
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_store_sh)
#[inline]
#[target_feature(enable = "avx512fp16,sse,avx512f")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_store_sh(mem_addr: *mut f16, k: __mmask8, a: __m128h) {
    asm!(
        vps!("vmovdqu16", "{{{k}}}, {src}"),
        p = in(reg) mem_addr,
        k = in(kreg) k,
        src = in(xmm_reg) a,
        options(nostack, preserves_flags)
    );
}

/// Store 128-bits (composed of 8 packed half-precision (16-bit) floating-point elements) from a into memory.
/// The address does not need to be aligned to any particular boundary.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_storeu_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_storeu_ph(mem_addr: *mut f16, a: __m128h) {
    ptr::write_unaligned(mem_addr.cast(), a);
}

/// Store 256-bits (composed of 16 packed half-precision (16-bit) floating-point elements) from a into memory.
/// The address does not need to be aligned to any particular boundary.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_storeu_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_storeu_ph(mem_addr: *mut f16, a: __m256h) {
    ptr::write_unaligned(mem_addr.cast(), a);
}

/// Store 512-bits (composed of 32 packed half-precision (16-bit) floating-point elements) from a into memory.
/// The address does not need to be aligned to any particular boundary.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_storeu_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_storeu_ph(mem_addr: *mut f16, a: __m512h) {
    ptr::write_unaligned(mem_addr.cast(), a);
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vaddph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_add_ph(a: __m128h, b: __m128h) -> __m128h {
    simd_add(a, b)
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vaddph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_add_ph(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    let r = _mm_add_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vaddph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_add_ph(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    let r = _mm_add_ph(a, b);
    simd_select_bitmask(k, r, _mm_setzero_ph())
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vaddph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_add_ph(a: __m256h, b: __m256h) -> __m256h {
    simd_add(a, b)
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vaddph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_add_ph(src: __m256h, k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    let r = _mm256_add_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vaddph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_add_ph(k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    let r = _mm256_add_ph(a, b);
    simd_select_bitmask(k, r, _mm256_setzero_ph())
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_add_ph(a: __m512h, b: __m512h) -> __m512h {
    simd_add(a, b)
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_add_ph(src: __m512h, k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    let r = _mm512_add_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_add_ph(k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    let r = _mm512_add_ph(a, b);
    simd_select_bitmask(k, r, _mm512_setzero_ph())
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_add_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddph, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_add_round_ph<const ROUNDING: i32>(a: __m512h, b: __m512h) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vaddph(a, b, ROUNDING)
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_add_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_add_round_ph<const ROUNDING: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    let r = _mm512_add_round_ph::<ROUNDING>(a, b);
    simd_select_bitmask(k, r, src)
}

/// Add packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_add_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_add_round_ph<const ROUNDING: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    let r = _mm512_add_round_ph::<ROUNDING>(a, b);
    simd_select_bitmask(k, r, _mm512_setzero_ph())
}

/// Add the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_add_round_sh<const ROUNDING: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_add_round_sh::<ROUNDING>(_mm_undefined_ph(), 0xff, a, b)
}

/// Add the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// writemask k (the element is copied from src when mask bit 0 is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_add_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_add_round_sh<const ROUNDING: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    vaddsh(a, b, src, k, ROUNDING)
}

/// Add the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// zeromask k (the element is zeroed out when mask bit 0 is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_add_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_add_round_sh<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_add_round_sh::<ROUNDING>(_mm_setzero_ph(), k, a, b)
}

/// Add the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_add_sh(a: __m128h, b: __m128h) -> __m128h {
    _mm_add_round_sh::<_MM_FROUND_CUR_DIRECTION>(a, b)
}

/// Add the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// writemask k (the element is copied from src when mask bit 0 is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_add_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_add_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_add_round_sh::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Add the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// zeromask k (the element is zeroed out when mask bit 0 is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_add_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vaddsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_add_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_maskz_add_round_sh::<_MM_FROUND_CUR_DIRECTION>(k, a, b)
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsubph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_sub_ph(a: __m128h, b: __m128h) -> __m128h {
    simd_sub(a, b)
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_sub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsubph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_sub_ph(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    let r = _mm_sub_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_sub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsubph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_sub_ph(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    let r = _mm_sub_ph(a, b);
    simd_select_bitmask(k, r, _mm_setzero_ph())
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_sub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsubph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_sub_ph(a: __m256h, b: __m256h) -> __m256h {
    simd_sub(a, b)
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_sub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsubph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_sub_ph(src: __m256h, k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    let r = _mm256_sub_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_sub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsubph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_sub_ph(k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    let r = _mm256_sub_ph(a, b);
    simd_select_bitmask(k, r, _mm256_setzero_ph())
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_sub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_sub_ph(a: __m512h, b: __m512h) -> __m512h {
    simd_sub(a, b)
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_sub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_sub_ph(src: __m512h, k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    let r = _mm512_sub_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_sub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_sub_ph(k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    let r = _mm512_sub_ph(a, b);
    simd_select_bitmask(k, r, _mm512_setzero_ph())
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_sub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubph, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_sub_round_ph<const ROUNDING: i32>(a: __m512h, b: __m512h) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vsubph(a, b, ROUNDING)
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_sub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_sub_round_ph<const ROUNDING: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    let r = _mm512_sub_round_ph::<ROUNDING>(a, b);
    simd_select_bitmask(k, r, src)
}

/// Subtract packed half-precision (16-bit) floating-point elements in b from a, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_sub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_sub_round_ph<const ROUNDING: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    let r = _mm512_sub_round_ph::<ROUNDING>(a, b);
    simd_select_bitmask(k, r, _mm512_setzero_ph())
}

/// Subtract the lower half-precision (16-bit) floating-point elements in b from a, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_sub_round_sh<const ROUNDING: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_sub_round_sh::<ROUNDING>(_mm_undefined_ph(), 0xff, a, b)
}

/// Subtract the lower half-precision (16-bit) floating-point elements in b from a, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// writemask k (the element is copied from src when mask bit 0 is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_sub_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_sub_round_sh<const ROUNDING: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    vsubsh(a, b, src, k, ROUNDING)
}

/// Subtract the lower half-precision (16-bit) floating-point elements in b from a, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// zeromask k (the element is zeroed out when mask bit 0 is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_sub_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_sub_round_sh<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_sub_round_sh::<ROUNDING>(_mm_setzero_ph(), k, a, b)
}

/// Subtract the lower half-precision (16-bit) floating-point elements in b from a, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_sub_sh(a: __m128h, b: __m128h) -> __m128h {
    _mm_sub_round_sh::<_MM_FROUND_CUR_DIRECTION>(a, b)
}

/// Subtract the lower half-precision (16-bit) floating-point elements in b from a, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// writemask k (the element is copied from src when mask bit 0 is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_sub_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_sub_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_sub_round_sh::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Subtract the lower half-precision (16-bit) floating-point elements in b from a, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// zeromask k (the element is zeroed out when mask bit 0 is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_sub_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsubsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_sub_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_maskz_sub_round_sh::<_MM_FROUND_CUR_DIRECTION>(k, a, b)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmulph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mul_ph(a: __m128h, b: __m128h) -> __m128h {
    simd_mul(a, b)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmulph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_mul_ph(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    let r = _mm_mul_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmulph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_mul_ph(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    let r = _mm_mul_ph(a, b);
    simd_select_bitmask(k, r, _mm_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmulph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mul_ph(a: __m256h, b: __m256h) -> __m256h {
    simd_mul(a, b)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmulph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_mul_ph(src: __m256h, k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    let r = _mm256_mul_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmulph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_mul_ph(k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    let r = _mm256_mul_ph(a, b);
    simd_select_bitmask(k, r, _mm256_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mul_ph(a: __m512h, b: __m512h) -> __m512h {
    simd_mul(a, b)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_mul_ph(src: __m512h, k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    let r = _mm512_mul_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_mul_ph(k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    let r = _mm512_mul_ph(a, b);
    simd_select_bitmask(k, r, _mm512_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mul_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulph, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mul_round_ph<const ROUNDING: i32>(a: __m512h, b: __m512h) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vmulph(a, b, ROUNDING)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_mul_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_mul_round_ph<const ROUNDING: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    let r = _mm512_mul_round_ph::<ROUNDING>(a, b);
    simd_select_bitmask(k, r, src)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_mul_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_mul_round_ph<const ROUNDING: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    let r = _mm512_mul_round_ph::<ROUNDING>(a, b);
    simd_select_bitmask(k, r, _mm512_setzero_ph())
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mul_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mul_round_sh<const ROUNDING: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_mul_round_sh::<ROUNDING>(_mm_undefined_ph(), 0xff, a, b)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// writemask k (the element is copied from src when mask bit 0 is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_mul_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_mul_round_sh<const ROUNDING: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    vmulsh(a, b, src, k, ROUNDING)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// zeromask k (the element is zeroed out when mask bit 0 is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_mul_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_mul_round_sh<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_mul_round_sh::<ROUNDING>(_mm_setzero_ph(), k, a, b)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mul_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mul_sh(a: __m128h, b: __m128h) -> __m128h {
    _mm_mul_round_sh::<_MM_FROUND_CUR_DIRECTION>(a, b)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// writemask k (the element is copied from src when mask bit 0 is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_mul_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_mul_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_mul_round_sh::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// zeromask k (the element is zeroed out when mask bit 0 is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_mul_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmulsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_mul_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_maskz_mul_round_sh::<_MM_FROUND_CUR_DIRECTION>(k, a, b)
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_div_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vdivph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_div_ph(a: __m128h, b: __m128h) -> __m128h {
    simd_div(a, b)
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_div_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vdivph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_div_ph(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    let r = _mm_div_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_div_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vdivph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_div_ph(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    let r = _mm_div_ph(a, b);
    simd_select_bitmask(k, r, _mm_setzero_ph())
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_div_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vdivph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_div_ph(a: __m256h, b: __m256h) -> __m256h {
    simd_div(a, b)
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_div_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vdivph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_div_ph(src: __m256h, k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    let r = _mm256_div_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_div_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vdivph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_div_ph(k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    let r = _mm256_div_ph(a, b);
    simd_select_bitmask(k, r, _mm256_setzero_ph())
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_div_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_div_ph(a: __m512h, b: __m512h) -> __m512h {
    simd_div(a, b)
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_div_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_div_ph(src: __m512h, k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    let r = _mm512_div_ph(a, b);
    simd_select_bitmask(k, r, src)
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_div_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_div_ph(k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    let r = _mm512_div_ph(a, b);
    simd_select_bitmask(k, r, _mm512_setzero_ph())
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_div_round_ph)

#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivph, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_div_round_ph<const ROUNDING: i32>(a: __m512h, b: __m512h) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vdivph(a, b, ROUNDING)
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst using
/// writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_div_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_div_round_ph<const ROUNDING: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    let r = _mm512_div_round_ph::<ROUNDING>(a, b);
    simd_select_bitmask(k, r, src)
}

/// Divide packed half-precision (16-bit) floating-point elements in a by b, and store the results in dst using
/// zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_div_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_div_round_ph<const ROUNDING: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    let r = _mm512_div_round_ph::<ROUNDING>(a, b);
    simd_select_bitmask(k, r, _mm512_setzero_ph())
}

/// Divide the lower half-precision (16-bit) floating-point elements in a by b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_div_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_div_round_sh<const ROUNDING: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_div_round_sh::<ROUNDING>(_mm_undefined_ph(), 0xff, a, b)
}

/// Divide the lower half-precision (16-bit) floating-point elements in a by b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// writemask k (the element is copied from src when mask bit 0 is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_div_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_div_round_sh<const ROUNDING: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    vdivsh(a, b, src, k, ROUNDING)
}

/// Divide the lower half-precision (16-bit) floating-point elements in a by b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// zeromask k (the element is zeroed out when mask bit 0 is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_div_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_div_round_sh<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_div_round_sh::<ROUNDING>(_mm_setzero_ph(), k, a, b)
}

/// Divide the lower half-precision (16-bit) floating-point elements in a by b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_div_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_div_sh(a: __m128h, b: __m128h) -> __m128h {
    _mm_div_round_sh::<_MM_FROUND_CUR_DIRECTION>(a, b)
}

/// Divide the lower half-precision (16-bit) floating-point elements in a by b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// writemask k (the element is copied from src when mask bit 0 is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_div_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_div_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_div_round_sh::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Divide the lower half-precision (16-bit) floating-point elements in a by b, store the result in the
/// lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst using
/// zeromask k (the element is zeroed out when mask bit 0 is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_div_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vdivsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_div_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_maskz_div_round_sh::<_MM_FROUND_CUR_DIRECTION>(k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst. Each complex number is
/// composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mul_pch(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_mul_pch(_mm_undefined_ph(), 0xff, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using writemask k (the element
/// is copied from src when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_mul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_mul_pch(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    transmute(vfmulcph_128(transmute(a), transmute(b), transmute(src), k))
}

/// Multiply packed complex numbers in a and b, and store the results in dst using zeromask k (the element
/// is zeroed out when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_mul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_mul_pch(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_mul_pch(_mm_setzero_ph(), k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst. Each complex number is
/// composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mul_pch(a: __m256h, b: __m256h) -> __m256h {
    _mm256_mask_mul_pch(_mm256_undefined_ph(), 0xff, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using writemask k (the element
/// is copied from src when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_mul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_mul_pch(src: __m256h, k: __mmask8, a: __m256h, b: __m256h) -> __m256h {
    transmute(vfmulcph_256(transmute(a), transmute(b), transmute(src), k))
}

/// Multiply packed complex numbers in a and b, and store the results in dst using zeromask k (the element
/// is zeroed out when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_mul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_mul_pch(k: __mmask8, a: __m256h, b: __m256h) -> __m256h {
    _mm256_mask_mul_pch(_mm256_setzero_ph(), k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst. Each complex number is
/// composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mul_pch(a: __m512h, b: __m512h) -> __m512h {
    _mm512_mask_mul_pch(_mm512_undefined_ph(), 0xffff, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using writemask k (the element
/// is copied from src when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_mul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_mul_pch(src: __m512h, k: __mmask16, a: __m512h, b: __m512h) -> __m512h {
    _mm512_mask_mul_round_pch::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using zeromask k (the element
/// is zeroed out when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_mul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_mul_pch(k: __mmask16, a: __m512h, b: __m512h) -> __m512h {
    _mm512_mask_mul_pch(_mm512_setzero_ph(), k, a, b)
}

/// Multiply the packed complex numbers in a and b, and store the results in dst. Each complex number is
/// composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mul_round_pch<const ROUNDING: i32>(a: __m512h, b: __m512h) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_mask_mul_round_pch::<ROUNDING>(_mm512_undefined_ph(), 0xffff, a, b)
}

/// Multiply the packed complex numbers in a and b, and store the results in dst using writemask k (the element
/// is copied from src when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_mul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_mul_round_pch<const ROUNDING: i32>(
    src: __m512h,
    k: __mmask16,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    transmute(vfmulcph_512(
        transmute(a),
        transmute(b),
        transmute(src),
        k,
        ROUNDING,
    ))
}

/// Multiply the packed complex numbers in a and b, and store the results in dst using zeromask k (the element
/// is zeroed out when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_mul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_mul_round_pch<const ROUNDING: i32>(
    k: __mmask16,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_mask_mul_round_pch::<ROUNDING>(_mm512_setzero_ph(), k, a, b)
}

/// Multiply the lower complex numbers in a and b, and store the result in the lower elements of dst,
/// and copy the upper 6 packed elements from a to the upper elements of dst. Each complex number is
/// composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mul_sch(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_mul_sch(_mm_undefined_ph(), 0xff, a, b)
}

/// Multiply the lower complex numbers in a and b, and store the result in the lower elements of dst using
/// writemask k (the element is copied from src when mask bit 0 is not set), and copy the upper 6 packed
/// elements from a to the upper elements of dst. Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_mul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_mul_sch(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_mul_round_sch::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Multiply the lower complex numbers in a and b, and store the result in the lower elements of dst using
/// zeromask k (the element is zeroed out when mask bit 0 is not set), and copy the upper 6 packed elements
/// from a to the upper elements of dst. Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_mul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_mul_sch(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_mul_sch(_mm_setzero_ph(), k, a, b)
}

/// Multiply the lower complex numbers in a and b, and store the result in the lower elements of dst,
/// and copy the upper 6 packed elements from a to the upper elements of dst. Each complex number is
/// composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mul_round_sch<const ROUNDING: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_mul_round_sch::<ROUNDING>(_mm_undefined_ph(), 0xff, a, b)
}

/// Multiply the lower complex numbers in a and b, and store the result in the lower elements of dst using
/// writemask k (the element is copied from src when mask bit 0 is not set), and copy the upper 6 packed
/// elements from a to the upper elements of dst. Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_mul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_mul_round_sch<const ROUNDING: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    transmute(vfmulcsh(
        transmute(a),
        transmute(b),
        transmute(src),
        k,
        ROUNDING,
    ))
}

/// Multiply the lower complex numbers in a and b, and store the result in the lower elements of dst using
/// zeromask k (the element is zeroed out when mask bit 0 is not set), and copy the upper 6 packed elements
/// from a to the upper elements of dst. Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_mul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_mul_round_sch<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_mul_round_sch::<ROUNDING>(_mm_setzero_ph(), k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst. Each complex number is
/// composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmul_pch(a: __m128h, b: __m128h) -> __m128h {
    _mm_mul_pch(a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using writemask k (the element
/// is copied from src when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmul_pch(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_mul_pch(src, k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using zeromask k (the element
/// is zeroed out when corresponding mask bit is not set). Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmul_pch(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_maskz_mul_pch(k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst. Each complex number is
/// composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_fmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_fmul_pch(a: __m256h, b: __m256h) -> __m256h {
    _mm256_mul_pch(a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using writemask k (the element
/// is copied from src when corresponding mask bit is not set). Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_fmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_fmul_pch(src: __m256h, k: __mmask8, a: __m256h, b: __m256h) -> __m256h {
    _mm256_mask_mul_pch(src, k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using zeromask k (the element
/// is zeroed out when corresponding mask bit is not set). Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_fmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_fmul_pch(k: __mmask8, a: __m256h, b: __m256h) -> __m256h {
    _mm256_maskz_mul_pch(k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst. Each complex number is composed
/// of two adjacent half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmul_pch(a: __m512h, b: __m512h) -> __m512h {
    _mm512_mul_pch(a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using writemask k (the element
/// is copied from src when corresponding mask bit is not set). Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmul_pch(src: __m512h, k: __mmask16, a: __m512h, b: __m512h) -> __m512h {
    _mm512_mask_mul_pch(src, k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using zeromask k (the element
/// is zeroed out when corresponding mask bit is not set). Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmul_pch(k: __mmask16, a: __m512h, b: __m512h) -> __m512h {
    _mm512_maskz_mul_pch(k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst. Each complex number is composed
/// of two adjacent half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmul_round_pch<const ROUNDING: i32>(a: __m512h, b: __m512h) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_mul_round_pch::<ROUNDING>(a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using writemask k (the element
/// is copied from src when corresponding mask bit is not set). Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmul_round_pch<const ROUNDING: i32>(
    src: __m512h,
    k: __mmask16,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_mask_mul_round_pch::<ROUNDING>(src, k, a, b)
}

/// Multiply packed complex numbers in a and b, and store the results in dst using zeromask k (the element
/// is zeroed out when corresponding mask bit is not set). Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmul_round_pch<const ROUNDING: i32>(
    k: __mmask16,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_maskz_mul_round_pch::<ROUNDING>(k, a, b)
}

/// Multiply the lower complex numbers in a and b, and store the results in dst. Each complex number is
/// composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmul_sch(a: __m128h, b: __m128h) -> __m128h {
    _mm_mul_sch(a, b)
}

/// Multiply the lower complex numbers in a and b, and store the results in dst using writemask k (the element
/// is copied from src when mask bit 0 is not set). Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmul_sch(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_mul_sch(src, k, a, b)
}

/// Multiply the lower complex numbers in a and b, and store the results in dst using zeromask k (the element
/// is zeroed out when mask bit 0 is not set). Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmul_sch(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_maskz_mul_sch(k, a, b)
}

/// Multiply the lower complex numbers in a and b, and store the results in dst. Each complex number is composed
/// of two adjacent half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmul_round_sch<const ROUNDING: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mul_round_sch::<ROUNDING>(a, b)
}

/// Multiply the lower complex numbers in a and b, and store the results in dst using writemask k (the element
/// is copied from src when mask bit 0 is not set). Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmul_round_sch<const ROUNDING: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_mul_round_sch::<ROUNDING>(src, k, a, b)
}

/// Multiply the lower complex numbers in a and b, and store the results in dst using zeromask k (the element
/// is zeroed out when mask bit 0 is not set). Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmul_round_sch<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_maskz_mul_round_sch::<ROUNDING>(k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_cmul_pch(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_cmul_pch(_mm_undefined_ph(), 0xff, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using writemask k (the element is copied from src when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_cmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_cmul_pch(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    transmute(vfcmulcph_128(transmute(a), transmute(b), transmute(src), k))
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using zeromask k (the element is zeroed out when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_cmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_cmul_pch(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_cmul_pch(_mm_setzero_ph(), k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_cmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_cmul_pch(a: __m256h, b: __m256h) -> __m256h {
    _mm256_mask_cmul_pch(_mm256_undefined_ph(), 0xff, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using writemask k (the element is copied from src when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_cmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_cmul_pch(src: __m256h, k: __mmask8, a: __m256h, b: __m256h) -> __m256h {
    transmute(vfcmulcph_256(transmute(a), transmute(b), transmute(src), k))
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using zeromask k (the element is zeroed out when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_cmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_cmul_pch(k: __mmask8, a: __m256h, b: __m256h) -> __m256h {
    _mm256_mask_cmul_pch(_mm256_setzero_ph(), k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_cmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_cmul_pch(a: __m512h, b: __m512h) -> __m512h {
    _mm512_mask_cmul_pch(_mm512_undefined_ph(), 0xffff, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using writemask k (the element is copied from src when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_cmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_cmul_pch(src: __m512h, k: __mmask16, a: __m512h, b: __m512h) -> __m512h {
    _mm512_mask_cmul_round_pch::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using zeromask k (the element is zeroed out when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_cmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_cmul_pch(k: __mmask16, a: __m512h, b: __m512h) -> __m512h {
    _mm512_mask_cmul_pch(_mm512_setzero_ph(), k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_cmul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_cmul_round_pch<const ROUNDING: i32>(a: __m512h, b: __m512h) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_mask_cmul_round_pch::<ROUNDING>(_mm512_undefined_ph(), 0xffff, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using writemask k (the element is copied from src when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_cmul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_cmul_round_pch<const ROUNDING: i32>(
    src: __m512h,
    k: __mmask16,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    transmute(vfcmulcph_512(
        transmute(a),
        transmute(b),
        transmute(src),
        k,
        ROUNDING,
    ))
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using zeromask k (the element is zeroed out when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_cmul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_cmul_round_pch<const ROUNDING: i32>(
    k: __mmask16,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_mask_cmul_round_pch::<ROUNDING>(_mm512_setzero_ph(), k, a, b)
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_cmul_sch(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_cmul_sch(_mm_undefined_ph(), 0xff, a, b)
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst using writemask k (the element is copied from src when mask bit 0 is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_cmul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_cmul_sch(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_cmul_round_sch::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst using zeromask k (the element is zeroed out when mask bit 0 is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_cmul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_cmul_sch(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_cmul_sch(_mm_setzero_ph(), k, a, b)
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_cmul_round_sch<const ROUNDING: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_cmul_round_sch::<ROUNDING>(_mm_undefined_ph(), 0xff, a, b)
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst using writemask k (the element is copied from src when mask bit 0 is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_cmul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_cmul_round_sch<const ROUNDING: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    transmute(vfcmulcsh(
        transmute(a),
        transmute(b),
        transmute(src),
        k,
        ROUNDING,
    ))
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst using zeromask k (the element is zeroed out when mask bit 0 is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_cmul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_cmul_round_sch<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_cmul_round_sch::<ROUNDING>(_mm_setzero_ph(), k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fcmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fcmul_pch(a: __m128h, b: __m128h) -> __m128h {
    _mm_cmul_pch(a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using writemask k (the element is copied from src when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fcmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fcmul_pch(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_cmul_pch(src, k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using zeromask k (the element is zeroed out when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fcmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fcmul_pch(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_maskz_cmul_pch(k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_fcmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_fcmul_pch(a: __m256h, b: __m256h) -> __m256h {
    _mm256_cmul_pch(a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using writemask k (the element is copied from src when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_fcmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_fcmul_pch(src: __m256h, k: __mmask8, a: __m256h, b: __m256h) -> __m256h {
    _mm256_mask_cmul_pch(src, k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using zeromask k (the element is zeroed out when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_fcmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_fcmul_pch(k: __mmask8, a: __m256h, b: __m256h) -> __m256h {
    _mm256_maskz_cmul_pch(k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fcmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fcmul_pch(a: __m512h, b: __m512h) -> __m512h {
    _mm512_cmul_pch(a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using writemask k (the element is copied from src when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fcmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fcmul_pch(src: __m512h, k: __mmask16, a: __m512h, b: __m512h) -> __m512h {
    _mm512_mask_cmul_pch(src, k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using zeromask k (the element is zeroed out when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fcmul_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fcmul_pch(k: __mmask16, a: __m512h, b: __m512h) -> __m512h {
    _mm512_maskz_cmul_pch(k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fcmul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fcmul_round_pch<const ROUNDING: i32>(a: __m512h, b: __m512h) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_cmul_round_pch::<ROUNDING>(a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using writemask k (the element is copied from src when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fcmul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fcmul_round_pch<const ROUNDING: i32>(
    src: __m512h,
    k: __mmask16,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_mask_cmul_round_pch::<ROUNDING>(src, k, a, b)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, and
/// store the results in dst using zeromask k (the element is zeroed out when corresponding mask bit is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fcmul_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fcmul_round_pch<const ROUNDING: i32>(
    k: __mmask16,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_maskz_cmul_round_pch::<ROUNDING>(k, a, b)
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fcmul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fcmul_sch(a: __m128h, b: __m128h) -> __m128h {
    _mm_cmul_sch(a, b)
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst using writemask k (the element is copied from src when mask bit 0 is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fcmul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fcmul_sch(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_cmul_sch(src, k, a, b)
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst using zeromask k (the element is zeroed out when mask bit 0 is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fcmul_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fcmul_sch(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_maskz_cmul_sch(k, a, b)
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fcmul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fcmul_round_sch<const ROUNDING: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_cmul_round_sch::<ROUNDING>(a, b)
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst using writemask k (the element is copied from src when mask bit 0 is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fcmul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fcmul_round_sch<const ROUNDING: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_cmul_round_sch::<ROUNDING>(src, k, a, b)
}

/// Multiply the lower complex numbers in a by the complex conjugates of the lower complex numbers in b,
/// and store the results in dst using zeromask k (the element is zeroed out when mask bit 0 is not set).
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fcmul_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmulcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fcmul_round_sch<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_maskz_cmul_round_sch::<ROUNDING>(k, a, b)
}

/// Finds the absolute value of each packed half-precision (16-bit) floating-point element in v2, storing
/// the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_abs_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_abs_ph(v2: __m128h) -> __m128h {
    transmute(_mm_and_si128(transmute(v2), _mm_set1_epi16(i16::MAX)))
}

/// Finds the absolute value of each packed half-precision (16-bit) floating-point element in v2, storing
/// the result in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_abs_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_abs_ph(v2: __m256h) -> __m256h {
    transmute(_mm256_and_si256(transmute(v2), _mm256_set1_epi16(i16::MAX)))
}

/// Finds the absolute value of each packed half-precision (16-bit) floating-point element in v2, storing
/// the result in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_abs_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_abs_ph(v2: __m512h) -> __m512h {
    transmute(_mm512_and_si512(transmute(v2), _mm512_set1_epi16(i16::MAX)))
}

/// Compute the complex conjugates of complex numbers in a, and store the results in dst. Each complex
/// number is composed of two adjacent half-precision (16-bit) floating-point elements, which defines
/// the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate
/// `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_conj_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_conj_pch(a: __m128h) -> __m128h {
    transmute(_mm_xor_si128(transmute(a), _mm_set1_epi32(i32::MIN)))
}

/// Compute the complex conjugates of complex numbers in a, and store the results in dst using writemask k
/// (the element is copied from src when corresponding mask bit is not set). Each complex number is composed of two
/// adjacent half-precision (16-bit) floating-point elements, which defines the complex number
/// `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_conj_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_conj_pch(src: __m128h, k: __mmask8, a: __m128h) -> __m128h {
    let r: __m128 = transmute(_mm_conj_pch(a));
    transmute(simd_select_bitmask(k, r, transmute(src)))
}

/// Compute the complex conjugates of complex numbers in a, and store the results in dst using zeromask k
/// (the element is zeroed out when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_conj_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_conj_pch(k: __mmask8, a: __m128h) -> __m128h {
    _mm_mask_conj_pch(_mm_setzero_ph(), k, a)
}

/// Compute the complex conjugates of complex numbers in a, and store the results in dst. Each complex number
/// is composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_conj_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_conj_pch(a: __m256h) -> __m256h {
    transmute(_mm256_xor_si256(transmute(a), _mm256_set1_epi32(i32::MIN)))
}

/// Compute the complex conjugates of complex numbers in a, and store the results in dst using writemask k
/// (the element is copied from src when corresponding mask bit is not set). Each complex number is composed of two
/// adjacent half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_conj_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_conj_pch(src: __m256h, k: __mmask8, a: __m256h) -> __m256h {
    let r: __m256 = transmute(_mm256_conj_pch(a));
    transmute(simd_select_bitmask(k, r, transmute(src)))
}

/// Compute the complex conjugates of complex numbers in a, and store the results in dst using zeromask k
/// (the element is zeroed out when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_conj_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_conj_pch(k: __mmask8, a: __m256h) -> __m256h {
    _mm256_mask_conj_pch(_mm256_setzero_ph(), k, a)
}

/// Compute the complex conjugates of complex numbers in a, and store the results in dst. Each complex number
/// is composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_conj_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_conj_pch(a: __m512h) -> __m512h {
    transmute(_mm512_xor_si512(transmute(a), _mm512_set1_epi32(i32::MIN)))
}

/// Compute the complex conjugates of complex numbers in a, and store the results in dst using writemask k
/// (the element is copied from src when corresponding mask bit is not set). Each complex number is composed of two
/// adjacent half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_conj_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_conj_pch(src: __m512h, k: __mmask16, a: __m512h) -> __m512h {
    let r: __m512 = transmute(_mm512_conj_pch(a));
    transmute(simd_select_bitmask(k, r, transmute(src)))
}

/// Compute the complex conjugates of complex numbers in a, and store the results in dst using zeromask k
/// (the element is zeroed out when corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_conj_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_conj_pch(k: __mmask16, a: __m512h) -> __m512h {
    _mm512_mask_conj_pch(_mm512_setzero_ph(), k, a)
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmadd_pch(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    _mm_mask3_fmadd_pch(a, b, c, 0xff)
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using writemask k (the element is copied from a when the corresponding
/// mask bit is not set). Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmadd_pch(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    let r: __m128 = transmute(_mm_mask3_fmadd_pch(a, b, c, k)); // using `0xff` would have been fine here, but this is what CLang does
    transmute(simd_select_bitmask(k, r, transmute(a)))
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using writemask k (the element is copied from c when the corresponding
/// mask bit is not set). Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fmadd_pch(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    transmute(vfmaddcph_mask3_128(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
    ))
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using zeromask k (the element is zeroed out when the corresponding mask
/// bit is not set). Each complex number is composed of two adjacent half-precision (16-bit) floating-point
/// elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmadd_pch(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    transmute(vfmaddcph_maskz_128(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
    ))
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_fmadd_pch(a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    _mm256_mask3_fmadd_pch(a, b, c, 0xff)
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using writemask k (the element is copied from a when the corresponding mask
/// bit is not set). Each complex number is composed of two adjacent half-precision (16-bit) floating-point
/// elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_fmadd_pch(a: __m256h, k: __mmask8, b: __m256h, c: __m256h) -> __m256h {
    let r: __m256 = transmute(_mm256_mask3_fmadd_pch(a, b, c, k)); // using `0xff` would have been fine here, but this is what CLang does
    transmute(simd_select_bitmask(k, r, transmute(a)))
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using writemask k (the element is copied from c when the corresponding
/// mask bit is not set). Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask3_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask3_fmadd_pch(a: __m256h, b: __m256h, c: __m256h, k: __mmask8) -> __m256h {
    transmute(vfmaddcph_mask3_256(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
    ))
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using zeromask k (the element is zeroed out when the corresponding mask
/// bit is not set). Each complex number is composed of two adjacent half-precision (16-bit) floating-point
/// elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_fmadd_pch(k: __mmask8, a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    transmute(vfmaddcph_maskz_256(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
    ))
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmadd_pch(a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    _mm512_fmadd_round_pch::<_MM_FROUND_CUR_DIRECTION>(a, b, c)
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using writemask k (the element is copied from a when the corresponding mask
/// bit is not set). Each complex number is composed of two adjacent half-precision (16-bit) floating-point
/// elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmadd_pch(a: __m512h, k: __mmask16, b: __m512h, c: __m512h) -> __m512h {
    _mm512_mask_fmadd_round_pch::<_MM_FROUND_CUR_DIRECTION>(a, k, b, c)
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using writemask k (the element is copied from c when the corresponding
/// mask bit is not set). Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fmadd_pch(a: __m512h, b: __m512h, c: __m512h, k: __mmask16) -> __m512h {
    _mm512_mask3_fmadd_round_pch::<_MM_FROUND_CUR_DIRECTION>(a, b, c, k)
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using zeromask k (the element is zeroed out when the corresponding mask
/// bit is not set). Each complex number is composed of two adjacent half-precision (16-bit) floating-point
/// elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmadd_pch(k: __mmask16, a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    _mm512_maskz_fmadd_round_pch::<_MM_FROUND_CUR_DIRECTION>(k, a, b, c)
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmadd_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmadd_round_pch<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_mask3_fmadd_round_pch::<ROUNDING>(a, b, c, 0xffff)
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using writemask k (the element is copied from a when the corresponding mask
/// bit is not set). Each complex number is composed of two adjacent half-precision (16-bit) floating-point
/// elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmadd_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmadd_round_pch<const ROUNDING: i32>(
    a: __m512h,
    k: __mmask16,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    let r: __m512 = transmute(_mm512_mask3_fmadd_round_pch::<ROUNDING>(a, b, c, k)); // using `0xffff` would have been fine here, but this is what CLang does
    transmute(simd_select_bitmask(k, r, transmute(a)))
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using writemask k (the element is copied from c when the corresponding
/// mask bit is not set). Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fmadd_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fmadd_round_pch<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
    k: __mmask16,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    transmute(vfmaddcph_mask3_512(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
        ROUNDING,
    ))
}

/// Multiply packed complex numbers in a and b, accumulate to the corresponding complex numbers in c,
/// and store the results in dst using zeromask k (the element is zeroed out when the corresponding mask
/// bit is not set). Each complex number is composed of two adjacent half-precision (16-bit) floating-point
/// elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmadd_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmadd_round_pch<const ROUNDING: i32>(
    k: __mmask16,
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    transmute(vfmaddcph_maskz_512(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
        ROUNDING,
    ))
}

/// Multiply the lower complex numbers in a and b, accumulate to the lower complex number in c, and
/// store the result in the lower elements of dst, and copy the upper 6 packed elements from a to the
/// upper elements of dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmadd_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmadd_sch(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    _mm_fmadd_round_sch::<_MM_FROUND_CUR_DIRECTION>(a, b, c)
}

/// Multiply the lower complex numbers in a and b, accumulate to the lower complex number in c, and
/// store the result in the lower elements of dst using writemask k (elements are copied from a when
/// mask bit 0 is not set), and copy the upper 6 packed elements from a to the upper elements of dst.
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements,
/// which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmadd_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmadd_sch(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    _mm_mask_fmadd_round_sch::<_MM_FROUND_CUR_DIRECTION>(a, k, b, c)
}

/// Multiply the lower complex numbers in a and b, accumulate to the lower complex number in c, and
/// store the result in the lower elements of dst using writemask k (elements are copied from c when
/// mask bit 0 is not set), and copy the upper 6 packed elements from a to the upper elements of dst.
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements,
/// which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fmadd_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fmadd_sch(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    _mm_mask3_fmadd_round_sch::<_MM_FROUND_CUR_DIRECTION>(a, b, c, k)
}

/// Multiply the lower complex numbers in a and b, accumulate to the lower complex number in c, and
/// store the result in the lower elements of dst using zeromask k (elements are zeroed out when mask
/// bit 0 is not set), and copy the upper 6 packed elements from a to the upper elements of dst. Each
/// complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmadd_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmadd_sch(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    _mm_maskz_fmadd_round_sch::<_MM_FROUND_CUR_DIRECTION>(k, a, b, c)
}

/// Multiply the lower complex numbers in a and b, accumulate to the lower complex number in c, and
/// store the result in the lower elements of dst. Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmadd_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmadd_round_sch<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    transmute(vfmaddcsh_mask(
        transmute(a),
        transmute(b),
        transmute(c),
        0xff,
        ROUNDING,
    ))
}

/// Multiply the lower complex numbers in a and b, accumulate to the lower complex number in c, and
/// store the result in the lower elements of dst using writemask k (elements are copied from a when
/// mask bit 0 is not set), and copy the upper 6 packed elements from a to the upper elements of dst.
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements,
/// which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmadd_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmadd_round_sch<const ROUNDING: i32>(
    a: __m128h,
    k: __mmask8,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let a = transmute(a);
    let r = vfmaddcsh_mask(a, transmute(b), transmute(c), k, ROUNDING); // using `0xff` would have been fine here, but this is what CLang does
    transmute(_mm_mask_move_ss(a, k, a, r))
}

/// Multiply the lower complex numbers in a and b, accumulate to the lower complex number in c, and
/// store the result in the lower elements of dst using writemask k (elements are copied from c when
/// mask bit 0 is not set), and copy the upper 6 packed elements from a to the upper elements of dst.
/// Each complex number is composed of two adjacent half-precision (16-bit) floating-point elements,
/// which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fmadd_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fmadd_round_sch<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
    k: __mmask8,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let c = transmute(c);
    let r = vfmaddcsh_mask(transmute(a), transmute(b), c, k, ROUNDING);
    transmute(_mm_move_ss(c, r))
}

/// Multiply the lower complex numbers in a and b, accumulate to the lower complex number in c, and
/// store the result in the lower elements of dst using zeromask k (elements are zeroed out when mask
/// bit 0 is not set), and copy the upper 6 packed elements from a to the upper elements of dst. Each
/// complex number is composed of two adjacent half-precision (16-bit) floating-point elements, which
/// defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmadd_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmadd_round_sch<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let a = transmute(a);
    let r = vfmaddcsh_maskz(a, transmute(b), transmute(c), k, ROUNDING);
    transmute(_mm_move_ss(a, r)) // FIXME: If `k == 0`, then LLVM optimized `vfmaddcsh_maskz` to output an all-zero vector, which is incorrect
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst. Each complex number is composed
/// of two adjacent half-precision (16-bit) floating-point elements, which defines the complex number
/// `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fcmadd_pch(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    _mm_mask3_fcmadd_pch(a, b, c, 0xff)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst using writemask k (the element is
/// copied from a when the corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fcmadd_pch(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    let r: __m128 = transmute(_mm_mask3_fcmadd_pch(a, b, c, k)); // using `0xff` would have been fine here, but this is what CLang does
    transmute(simd_select_bitmask(k, r, transmute(a)))
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst using writemask k (the element is
/// copied from c when the corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fcmadd_pch(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    transmute(vfcmaddcph_mask3_128(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
    ))
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst using zeromask k (the element is
/// zeroed out when the corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fcmadd_pch(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    transmute(vfcmaddcph_maskz_128(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
    ))
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst. Each complex number is composed
/// of two adjacent half-precision (16-bit) floating-point elements, which defines the complex number
/// `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_fcmadd_pch(a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    _mm256_mask3_fcmadd_pch(a, b, c, 0xff)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst using writemask k (the element is
/// copied from a when the corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_fcmadd_pch(a: __m256h, k: __mmask8, b: __m256h, c: __m256h) -> __m256h {
    let r: __m256 = transmute(_mm256_mask3_fcmadd_pch(a, b, c, k)); // using `0xff` would have been fine here, but this is what CLang does
    transmute(simd_select_bitmask(k, r, transmute(a)))
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst using writemask k (the element is
/// copied from c when the corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask3_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask3_fcmadd_pch(a: __m256h, b: __m256h, c: __m256h, k: __mmask8) -> __m256h {
    transmute(vfcmaddcph_mask3_256(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
    ))
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst using zeromask k (the element is
/// zeroed out when the corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_fcmadd_pch(k: __mmask8, a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    transmute(vfcmaddcph_maskz_256(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
    ))
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst. Each complex number is composed
/// of two adjacent half-precision (16-bit) floating-point elements, which defines the complex number
/// `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fcmadd_pch(a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    _mm512_fcmadd_round_pch::<_MM_FROUND_CUR_DIRECTION>(a, b, c)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst using writemask k (the element is
/// copied from a when the corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fcmadd_pch(a: __m512h, k: __mmask16, b: __m512h, c: __m512h) -> __m512h {
    _mm512_mask_fcmadd_round_pch::<_MM_FROUND_CUR_DIRECTION>(a, k, b, c)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst using writemask k (the element is
/// copied from c when the corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fcmadd_pch(a: __m512h, b: __m512h, c: __m512h, k: __mmask16) -> __m512h {
    _mm512_mask3_fcmadd_round_pch::<_MM_FROUND_CUR_DIRECTION>(a, b, c, k)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst using zeromask k (the element is
/// zeroed out when the corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fcmadd_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fcmadd_pch(k: __mmask16, a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    _mm512_maskz_fcmadd_round_pch::<_MM_FROUND_CUR_DIRECTION>(k, a, b, c)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst. Each complex number is composed
/// of two adjacent half-precision (16-bit) floating-point elements, which defines the complex number
/// `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fcmadd_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fcmadd_round_pch<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_mask3_fcmadd_round_pch::<ROUNDING>(a, b, c, 0xffff)
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c, and store the results in dst using writemask k (the element is
/// copied from a when the corresponding mask bit is not set). Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fcmadd_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fcmadd_round_pch<const ROUNDING: i32>(
    a: __m512h,
    k: __mmask16,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    let r: __m512 = transmute(_mm512_mask3_fcmadd_round_pch::<ROUNDING>(a, b, c, k)); // using `0xffff` would have been fine here, but this is what CLang does
    transmute(simd_select_bitmask(k, r, transmute(a)))
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c using writemask k (the element is copied from c when the corresponding
/// mask bit is not set), and store the results in dst. Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1`, or the complex
/// conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fcmadd_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fcmadd_round_pch<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
    k: __mmask16,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    transmute(vfcmaddcph_mask3_512(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
        ROUNDING,
    ))
}

/// Multiply packed complex numbers in a by the complex conjugates of packed complex numbers in b, accumulate
/// to the corresponding complex numbers in c using zeromask k (the element is zeroed out when the corresponding
/// mask bit is not set), and store the results in dst. Each complex number is composed of two adjacent half-precision
/// (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1`, or the complex
/// conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fcmadd_round_pch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fcmadd_round_pch<const ROUNDING: i32>(
    k: __mmask16,
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    transmute(vfcmaddcph_maskz_512(
        transmute(a),
        transmute(b),
        transmute(c),
        k,
        ROUNDING,
    ))
}

/// Multiply the lower complex number in a by the complex conjugate of the lower complex number in b,
/// accumulate to the lower complex number in c, and store the result in the lower elements of dst,
/// and copy the upper 6 packed elements from a to the upper elements of dst. Each complex number is
/// composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fcmadd_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fcmadd_sch(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    _mm_fcmadd_round_sch::<_MM_FROUND_CUR_DIRECTION>(a, b, c)
}

/// Multiply the lower complex number in a by the complex conjugate of the lower complex number in b,
/// accumulate to the lower complex number in c, and store the result in the lower elements of dst using
/// writemask k (the element is copied from a when the corresponding mask bit is not set), and copy the upper
/// 6 packed elements from a to the upper elements of dst. Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fcmadd_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fcmadd_sch(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    _mm_mask_fcmadd_round_sch::<_MM_FROUND_CUR_DIRECTION>(a, k, b, c)
}

/// Multiply the lower complex number in a by the complex conjugate of the lower complex number in b,
/// accumulate to the lower complex number in c, and store the result in the lower elements of dst using
/// writemask k (the element is copied from c when the corresponding mask bit is not set), and copy the upper
/// 6 packed elements from a to the upper elements of dst. Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fcmadd_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fcmadd_sch(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    _mm_mask3_fcmadd_round_sch::<_MM_FROUND_CUR_DIRECTION>(a, b, c, k)
}

/// Multiply the lower complex number in a by the complex conjugate of the lower complex number in b,
/// accumulate to the lower complex number in c, and store the result in the lower elements of dst using
/// zeromask k (the element is zeroed out when the corresponding mask bit is not set), and copy the upper
/// 6 packed elements from a to the upper elements of dst. Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fcmadd_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fcmadd_sch(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    _mm_maskz_fcmadd_round_sch::<_MM_FROUND_CUR_DIRECTION>(k, a, b, c)
}

/// Multiply the lower complex number in a by the complex conjugate of the lower complex number in b,
/// accumulate to the lower complex number in c, and store the result in the lower elements of dst,
/// and copy the upper 6 packed elements from a to the upper elements of dst. Each complex number is
/// composed of two adjacent half-precision (16-bit) floating-point elements, which defines the complex
/// number `complex = vec.fp16[0] + i * vec.fp16[1]`, or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fcmadd_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fcmadd_round_sch<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    transmute(vfcmaddcsh_mask(
        transmute(a),
        transmute(b),
        transmute(c),
        0xff,
        ROUNDING,
    ))
}

/// Multiply the lower complex number in a by the complex conjugate of the lower complex number in b,
/// accumulate to the lower complex number in c, and store the result in the lower elements of dst using
/// writemask k (the element is copied from a when the corresponding mask bit is not set), and copy the upper
/// 6 packed elements from a to the upper elements of dst. Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fcmadd_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fcmadd_round_sch<const ROUNDING: i32>(
    a: __m128h,
    k: __mmask8,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let a = transmute(a);
    let r = vfcmaddcsh_mask(a, transmute(b), transmute(c), k, ROUNDING);
    transmute(_mm_mask_move_ss(a, k, a, r))
}

/// Multiply the lower complex number in a by the complex conjugate of the lower complex number in b,
/// accumulate to the lower complex number in c, and store the result in the lower elements of dst using
/// writemask k (the element is copied from c when the corresponding mask bit is not set), and copy the upper
/// 6 packed elements from a to the upper elements of dst. Each complex number is composed of two adjacent
/// half-precision (16-bit) floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1]`,
/// or the complex conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fcmadd_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fcmadd_round_sch<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
    k: __mmask8,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let c = transmute(c);
    let r = vfcmaddcsh_mask(transmute(a), transmute(b), c, k, ROUNDING);
    transmute(_mm_move_ss(c, r))
}

/// Multiply the lower complex number in a by the complex conjugate of the lower complex number in b,
/// accumulate to the lower complex number in c using zeromask k (the element is zeroed out when the corresponding
/// mask bit is not set), and store the result in the lower elements of dst, and copy the upper 6 packed elements
/// from a to the upper elements of dst. Each complex number is composed of two adjacent half-precision (16-bit)
/// floating-point elements, which defines the complex number `complex = vec.fp16[0] + i * vec.fp16[1`, or the complex
/// conjugate `conjugate = vec.fp16[0] - i * vec.fp16[1]`.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fcmadd_round_sch)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfcmaddcsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fcmadd_round_sch<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let a = transmute(a);
    let r = vfcmaddcsh_maskz(a, transmute(b), transmute(c), k, ROUNDING);
    transmute(_mm_move_ss(a, r)) // FIXME: If `k == 0`, then LLVM optimized `vfcmaddcsh_maskz` to output an all-zero vector, which is incorrect
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmadd_ph(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    simd_fma(a, b, c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmadd_ph(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fmadd_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fmadd_ph(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    simd_select_bitmask(k, _mm_fmadd_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmadd_ph(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fmadd_ph(a, b, c), _mm_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_fmadd_ph(a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    simd_fma(a, b, c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_fmadd_ph(a: __m256h, k: __mmask16, b: __m256h, c: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_fmadd_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask3_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask3_fmadd_ph(a: __m256h, b: __m256h, c: __m256h, k: __mmask16) -> __m256h {
    simd_select_bitmask(k, _mm256_fmadd_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_fmadd_ph(k: __mmask16, a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_fmadd_ph(a, b, c), _mm256_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmadd_ph(a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    simd_fma(a, b, c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmadd_ph(a: __m512h, k: __mmask32, b: __m512h, c: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_fmadd_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fmadd_ph(a: __m512h, b: __m512h, c: __m512h, k: __mmask32) -> __m512h {
    simd_select_bitmask(k, _mm512_fmadd_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmadd_ph(k: __mmask32, a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_fmadd_ph(a, b, c), _mm512_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmadd_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vfmaddph_512(a, b, c, ROUNDING)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmadd_round_ph<const ROUNDING: i32>(
    a: __m512h,
    k: __mmask32,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fmadd_round_ph::<ROUNDING>(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fmadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fmadd_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
    k: __mmask32,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fmadd_round_ph::<ROUNDING>(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, add the intermediate
/// result to packed elements in c, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmadd_round_ph<const ROUNDING: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(
        k,
        _mm512_fmadd_round_ph::<ROUNDING>(a, b, c),
        _mm512_setzero_ph(),
    )
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and add the intermediate
/// result to the lower element in c. Store the result in the lower element of dst, and copy the upper
/// 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmadd_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmadd_sh(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    let extracta: f16 = simd_extract!(a, 0);
    let extractb: f16 = simd_extract!(b, 0);
    let extractc: f16 = simd_extract!(c, 0);
    let r = fmaf16(extracta, extractb, extractc);
    simd_insert!(a, 0, r)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and add the intermediate
/// result to the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from a when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the
/// upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmadd_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmadd_sh(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    let mut fmadd: f16 = simd_extract!(a, 0);
    if k & 1 != 0 {
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fmadd = fmaf16(fmadd, extractb, extractc);
    }
    simd_insert!(a, 0, fmadd)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and add the intermediate
/// result to the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from c when the mask bit 0 is not set), and copy the upper 7 packed elements from c to the
/// upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fmadd_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fmadd_sh(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    let mut fmadd: f16 = simd_extract!(c, 0);
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        fmadd = fmaf16(extracta, extractb, fmadd);
    }
    simd_insert!(c, 0, fmadd)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and add the intermediate
/// result to the lower element in c. Store the result in the lower element of dst using zeromask k (the element
/// is zeroed out when mask bit 0 is not set), and copy the upper 7 packed elements from a to the
/// upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmadd_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmadd_sh(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    let mut fmadd: f16 = 0.0;
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fmadd = fmaf16(extracta, extractb, extractc);
    }
    simd_insert!(a, 0, fmadd)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and add the intermediate
/// result to the lower element in c. Store the result in the lower element of dst, and copy the upper
/// 7 packed elements from a to the upper elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmadd_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmadd_round_sh<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let extracta: f16 = simd_extract!(a, 0);
    let extractb: f16 = simd_extract!(b, 0);
    let extractc: f16 = simd_extract!(c, 0);
    let r = vfmaddsh(extracta, extractb, extractc, ROUNDING);
    simd_insert!(a, 0, r)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and add the intermediate
/// result to the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from a when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the
/// upper elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmadd_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmadd_round_sh<const ROUNDING: i32>(
    a: __m128h,
    k: __mmask8,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fmadd: f16 = simd_extract!(a, 0);
    if k & 1 != 0 {
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fmadd = vfmaddsh(fmadd, extractb, extractc, ROUNDING);
    }
    simd_insert!(a, 0, fmadd)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and add the intermediate
/// result to the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from c when the mask bit 0 is not set), and copy the upper 7 packed elements from c to the
/// upper elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fmadd_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fmadd_round_sh<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
    k: __mmask8,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fmadd: f16 = simd_extract!(c, 0);
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        fmadd = vfmaddsh(extracta, extractb, fmadd, ROUNDING);
    }
    simd_insert!(c, 0, fmadd)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and add the intermediate
/// result to the lower element in c. Store the result in the lower element of dst using zeromask k (the element
/// is zeroed out when mask bit 0 is not set), and copy the upper 7 packed elements from a to the
/// upper elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmadd_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmadd_round_sh<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fmadd: f16 = 0.0;
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fmadd = vfmaddsh(extracta, extractb, extractc, ROUNDING);
    }
    simd_insert!(a, 0, fmadd)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst.
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmsub_ph(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    simd_fma(a, b, simd_neg(c))
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmsub_ph(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fmsub_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fmsub_ph(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    simd_select_bitmask(k, _mm_fmsub_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmsub_ph(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fmsub_ph(a, b, c), _mm_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_fmsub_ph(a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    simd_fma(a, b, simd_neg(c))
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_fmsub_ph(a: __m256h, k: __mmask16, b: __m256h, c: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_fmsub_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask3_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask3_fmsub_ph(a: __m256h, b: __m256h, c: __m256h, k: __mmask16) -> __m256h {
    simd_select_bitmask(k, _mm256_fmsub_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_fmsub_ph(k: __mmask16, a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_fmsub_ph(a, b, c), _mm256_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmsub_ph(a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    simd_fma(a, b, simd_neg(c))
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmsub_ph(a: __m512h, k: __mmask32, b: __m512h, c: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_fmsub_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fmsub_ph(a: __m512h, b: __m512h, c: __m512h, k: __mmask32) -> __m512h {
    simd_select_bitmask(k, _mm512_fmsub_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmsub_ph(k: __mmask32, a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_fmsub_ph(a, b, c), _mm512_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmsub_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vfmaddph_512(a, b, simd_neg(c), ROUNDING)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmsub_round_ph<const ROUNDING: i32>(
    a: __m512h,
    k: __mmask32,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fmsub_round_ph::<ROUNDING>(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fmsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fmsub_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
    k: __mmask32,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fmsub_round_ph::<ROUNDING>(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the intermediate result, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmsub_round_ph<const ROUNDING: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(
        k,
        _mm512_fmsub_round_ph::<ROUNDING>(a, b, c),
        _mm512_setzero_ph(),
    )
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract packed elements
/// in c from the intermediate result. Store the result in the lower element of dst, and copy the upper
/// 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmsub_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmsub_sh(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    let extracta: f16 = simd_extract!(a, 0);
    let extractb: f16 = simd_extract!(b, 0);
    let extractc: f16 = simd_extract!(c, 0);
    let r = fmaf16(extracta, extractb, -extractc);
    simd_insert!(a, 0, r)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract packed elements
/// in c from the intermediate result. Store the result in the lower element of dst using writemask k (the element
/// is copied from a when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the
/// upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmsub_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmsub_sh(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    let mut fmsub: f16 = simd_extract!(a, 0);
    if k & 1 != 0 {
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fmsub = fmaf16(fmsub, extractb, -extractc);
    }
    simd_insert!(a, 0, fmsub)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract packed elements
/// in c from the intermediate result. Store the result in the lower element of dst using writemask k (the element
/// is copied from c when the mask bit 0 is not set), and copy the upper 7 packed elements from c to the
/// upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fmsub_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fmsub_sh(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    let mut fmsub: f16 = simd_extract!(c, 0);
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        fmsub = fmaf16(extracta, extractb, -fmsub);
    }
    simd_insert!(c, 0, fmsub)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract packed elements
/// in c from the intermediate result. Store the result in the lower element of dst using zeromask k (the element
/// is zeroed out when mask bit 0 is not set), and copy the upper 7 packed elements from a to the
/// upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmsub_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmsub_sh(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    let mut fmsub: f16 = 0.0;
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fmsub = fmaf16(extracta, extractb, -extractc);
    }
    simd_insert!(a, 0, fmsub)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract packed elements
/// in c from the intermediate result. Store the result in the lower element of dst, and copy the upper
/// 7 packed elements from a to the upper elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmsub_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmsub_round_sh<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let extracta: f16 = simd_extract!(a, 0);
    let extractb: f16 = simd_extract!(b, 0);
    let extractc: f16 = simd_extract!(c, 0);
    let r = vfmaddsh(extracta, extractb, -extractc, ROUNDING);
    simd_insert!(a, 0, r)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract packed elements
/// in c from the intermediate result. Store the result in the lower element of dst using writemask k (the element
/// is copied from a when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the
/// upper elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmsub_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmsub_round_sh<const ROUNDING: i32>(
    a: __m128h,
    k: __mmask8,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fmsub: f16 = simd_extract!(a, 0);
    if k & 1 != 0 {
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fmsub = vfmaddsh(fmsub, extractb, -extractc, ROUNDING);
    }
    simd_insert!(a, 0, fmsub)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract packed elements
/// in c from the intermediate result. Store the result in the lower element of dst using writemask k (the element
/// is copied from c when the mask bit 0 is not set), and copy the upper 7 packed elements from c to the
/// upper elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fmsub_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fmsub_round_sh<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
    k: __mmask8,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fmsub: f16 = simd_extract!(c, 0);
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        fmsub = vfmaddsh(extracta, extractb, -fmsub, ROUNDING);
    }
    simd_insert!(c, 0, fmsub)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract packed elements
/// in c from the intermediate result. Store the result in the lower element of dst using zeromask k (the element
/// is zeroed out when mask bit 0 is not set), and copy the upper 7 packed elements from a to the
/// upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmsub_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmsub_round_sh<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fmsub: f16 = 0.0;
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fmsub = vfmaddsh(extracta, extractb, -extractc, ROUNDING);
    }
    simd_insert!(a, 0, fmsub)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fnmadd_ph(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    simd_fma(simd_neg(a), b, c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fnmadd_ph(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fnmadd_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fnmadd_ph(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    simd_select_bitmask(k, _mm_fnmadd_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fnmadd_ph(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fnmadd_ph(a, b, c), _mm_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_fnmadd_ph(a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    simd_fma(simd_neg(a), b, c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_fnmadd_ph(a: __m256h, k: __mmask16, b: __m256h, c: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_fnmadd_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask3_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask3_fnmadd_ph(a: __m256h, b: __m256h, c: __m256h, k: __mmask16) -> __m256h {
    simd_select_bitmask(k, _mm256_fnmadd_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_fnmadd_ph(k: __mmask16, a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_fnmadd_ph(a, b, c), _mm256_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fnmadd_ph(a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    simd_fma(simd_neg(a), b, c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fnmadd_ph(a: __m512h, k: __mmask32, b: __m512h, c: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_fnmadd_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fnmadd_ph(a: __m512h, b: __m512h, c: __m512h, k: __mmask32) -> __m512h {
    simd_select_bitmask(k, _mm512_fnmadd_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fnmadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fnmadd_ph(k: __mmask32, a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_fnmadd_ph(a, b, c), _mm512_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fnmadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fnmadd_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vfmaddph_512(simd_neg(a), b, c, ROUNDING)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using writemask k (the element is copied
/// from a when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fnmadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fnmadd_round_ph<const ROUNDING: i32>(
    a: __m512h,
    k: __mmask32,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fnmadd_round_ph::<ROUNDING>(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using writemask k (the element is copied
/// from c when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fnmadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fnmadd_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
    k: __mmask32,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fnmadd_round_ph::<ROUNDING>(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract the intermediate
/// result from packed elements in c, and store the results in dst using zeromask k (the element is zeroed
/// out when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fnmadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fnmadd_round_ph<const ROUNDING: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(
        k,
        _mm512_fnmadd_round_ph::<ROUNDING>(a, b, c),
        _mm512_setzero_ph(),
    )
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst, and copy the upper 7 packed
/// elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fnmadd_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fnmadd_sh(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    let extracta: f16 = simd_extract!(a, 0);
    let extractb: f16 = simd_extract!(b, 0);
    let extractc: f16 = simd_extract!(c, 0);
    let r = fmaf16(-extracta, extractb, extractc);
    simd_insert!(a, 0, r)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from a when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper
/// elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fnmadd_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fnmadd_sh(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    let mut fnmadd: f16 = simd_extract!(a, 0);
    if k & 1 != 0 {
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fnmadd = fmaf16(-fnmadd, extractb, extractc);
    }
    simd_insert!(a, 0, fnmadd)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from c when the mask bit 0 is not set), and copy the upper 7 packed elements from c to the upper
/// elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fnmadd_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fnmadd_sh(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    let mut fnmadd: f16 = simd_extract!(c, 0);
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        fnmadd = fmaf16(-extracta, extractb, fnmadd);
    }
    simd_insert!(c, 0, fnmadd)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using zeromask k (the element
/// is zeroed out when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper
/// elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fnmadd_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fnmadd_sh(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    let mut fnmadd: f16 = 0.0;
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fnmadd = fmaf16(-extracta, extractb, extractc);
    }
    simd_insert!(a, 0, fnmadd)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst, and copy the upper 7 packed
/// elements from a to the upper elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fnmadd_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fnmadd_round_sh<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let extracta: f16 = simd_extract!(a, 0);
    let extractb: f16 = simd_extract!(b, 0);
    let extractc: f16 = simd_extract!(c, 0);
    let r = vfmaddsh(-extracta, extractb, extractc, ROUNDING);
    simd_insert!(a, 0, r)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from a when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper
/// elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fnmadd_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fnmadd_round_sh<const ROUNDING: i32>(
    a: __m128h,
    k: __mmask8,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fnmadd: f16 = simd_extract!(a, 0);
    if k & 1 != 0 {
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fnmadd = vfmaddsh(-fnmadd, extractb, extractc, ROUNDING);
    }
    simd_insert!(a, 0, fnmadd)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from c when the mask bit 0 is not set), and copy the upper 7 packed elements from c to the upper
/// elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fnmadd_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fnmadd_round_sh<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
    k: __mmask8,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fnmadd: f16 = simd_extract!(c, 0);
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        fnmadd = vfmaddsh(-extracta, extractb, fnmadd, ROUNDING);
    }
    simd_insert!(c, 0, fnmadd)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using zeromask k (the element
/// is zeroed out when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper
/// elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fnmadd_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fnmadd_round_sh<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fnmadd: f16 = 0.0;
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fnmadd = vfmaddsh(-extracta, extractb, extractc, ROUNDING);
    }
    simd_insert!(a, 0, fnmadd)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fnmsub_ph(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    simd_fma(simd_neg(a), b, simd_neg(c))
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using writemask k (the element is
/// copied from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fnmsub_ph(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fnmsub_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using writemask k (the element is
/// copied from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fnmsub_ph(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    simd_select_bitmask(k, _mm_fnmsub_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using zeromask k (the element is
/// zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fnmsub_ph(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fnmsub_ph(a, b, c), _mm_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_fnmsub_ph(a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    simd_fma(simd_neg(a), b, simd_neg(c))
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using writemask k (the element is
/// copied from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_fnmsub_ph(a: __m256h, k: __mmask16, b: __m256h, c: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_fnmsub_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using writemask k (the element is
/// copied from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask3_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask3_fnmsub_ph(a: __m256h, b: __m256h, c: __m256h, k: __mmask16) -> __m256h {
    simd_select_bitmask(k, _mm256_fnmsub_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using zeromask k (the element is
/// zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_fnmsub_ph(k: __mmask16, a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_fnmsub_ph(a, b, c), _mm256_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fnmsub_ph(a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    simd_fma(simd_neg(a), b, simd_neg(c))
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using writemask k (the element is
/// copied from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fnmsub_ph(a: __m512h, k: __mmask32, b: __m512h, c: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_fnmsub_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using writemask k (the element is
/// copied from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fnmsub_ph(a: __m512h, b: __m512h, c: __m512h, k: __mmask32) -> __m512h {
    simd_select_bitmask(k, _mm512_fnmsub_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using zeromask k (the element is
/// zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fnmsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fnmsub_ph(k: __mmask32, a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_fnmsub_ph(a, b, c), _mm512_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fnmsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fnmsub_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vfmaddph_512(simd_neg(a), b, simd_neg(c), ROUNDING)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using writemask k (the element is
/// copied from a when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fnmsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fnmsub_round_ph<const ROUNDING: i32>(
    a: __m512h,
    k: __mmask32,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fnmsub_round_ph::<ROUNDING>(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using writemask k (the element is
/// copied from c when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fnmsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fnmsub_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
    k: __mmask32,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fnmsub_round_ph::<ROUNDING>(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, subtract packed elements
/// in c from the negated intermediate result, and store the results in dst using zeromask k (the element is
/// zeroed out when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fnmsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fnmsub_round_ph<const ROUNDING: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(
        k,
        _mm512_fnmsub_round_ph::<ROUNDING>(a, b, c),
        _mm512_setzero_ph(),
    )
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst, and copy the upper 7 packed
/// elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fnmsub_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fnmsub_sh(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    let extracta: f16 = simd_extract!(a, 0);
    let extractb: f16 = simd_extract!(b, 0);
    let extractc: f16 = simd_extract!(c, 0);
    let r = fmaf16(-extracta, extractb, -extractc);
    simd_insert!(a, 0, r)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from a when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper
/// elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fnmsub_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fnmsub_sh(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    let mut fnmsub: f16 = simd_extract!(a, 0);
    if k & 1 != 0 {
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fnmsub = fmaf16(-fnmsub, extractb, -extractc);
    }
    simd_insert!(a, 0, fnmsub)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from c when the mask bit 0 is not set), and copy the upper 7 packed elements from c to the upper
/// elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fnmsub_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fnmsub_sh(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    let mut fnmsub: f16 = simd_extract!(c, 0);
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        fnmsub = fmaf16(-extracta, extractb, -fnmsub);
    }
    simd_insert!(c, 0, fnmsub)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using zeromask k (the element
/// is zeroed out when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper
/// elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fnmsub_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fnmsub_sh(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    let mut fnmsub: f16 = 0.0;
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fnmsub = fmaf16(-extracta, extractb, -extractc);
    }
    simd_insert!(a, 0, fnmsub)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst, and copy the upper 7 packed
/// elements from a to the upper elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fnmsub_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fnmsub_round_sh<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let extracta: f16 = simd_extract!(a, 0);
    let extractb: f16 = simd_extract!(b, 0);
    let extractc: f16 = simd_extract!(c, 0);
    let r = vfmaddsh(-extracta, extractb, -extractc, ROUNDING);
    simd_insert!(a, 0, r)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from a when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper
/// elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fnmsub_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fnmsub_round_sh<const ROUNDING: i32>(
    a: __m128h,
    k: __mmask8,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fnmsub: f16 = simd_extract!(a, 0);
    if k & 1 != 0 {
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fnmsub = vfmaddsh(-fnmsub, extractb, -extractc, ROUNDING);
    }
    simd_insert!(a, 0, fnmsub)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using writemask k (the element
/// is copied from c when the mask bit 0 is not set), and copy the upper 7 packed elements from c to the upper
/// elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fnmsub_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fnmsub_round_sh<const ROUNDING: i32>(
    a: __m128h,
    b: __m128h,
    c: __m128h,
    k: __mmask8,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fnmsub: f16 = simd_extract!(c, 0);
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        fnmsub = vfmaddsh(-extracta, extractb, -fnmsub, ROUNDING);
    }
    simd_insert!(c, 0, fnmsub)
}

/// Multiply the lower half-precision (16-bit) floating-point elements in a and b, and subtract the intermediate
/// result from the lower element in c. Store the result in the lower element of dst using zeromask k (the element
/// is zeroed out when the mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper
/// elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fnmsub_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfnmsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fnmsub_round_sh<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
    c: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    let mut fnmsub: f16 = 0.0;
    if k & 1 != 0 {
        let extracta: f16 = simd_extract!(a, 0);
        let extractb: f16 = simd_extract!(b, 0);
        let extractc: f16 = simd_extract!(c, 0);
        fnmsub = vfmaddsh(-extracta, extractb, -extractc, ROUNDING);
    }
    simd_insert!(a, 0, fnmsub)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmaddsub_ph(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    vfmaddsubph_128(a, b, c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmaddsub_ph(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fmaddsub_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fmaddsub_ph(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    simd_select_bitmask(k, _mm_fmaddsub_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using zeromask k
/// (the element is zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmaddsub_ph(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fmaddsub_ph(a, b, c), _mm_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_fmaddsub_ph(a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    vfmaddsubph_256(a, b, c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_fmaddsub_ph(a: __m256h, k: __mmask16, b: __m256h, c: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_fmaddsub_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask3_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask3_fmaddsub_ph(
    a: __m256h,
    b: __m256h,
    c: __m256h,
    k: __mmask16,
) -> __m256h {
    simd_select_bitmask(k, _mm256_fmaddsub_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using zeromask k
/// (the element is zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_fmaddsub_ph(
    k: __mmask16,
    a: __m256h,
    b: __m256h,
    c: __m256h,
) -> __m256h {
    simd_select_bitmask(k, _mm256_fmaddsub_ph(a, b, c), _mm256_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmaddsub_ph(a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    _mm512_fmaddsub_round_ph::<_MM_FROUND_CUR_DIRECTION>(a, b, c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmaddsub_ph(a: __m512h, k: __mmask32, b: __m512h, c: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_fmaddsub_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fmaddsub_ph(
    a: __m512h,
    b: __m512h,
    c: __m512h,
    k: __mmask32,
) -> __m512h {
    simd_select_bitmask(k, _mm512_fmaddsub_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using zeromask k
/// (the element is zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmaddsub_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddsub))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmaddsub_ph(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    simd_select_bitmask(k, _mm512_fmaddsub_ph(a, b, c), _mm512_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmaddsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmaddsub_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vfmaddsubph_512(a, b, c, ROUNDING)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from a when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmaddsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmaddsub_round_ph<const ROUNDING: i32>(
    a: __m512h,
    k: __mmask32,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fmaddsub_round_ph::<ROUNDING>(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from c when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fmaddsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fmaddsub_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
    k: __mmask32,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fmaddsub_round_ph::<ROUNDING>(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively add and
/// subtract packed elements in c to/from the intermediate result, and store the results in dst using zeromask k
/// (the element is zeroed out when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmaddsub_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmaddsub, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmaddsub_round_ph<const ROUNDING: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(
        k,
        _mm512_fmaddsub_round_ph::<ROUNDING>(a, b, c),
        _mm512_setzero_ph(),
    )
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fmsubadd_ph(a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    vfmaddsubph_128(a, b, simd_neg(c))
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fmsubadd_ph(a: __m128h, k: __mmask8, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fmsubadd_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask3_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask3_fmsubadd_ph(a: __m128h, b: __m128h, c: __m128h, k: __mmask8) -> __m128h {
    simd_select_bitmask(k, _mm_fmsubadd_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using zeromask k
/// (the element is zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_fmsubadd_ph(k: __mmask8, a: __m128h, b: __m128h, c: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_fmsubadd_ph(a, b, c), _mm_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_fmsubadd_ph(a: __m256h, b: __m256h, c: __m256h) -> __m256h {
    vfmaddsubph_256(a, b, simd_neg(c))
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_fmsubadd_ph(a: __m256h, k: __mmask16, b: __m256h, c: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_fmsubadd_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask3_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask3_fmsubadd_ph(
    a: __m256h,
    b: __m256h,
    c: __m256h,
    k: __mmask16,
) -> __m256h {
    simd_select_bitmask(k, _mm256_fmsubadd_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using zeromask k
/// (the element is zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_fmsubadd_ph(
    k: __mmask16,
    a: __m256h,
    b: __m256h,
    c: __m256h,
) -> __m256h {
    simd_select_bitmask(k, _mm256_fmsubadd_ph(a, b, c), _mm256_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmsubadd_ph(a: __m512h, b: __m512h, c: __m512h) -> __m512h {
    _mm512_fmsubadd_round_ph::<_MM_FROUND_CUR_DIRECTION>(a, b, c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from a when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmsubadd_ph(a: __m512h, k: __mmask32, b: __m512h, c: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_fmsubadd_ph(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from c when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fmsubadd_ph(
    a: __m512h,
    b: __m512h,
    c: __m512h,
    k: __mmask32,
) -> __m512h {
    simd_select_bitmask(k, _mm512_fmsubadd_ph(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using zeromask k
/// (the element is zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmsubadd_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsubadd))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmsubadd_ph(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    simd_select_bitmask(k, _mm512_fmsubadd_ph(a, b, c), _mm512_setzero_ph())
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fmsubadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsubadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fmsubadd_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vfmaddsubph_512(a, b, simd_neg(c), ROUNDING)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from a when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fmsubadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsubadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fmsubadd_round_ph<const ROUNDING: i32>(
    a: __m512h,
    k: __mmask32,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fmsubadd_round_ph::<ROUNDING>(a, b, c), a)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using writemask k
/// (the element is copied from c when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask3_fmsubadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsubadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask3_fmsubadd_round_ph<const ROUNDING: i32>(
    a: __m512h,
    b: __m512h,
    c: __m512h,
    k: __mmask32,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_fmsubadd_round_ph::<ROUNDING>(a, b, c), c)
}

/// Multiply packed half-precision (16-bit) floating-point elements in a and b, alternatively subtract
/// and add packed elements in c to/from the intermediate result, and store the results in dst using zeromask k
/// (the element is zeroed out when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_fmsubadd_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfmsubadd, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_fmsubadd_round_ph<const ROUNDING: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
    c: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(
        k,
        _mm512_fmsubadd_round_ph::<ROUNDING>(a, b, c),
        _mm512_setzero_ph(),
    )
}

/// Compute the approximate reciprocal of packed 16-bit floating-point elements in `a` and stores the results in `dst`.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_rcp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrcpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_rcp_ph(a: __m128h) -> __m128h {
    _mm_mask_rcp_ph(_mm_undefined_ph(), 0xff, a)
}

/// Compute the approximate reciprocal of packed 16-bit floating-point elements in `a` and stores the results in `dst`
/// using writemask `k` (elements are copied from `src` when the corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_rcp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrcpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_rcp_ph(src: __m128h, k: __mmask8, a: __m128h) -> __m128h {
    vrcpph_128(a, src, k)
}

/// Compute the approximate reciprocal of packed 16-bit floating-point elements in `a` and stores the results in `dst`
/// using zeromask `k` (elements are zeroed out when the corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_rcp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrcpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_rcp_ph(k: __mmask8, a: __m128h) -> __m128h {
    _mm_mask_rcp_ph(_mm_setzero_ph(), k, a)
}

/// Compute the approximate reciprocal of packed 16-bit floating-point elements in `a` and stores the results in `dst`.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_rcp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrcpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_rcp_ph(a: __m256h) -> __m256h {
    _mm256_mask_rcp_ph(_mm256_undefined_ph(), 0xffff, a)
}

/// Compute the approximate reciprocal of packed 16-bit floating-point elements in `a` and stores the results in `dst`
/// using writemask `k` (elements are copied from `src` when the corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_rcp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrcpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_rcp_ph(src: __m256h, k: __mmask16, a: __m256h) -> __m256h {
    vrcpph_256(a, src, k)
}

/// Compute the approximate reciprocal of packed 16-bit floating-point elements in `a` and stores the results in `dst`
/// using zeromask `k` (elements are zeroed out when the corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_rcp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrcpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_rcp_ph(k: __mmask16, a: __m256h) -> __m256h {
    _mm256_mask_rcp_ph(_mm256_setzero_ph(), k, a)
}

/// Compute the approximate reciprocal of packed 16-bit floating-point elements in `a` and stores the results in `dst`.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_rcp_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrcpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_rcp_ph(a: __m512h) -> __m512h {
    _mm512_mask_rcp_ph(_mm512_undefined_ph(), 0xffffffff, a)
}

/// Compute the approximate reciprocal of packed 16-bit floating-point elements in `a` and stores the results in `dst`
/// using writemask `k` (elements are copied from `src` when the corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_rcp_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrcpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_rcp_ph(src: __m512h, k: __mmask32, a: __m512h) -> __m512h {
    vrcpph_512(a, src, k)
}

/// Compute the approximate reciprocal of packed 16-bit floating-point elements in `a` and stores the results in `dst`
/// using zeromask `k` (elements are zeroed out when the corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_rcp_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrcpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_rcp_ph(k: __mmask32, a: __m512h) -> __m512h {
    _mm512_mask_rcp_ph(_mm512_setzero_ph(), k, a)
}

/// Compute the approximate reciprocal of the lower half-precision (16-bit) floating-point element in b,
/// store the result in the lower element of dst, and copy the upper 7 packed elements from a to the
/// upper elements of dst.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_rcp_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrcpsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_rcp_sh(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_rcp_sh(_mm_undefined_ph(), 0xff, a, b)
}

/// Compute the approximate reciprocal of the lower half-precision (16-bit) floating-point element in b,
/// store the result in the lower element of dst using writemask k (the element is copied from src when
/// mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_rcp_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrcpsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_rcp_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    vrcpsh(a, b, src, k)
}

/// Compute the approximate reciprocal of the lower half-precision (16-bit) floating-point element in b,
/// store the result in the lower element of dst using zeromask k (the element is zeroed out when mask bit 0
/// is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_rcp_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrcpsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_rcp_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_rcp_sh(_mm_setzero_ph(), k, a, b)
}

/// Compute the approximate reciprocal square root of packed half-precision (16-bit) floating-point
/// elements in a, and store the results in dst.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_rsqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_rsqrt_ph(a: __m128h) -> __m128h {
    _mm_mask_rsqrt_ph(_mm_undefined_ph(), 0xff, a)
}

/// Compute the approximate reciprocal square root of packed half-precision (16-bit) floating-point
/// elements in a, and store the results in dst using writemask k (elements are copied from src when
/// the corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_rsqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_rsqrt_ph(src: __m128h, k: __mmask8, a: __m128h) -> __m128h {
    vrsqrtph_128(a, src, k)
}

/// Compute the approximate reciprocal square root of packed half-precision (16-bit) floating-point
/// elements in a, and store the results in dst using zeromask k (elements are zeroed out when the
/// corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_rsqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_rsqrt_ph(k: __mmask8, a: __m128h) -> __m128h {
    _mm_mask_rsqrt_ph(_mm_setzero_ph(), k, a)
}

/// Compute the approximate reciprocal square root of packed half-precision (16-bit) floating-point
/// elements in a, and store the results in dst.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_rsqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_rsqrt_ph(a: __m256h) -> __m256h {
    _mm256_mask_rsqrt_ph(_mm256_undefined_ph(), 0xffff, a)
}

/// Compute the approximate reciprocal square root of packed half-precision (16-bit) floating-point
/// elements in a, and store the results in dst using writemask k (elements are copied from src when
/// the corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_rsqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_rsqrt_ph(src: __m256h, k: __mmask16, a: __m256h) -> __m256h {
    vrsqrtph_256(a, src, k)
}

/// Compute the approximate reciprocal square root of packed half-precision (16-bit) floating-point
/// elements in a, and store the results in dst using zeromask k (elements are zeroed out when the
/// corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_rsqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_rsqrt_ph(k: __mmask16, a: __m256h) -> __m256h {
    _mm256_mask_rsqrt_ph(_mm256_setzero_ph(), k, a)
}

/// Compute the approximate reciprocal square root of packed half-precision (16-bit) floating-point
/// elements in a, and store the results in dst.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_rsqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_rsqrt_ph(a: __m512h) -> __m512h {
    _mm512_mask_rsqrt_ph(_mm512_undefined_ph(), 0xffffffff, a)
}

/// Compute the approximate reciprocal square root of packed half-precision (16-bit) floating-point
/// elements in a, and store the results in dst using writemask k (elements are copied from src when
/// the corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_rsqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_rsqrt_ph(src: __m512h, k: __mmask32, a: __m512h) -> __m512h {
    vrsqrtph_512(a, src, k)
}

/// Compute the approximate reciprocal square root of packed half-precision (16-bit) floating-point
/// elements in a, and store the results in dst using zeromask k (elements are zeroed out when the
/// corresponding mask bit is not set).
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_rsqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_rsqrt_ph(k: __mmask32, a: __m512h) -> __m512h {
    _mm512_mask_rsqrt_ph(_mm512_setzero_ph(), k, a)
}

/// Compute the approximate reciprocal square root of the lower half-precision (16-bit) floating-point
/// element in b, store the result in the lower element of dst, and copy the upper 7 packed elements from a
/// to the upper elements of dst.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_rsqrt_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrsqrtsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_rsqrt_sh(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_rsqrt_sh(_mm_undefined_ph(), 0xff, a, b)
}

/// Compute the approximate reciprocal square root of the lower half-precision (16-bit) floating-point
/// element in b, store the result in the lower element of dst using writemask k (the element is copied from src
/// when mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_rsqrt_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrsqrtsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_rsqrt_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    vrsqrtsh(a, b, src, k)
}

/// Compute the approximate reciprocal square root of the lower half-precision (16-bit) floating-point
/// element in b, store the result in the lower element of dst using zeromask k (the element is zeroed out when
/// mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
/// The maximum relative error for this approximation is less than `1.5*2^-12`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_rsqrt_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrsqrtsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_rsqrt_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_rsqrt_sh(_mm_setzero_ph(), k, a, b)
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_sqrt_ph(a: __m128h) -> __m128h {
    simd_fsqrt(a)
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_sqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_sqrt_ph(src: __m128h, k: __mmask8, a: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_sqrt_ph(a), src)
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_sqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_sqrt_ph(k: __mmask8, a: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_sqrt_ph(a), _mm_setzero_ph())
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_sqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_sqrt_ph(a: __m256h) -> __m256h {
    simd_fsqrt(a)
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_sqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_sqrt_ph(src: __m256h, k: __mmask16, a: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_sqrt_ph(a), src)
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_sqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_sqrt_ph(k: __mmask16, a: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_sqrt_ph(a), _mm256_setzero_ph())
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_sqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_sqrt_ph(a: __m512h) -> __m512h {
    simd_fsqrt(a)
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_sqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_sqrt_ph(src: __m512h, k: __mmask32, a: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_sqrt_ph(a), src)
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_sqrt_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_sqrt_ph(k: __mmask32, a: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_sqrt_ph(a), _mm512_setzero_ph())
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_sqrt_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtph, ROUNDING = 8))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_sqrt_round_ph<const ROUNDING: i32>(a: __m512h) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vsqrtph_512(a, ROUNDING)
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_sqrt_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_sqrt_round_ph<const ROUNDING: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_sqrt_round_ph::<ROUNDING>(a), src)
}

/// Compute the square root of packed half-precision (16-bit) floating-point elements in a, and store the
/// results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_sqrt_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtph, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_sqrt_round_ph<const ROUNDING: i32>(k: __mmask32, a: __m512h) -> __m512h {
    static_assert_rounding!(ROUNDING);
    simd_select_bitmask(k, _mm512_sqrt_round_ph::<ROUNDING>(a), _mm512_setzero_ph())
}

/// Compute the square root of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst, and copy the upper 7 packed elements from a to the upper
/// elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sqrt_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_sqrt_sh(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_sqrt_sh(_mm_undefined_ph(), 0xff, a, b)
}

/// Compute the square root of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst using writemask k (the element is copied from src when mask
/// bit 0 is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_sqrt_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_sqrt_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_sqrt_round_sh::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Compute the square root of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst using zeromask k (the element is zeroed out when mask bit 0
/// is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_sqrt_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_sqrt_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_sqrt_sh(_mm_setzero_ph(), k, a, b)
}

/// Compute the square root of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst, and copy the upper 7 packed elements from a to the upper
/// elements of dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sqrt_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_sqrt_round_sh<const ROUNDING: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_sqrt_round_sh::<ROUNDING>(_mm_undefined_ph(), 0xff, a, b)
}

/// Compute the square root of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst using writemask k (the element is copied from src when mask
/// bit 0 is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_sqrt_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_sqrt_round_sh<const ROUNDING: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    vsqrtsh(a, b, src, k, ROUNDING)
}

/// Compute the square root of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst using zeromask k (the element is zeroed out when mask bit 0
/// is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_sqrt_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vsqrtsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_sqrt_round_sh<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_sqrt_round_sh::<ROUNDING>(_mm_setzero_ph(), k, a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst. Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum
/// value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_max_ph(a: __m128h, b: __m128h) -> __m128h {
    vmaxph_128(a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_max_ph(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_max_ph(a, b), src)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_max_ph(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_max_ph(a, b), _mm_setzero_ph())
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst. Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum
/// value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_max_ph(a: __m256h, b: __m256h) -> __m256h {
    vmaxph_256(a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_max_ph(src: __m256h, k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_max_ph(a, b), src)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_max_ph(k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_max_ph(a, b), _mm256_setzero_ph())
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst. Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum
/// value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmaxph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_max_ph(a: __m512h, b: __m512h) -> __m512h {
    _mm512_max_round_ph::<_MM_FROUND_CUR_DIRECTION>(a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmaxph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_max_ph(src: __m512h, k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_max_ph(a, b), src)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmaxph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_max_ph(k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_max_ph(a, b), _mm512_setzero_ph())
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter.
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_max_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmaxph, SAE = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_max_round_ph<const SAE: i32>(a: __m512h, b: __m512h) -> __m512h {
    static_assert_sae!(SAE);
    vmaxph_512(a, b, SAE)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter. Does not follow the
/// IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_max_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmaxph, SAE = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_max_round_ph<const SAE: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_sae!(SAE);
    simd_select_bitmask(k, _mm512_max_round_ph::<SAE>(a, b), src)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed maximum
/// values in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter. Does not follow the
/// IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_max_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vmaxph, SAE = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_max_round_ph<const SAE: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_sae!(SAE);
    simd_select_bitmask(k, _mm512_max_round_ph::<SAE>(a, b), _mm512_setzero_ph())
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the maximum
/// value in the lower element of dst, and copy the upper 7 packed elements from a to the upper elements
/// of dst. Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value
/// when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_max_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_max_sh(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_max_sh(_mm_undefined_ph(), 0xff, a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the maximum
/// value in the lower element of dst using writemask k (the element is copied from src when mask bit 0
/// is not set), and copy the upper 7 packed elements from a to the upper elements of dst. Does not follow
/// the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_max_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_max_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_max_round_sh::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the maximum value
/// in the lower element of dst using zeromask k (the element is zeroed out when mask bit 0 is not set), and
/// copy the upper 7 packed elements from a to the upper elements of dst. Does not follow the IEEE Standard
/// for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_max_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_max_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_max_sh(_mm_setzero_ph(), k, a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the maximum value
/// in the lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst.
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter. Does not follow the
/// IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_max_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxsh, SAE = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_max_round_sh<const SAE: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_sae!(SAE);
    _mm_mask_max_round_sh::<SAE>(_mm_undefined_ph(), 0xff, a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the maximum value
/// in the lower element of dst using writemask k (the element is copied from src when mask bit 0 is not set),
/// and copy the upper 7 packed elements from a to the upper elements of dst. Exceptions can be suppressed by
/// passing _MM_FROUND_NO_EXC in the sae parameter. Does not follow the IEEE Standard for Floating-Point Arithmetic
/// (IEEE 754) maximum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_max_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxsh, SAE = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_max_round_sh<const SAE: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_sae!(SAE);
    vmaxsh(a, b, src, k, SAE)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the maximum value
/// in the lower element of dst using zeromask k (the element is zeroed out when mask bit 0 is not set), and
/// copy the upper 7 packed elements from a to the upper elements of dst. Exceptions can be suppressed by
/// passing _MM_FROUND_NO_EXC in the sae parameter. Does not follow the IEEE Standard for Floating-Point Arithmetic
/// (IEEE 754) maximum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_max_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vmaxsh, SAE = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_max_round_sh<const SAE: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_sae!(SAE);
    _mm_mask_max_round_sh::<SAE>(_mm_setzero_ph(), k, a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst. Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value
/// when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_min_ph(a: __m128h, b: __m128h) -> __m128h {
    vminph_128(a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_min_ph(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_min_ph(a, b), src)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_min_ph(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    simd_select_bitmask(k, _mm_min_ph(a, b), _mm_setzero_ph())
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst. Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value
/// when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_min_ph(a: __m256h, b: __m256h) -> __m256h {
    vminph_256(a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_min_ph(src: __m256h, k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_min_ph(a, b), src)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_min_ph(k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    simd_select_bitmask(k, _mm256_min_ph(a, b), _mm256_setzero_ph())
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst. Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value
/// when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vminph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_min_ph(a: __m512h, b: __m512h) -> __m512h {
    _mm512_min_round_ph::<_MM_FROUND_CUR_DIRECTION>(a, b)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vminph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_min_ph(src: __m512h, k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_min_ph(a, b), src)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are
/// NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vminph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_min_ph(k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    simd_select_bitmask(k, _mm512_min_ph(a, b), _mm512_setzero_ph())
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter. Does not
/// follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_min_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vminph, SAE = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_min_round_ph<const SAE: i32>(a: __m512h, b: __m512h) -> __m512h {
    static_assert_sae!(SAE);
    vminph_512(a, b, SAE)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter. Does not follow the
/// IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_min_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vminph, SAE = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_min_round_ph<const SAE: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_sae!(SAE);
    simd_select_bitmask(k, _mm512_min_round_ph::<SAE>(a, b), src)
}

/// Compare packed half-precision (16-bit) floating-point elements in a and b, and store packed minimum
/// values in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter. Does not follow the
/// IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_min_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vminph, SAE = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_min_round_ph<const SAE: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_sae!(SAE);
    simd_select_bitmask(k, _mm512_min_round_ph::<SAE>(a, b), _mm512_setzero_ph())
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the minimum
/// value in the lower element of dst, and copy the upper 7 packed elements from a to the upper elements
/// of dst. Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when
/// inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_min_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_min_sh(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_min_sh(_mm_undefined_ph(), 0xff, a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the minimum
/// value in the lower element of dst using writemask k (the element is copied from src when mask bit 0
/// is not set), and copy the upper 7 packed elements from a to the upper elements of dst. Does not follow
/// the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_min_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_min_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_min_round_sh::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the minimum value
/// in the lower element of dst using zeromask k (the element is zeroed out when mask bit 0 is not set), and
/// copy the upper 7 packed elements from a to the upper elements of dst. Does not follow the IEEE Standard
/// for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_min_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_min_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_min_sh(_mm_setzero_ph(), k, a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the minimum value
/// in the lower element of dst, and copy the upper 7 packed elements from a to the upper elements of dst.
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter. Does not follow the
/// IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_min_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminsh, SAE = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_min_round_sh<const SAE: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_sae!(SAE);
    _mm_mask_min_round_sh::<SAE>(_mm_undefined_ph(), 0xff, a, b)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the minimum value
/// in the lower element of dst using writemask k (the element is copied from src when mask bit 0 is not set),
/// and copy the upper 7 packed elements from a to the upper elements of dst. Exceptions can be suppressed by
/// passing _MM_FROUND_NO_EXC in the sae parameter. Does not follow the IEEE Standard for Floating-Point Arithmetic
/// (IEEE 754) minimum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_min_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminsh, SAE = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_min_round_sh<const SAE: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_sae!(SAE);
    vminsh(a, b, src, k, SAE)
}

/// Compare the lower half-precision (16-bit) floating-point elements in a and b, store the minimum value
/// in the lower element of dst using zeromask k (the element is zeroed out when mask bit 0 is not set), and
/// copy the upper 7 packed elements from a to the upper elements of dst. Exceptions can be suppressed by
/// passing _MM_FROUND_NO_EXC in the sae parameter. Does not follow the IEEE Standard for Floating-Point Arithmetic
/// (IEEE 754) minimum value when inputs are NaN or signed-zero values.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_min_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vminsh, SAE = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_min_round_sh<const SAE: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_sae!(SAE);
    _mm_mask_min_round_sh::<SAE>(_mm_setzero_ph(), k, a, b)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst.
/// This intrinsic essentially calculates `floor(log2(x))` for each element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_getexp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetexpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_getexp_ph(a: __m128h) -> __m128h {
    _mm_mask_getexp_ph(_mm_undefined_ph(), 0xff, a)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst using writemask k
/// (elements are copied from src when the corresponding mask bit is not set). This intrinsic essentially calculates
/// `floor(log2(x))` for each element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_getexp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetexpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_getexp_ph(src: __m128h, k: __mmask8, a: __m128h) -> __m128h {
    vgetexpph_128(a, src, k)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst using zeromask
/// k (elements are zeroed out when the corresponding mask bit is not set). This intrinsic essentially calculates
/// `floor(log2(x))` for each element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_getexp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetexpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_getexp_ph(k: __mmask8, a: __m128h) -> __m128h {
    _mm_mask_getexp_ph(_mm_setzero_ph(), k, a)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst.
/// This intrinsic essentially calculates `floor(log2(x))` for each element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_getexp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetexpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_getexp_ph(a: __m256h) -> __m256h {
    _mm256_mask_getexp_ph(_mm256_undefined_ph(), 0xffff, a)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst using writemask k
/// (elements are copied from src when the corresponding mask bit is not set). This intrinsic essentially calculates
/// `floor(log2(x))` for each element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_getexp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetexpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_getexp_ph(src: __m256h, k: __mmask16, a: __m256h) -> __m256h {
    vgetexpph_256(a, src, k)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst using zeromask
/// k (elements are zeroed out when the corresponding mask bit is not set). This intrinsic essentially calculates
/// `floor(log2(x))` for each element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_getexp_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetexpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_getexp_ph(k: __mmask16, a: __m256h) -> __m256h {
    _mm256_mask_getexp_ph(_mm256_setzero_ph(), k, a)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst.
/// This intrinsic essentially calculates `floor(log2(x))` for each element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_getexp_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_getexp_ph(a: __m512h) -> __m512h {
    _mm512_mask_getexp_ph(_mm512_undefined_ph(), 0xffffffff, a)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst using writemask k
/// (elements are copied from src when the corresponding mask bit is not set). This intrinsic essentially calculates
/// `floor(log2(x))` for each element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_getexp_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_getexp_ph(src: __m512h, k: __mmask32, a: __m512h) -> __m512h {
    _mm512_mask_getexp_round_ph::<_MM_FROUND_CUR_DIRECTION>(src, k, a)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst using zeromask
/// k (elements are zeroed out when the corresponding mask bit is not set). This intrinsic essentially calculates
/// `floor(log2(x))` for each element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_getexp_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_getexp_ph(k: __mmask32, a: __m512h) -> __m512h {
    _mm512_mask_getexp_ph(_mm512_setzero_ph(), k, a)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst.
/// This intrinsic essentially calculates `floor(log2(x))` for each element. Exceptions can be suppressed
/// by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_getexp_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpph, SAE = 8))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_getexp_round_ph<const SAE: i32>(a: __m512h) -> __m512h {
    static_assert_sae!(SAE);
    _mm512_mask_getexp_round_ph::<SAE>(_mm512_undefined_ph(), 0xffffffff, a)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst using writemask k
/// (elements are copied from src when the corresponding mask bit is not set). This intrinsic essentially calculates
/// `floor(log2(x))` for each element. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_getexp_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpph, SAE = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_getexp_round_ph<const SAE: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_sae!(SAE);
    vgetexpph_512(a, src, k, SAE)
}

/// Convert the exponent of each packed half-precision (16-bit) floating-point element in a to a half-precision
/// (16-bit) floating-point number representing the integer exponent, and store the results in dst using zeromask
/// k (elements are zeroed out when the corresponding mask bit is not set). This intrinsic essentially calculates
/// `floor(log2(x))` for each element. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_getexp_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpph, SAE = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_getexp_round_ph<const SAE: i32>(k: __mmask32, a: __m512h) -> __m512h {
    static_assert_sae!(SAE);
    _mm512_mask_getexp_round_ph::<SAE>(_mm512_setzero_ph(), k, a)
}

/// Convert the exponent of the lower half-precision (16-bit) floating-point element in b to a half-precision
/// (16-bit) floating-point number representing the integer exponent, store the result in the lower element
/// of dst, and copy the upper 7 packed elements from a to the upper elements of dst. This intrinsic essentially
/// calculates `floor(log2(x))` for the lower element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_getexp_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_getexp_sh(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_getexp_sh(_mm_undefined_ph(), 0xff, a, b)
}

/// Convert the exponent of the lower half-precision (16-bit) floating-point element in b to a half-precision
/// (16-bit) floating-point number representing the integer exponent, store the result in the lower element
/// of dst using writemask k (the element is copied from src when mask bit 0 is not set), and copy the upper 7
/// packed elements from a to the upper elements of dst. This intrinsic essentially calculates `floor(log2(x))`
/// for the lower element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_getexp_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_getexp_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_getexp_round_sh::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Convert the exponent of the lower half-precision (16-bit) floating-point element in b to a half-precision
/// (16-bit) floating-point number representing the integer exponent, store the result in the lower element
/// of dst using zeromask k (the element is zeroed out when mask bit 0 is not set), and copy the upper 7 packed
/// elements from a to the upper elements of dst. This intrinsic essentially calculates `floor(log2(x))` for the
/// lower element.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_getexp_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_getexp_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_getexp_sh(_mm_setzero_ph(), k, a, b)
}

/// Convert the exponent of the lower half-precision (16-bit) floating-point element in b to a half-precision
/// (16-bit) floating-point number representing the integer exponent, store the result in the lower element
/// of dst, and copy the upper 7 packed elements from a to the upper elements of dst. This intrinsic essentially
/// calculates `floor(log2(x))` for the lower element. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC
/// in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_getexp_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpsh, SAE = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_getexp_round_sh<const SAE: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_sae!(SAE);
    _mm_mask_getexp_round_sh::<SAE>(_mm_undefined_ph(), 0xff, a, b)
}

/// Convert the exponent of the lower half-precision (16-bit) floating-point element in b to a half-precision
/// (16-bit) floating-point number representing the integer exponent, store the result in the lower element
/// of dst using writemask k (the element is copied from src when mask bit 0 is not set), and copy the upper 7
/// packed elements from a to the upper elements of dst. This intrinsic essentially calculates `floor(log2(x))`
/// for the lower element. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_getexp_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpsh, SAE = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_getexp_round_sh<const SAE: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_sae!(SAE);
    vgetexpsh(a, b, src, k, SAE)
}

/// Convert the exponent of the lower half-precision (16-bit) floating-point element in b to a half-precision
/// (16-bit) floating-point number representing the integer exponent, store the result in the lower element
/// of dst using zeromask k (the element is zeroed out when mask bit 0 is not set), and copy the upper 7 packed
/// elements from a to the upper elements of dst. This intrinsic essentially calculates `floor(log2(x))` for the
/// lower element. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_getexp_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetexpsh, SAE = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_getexp_round_sh<const SAE: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_sae!(SAE);
    _mm_mask_getexp_round_sh::<SAE>(_mm_setzero_ph(), k, a, b)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst. This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends
/// on the interval range defined by norm and the sign depends on sign and the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_getmant_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(1, 2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_getmant_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    a: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    _mm_mask_getmant_ph::<NORM, SIGN>(_mm_undefined_ph(), 0xff, a)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends on the interval range defined
/// by norm and the sign depends on sign and the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_getmant_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(3, 4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_getmant_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    vgetmantph_128(a, (SIGN << 2) | NORM, src, k)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends on the interval range defined
/// by norm and the sign depends on sign and the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_getmant_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_getmant_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    k: __mmask8,
    a: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    _mm_mask_getmant_ph::<NORM, SIGN>(_mm_setzero_ph(), k, a)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst. This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends
/// on the interval range defined by norm and the sign depends on sign and the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_getmant_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(1, 2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_getmant_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    a: __m256h,
) -> __m256h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    _mm256_mask_getmant_ph::<NORM, SIGN>(_mm256_undefined_ph(), 0xffff, a)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends on the interval range defined
/// by norm and the sign depends on sign and the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_getmant_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(3, 4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_getmant_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    src: __m256h,
    k: __mmask16,
    a: __m256h,
) -> __m256h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    vgetmantph_256(a, (SIGN << 2) | NORM, src, k)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends on the interval range defined
/// by norm and the sign depends on sign and the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_getmant_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_getmant_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    k: __mmask16,
    a: __m256h,
) -> __m256h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    _mm256_mask_getmant_ph::<NORM, SIGN>(_mm256_setzero_ph(), k, a)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst. This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends
/// on the interval range defined by norm and the sign depends on sign and the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_getmant_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(1, 2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_getmant_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    _mm512_mask_getmant_ph::<NORM, SIGN>(_mm512_undefined_ph(), 0xffffffff, a)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends on the interval range defined
/// by norm and the sign depends on sign and the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_getmant_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(3, 4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_getmant_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    _mm512_mask_getmant_round_ph::<NORM, SIGN, _MM_FROUND_CUR_DIRECTION>(src, k, a)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends on the interval range defined
/// by norm and the sign depends on sign and the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_getmant_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_getmant_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    _mm512_mask_getmant_ph::<NORM, SIGN>(_mm512_setzero_ph(), k, a)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst. This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends
/// on the interval range defined by norm and the sign depends on sign and the source sign. Exceptions can
/// be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_getmant_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0, SAE = 8))]
#[rustc_legacy_const_generics(1, 2, 3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_getmant_round_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
    const SAE: i32,
>(
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    static_assert_sae!(SAE);
    _mm512_mask_getmant_round_ph::<NORM, SIGN, SAE>(_mm512_undefined_ph(), 0xffffffff, a)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
/// This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends on the interval range defined
/// by norm and the sign depends on sign and the source sign. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC
/// in the sae parameter
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_getmant_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0, SAE = 8))]
#[rustc_legacy_const_generics(3, 4, 5)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_getmant_round_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
    const SAE: i32,
>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    static_assert_sae!(SAE);
    vgetmantph_512(a, (SIGN << 2) | NORM, src, k, SAE)
}

/// Normalize the mantissas of packed half-precision (16-bit) floating-point elements in a, and store
/// the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
/// This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends on the interval range defined
/// by norm and the sign depends on sign and the source sign. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC
/// in the sae parameter
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_getmant_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantph, NORM = 0, SIGN = 0, SAE = 8))]
#[rustc_legacy_const_generics(2, 3, 4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_getmant_round_ph<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
    const SAE: i32,
>(
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    static_assert_sae!(SAE);
    _mm512_mask_getmant_round_ph::<NORM, SIGN, SAE>(_mm512_setzero_ph(), k, a)
}

/// Normalize the mantissas of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst, and copy the upper 7 packed elements from a to the upper
/// elements of dst. This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends
/// on the interval range defined by norm and the sign depends on sign and the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_getmant_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantsh, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_getmant_sh<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    _mm_mask_getmant_sh::<NORM, SIGN>(_mm_undefined_ph(), 0xff, a, b)
}

/// Normalize the mantissas of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst using writemask k (the element is copied from src when mask bit 0 is not set),
/// and copy the upper 7 packed elements from a to the upper elements of dst. This intrinsic essentially calculates
/// `±(2^k)*|x.significand|`, where k depends on the interval range defined by norm and the sign depends on sign and
/// the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_getmant_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantsh, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(4, 5)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_getmant_sh<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    _mm_mask_getmant_round_sh::<NORM, SIGN, _MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Normalize the mantissas of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst using zeromask k (the element is zeroed out when mask bit 0 is not set),
/// and copy the upper 7 packed elements from a to the upper elements of dst. This intrinsic essentially calculates
/// `±(2^k)*|x.significand|`, where k depends on the interval range defined by norm and the sign depends on sign and
/// the source sign.
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_getmant_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantsh, NORM = 0, SIGN = 0))]
#[rustc_legacy_const_generics(3, 4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_getmant_sh<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    _mm_mask_getmant_sh::<NORM, SIGN>(_mm_setzero_ph(), k, a, b)
}

/// Normalize the mantissas of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst, and copy the upper 7 packed elements from a to the upper
/// elements of dst. This intrinsic essentially calculates `±(2^k)*|x.significand|`, where k depends
/// on the interval range defined by norm and the sign depends on sign and the source sign. Exceptions can
/// be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_getmant_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantsh, NORM = 0, SIGN = 0, SAE = 8))]
#[rustc_legacy_const_generics(2, 3, 4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_getmant_round_sh<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
    const SAE: i32,
>(
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    static_assert_sae!(SAE);
    _mm_mask_getmant_round_sh::<NORM, SIGN, SAE>(_mm_undefined_ph(), 0xff, a, b)
}

/// Normalize the mantissas of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst using writemask k (the element is copied from src when mask bit 0 is not set),
/// and copy the upper 7 packed elements from a to the upper elements of dst. This intrinsic essentially calculates
/// `±(2^k)*|x.significand|`, where k depends on the interval range defined by norm and the sign depends on sign and
/// the source sign. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_getmant_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantsh, NORM = 0, SIGN = 0, SAE = 8))]
#[rustc_legacy_const_generics(4, 5, 6)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_getmant_round_sh<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
    const SAE: i32,
>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    static_assert_sae!(SAE);
    vgetmantsh(a, b, (SIGN << 2) | NORM, src, k, SAE)
}

/// Normalize the mantissas of the lower half-precision (16-bit) floating-point element in b, store
/// the result in the lower element of dst using zeromask k (the element is zeroed out when mask bit 0 is not set),
/// and copy the upper 7 packed elements from a to the upper elements of dst. This intrinsic essentially calculates
/// `±(2^k)*|x.significand|`, where k depends on the interval range defined by norm and the sign depends on sign and
/// the source sign. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// The mantissa is normalized to the interval specified by interv, which can take the following values:
///
///     _MM_MANT_NORM_1_2     // interval [1, 2)
///     _MM_MANT_NORM_p5_2    // interval [0.5, 2)
///     _MM_MANT_NORM_p5_1    // interval [0.5, 1)
///     _MM_MANT_NORM_p75_1p5 // interval [0.75, 1.5)
///
/// The sign is determined by sc which can take the following values:
///
///     _MM_MANT_SIGN_src     // sign = sign(src)
///     _MM_MANT_SIGN_zero    // sign = 0
///     _MM_MANT_SIGN_nan     // dst = NaN if sign(src) = 1
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_getmant_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vgetmantsh, NORM = 0, SIGN = 0, SAE = 8))]
#[rustc_legacy_const_generics(3, 4, 5)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_getmant_round_sh<
    const NORM: _MM_MANTISSA_NORM_ENUM,
    const SIGN: _MM_MANTISSA_SIGN_ENUM,
    const SAE: i32,
>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(NORM, 4);
    static_assert_uimm_bits!(SIGN, 2);
    static_assert_sae!(SAE);
    _mm_mask_getmant_round_sh::<NORM, SIGN, SAE>(_mm_setzero_ph(), k, a, b)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_roundscale_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_roundscale_ph<const IMM8: i32>(a: __m128h) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm_mask_roundscale_ph::<IMM8>(_mm_undefined_ph(), 0xff, a)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst using writemask k (elements are copied from src when
/// the corresponding mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_roundscale_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_roundscale_ph<const IMM8: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    vrndscaleph_128(a, IMM8, src, k)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst using zeromask k (elements are zeroed out when the corresponding
/// mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_roundscale_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_roundscale_ph<const IMM8: i32>(k: __mmask8, a: __m128h) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm_mask_roundscale_ph::<IMM8>(_mm_setzero_ph(), k, a)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_roundscale_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_roundscale_ph<const IMM8: i32>(a: __m256h) -> __m256h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm256_mask_roundscale_ph::<IMM8>(_mm256_undefined_ph(), 0xffff, a)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst using writemask k (elements are copied from src when
/// the corresponding mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_roundscale_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_roundscale_ph<const IMM8: i32>(
    src: __m256h,
    k: __mmask16,
    a: __m256h,
) -> __m256h {
    static_assert_uimm_bits!(IMM8, 8);
    vrndscaleph_256(a, IMM8, src, k)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst using zeromask k (elements are zeroed out when the corresponding
/// mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_roundscale_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_roundscale_ph<const IMM8: i32>(k: __mmask16, a: __m256h) -> __m256h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm256_mask_roundscale_ph::<IMM8>(_mm256_setzero_ph(), k, a)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_roundscale_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_roundscale_ph<const IMM8: i32>(a: __m512h) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm512_mask_roundscale_ph::<IMM8>(_mm512_undefined_ph(), 0xffffffff, a)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst using writemask k (elements are copied from src when
/// the corresponding mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_roundscale_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_roundscale_ph<const IMM8: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm512_mask_roundscale_round_ph::<IMM8, _MM_FROUND_CUR_DIRECTION>(src, k, a)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst using zeromask k (elements are zeroed out when the corresponding
/// mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_roundscale_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_roundscale_ph<const IMM8: i32>(k: __mmask32, a: __m512h) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm512_mask_roundscale_ph::<IMM8>(_mm512_setzero_ph(), k, a)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst. Exceptions can be suppressed by passing _MM_FROUND_NO_EXC
/// in the sae parameter
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_roundscale_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(1, 2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_roundscale_round_ph<const IMM8: i32, const SAE: i32>(a: __m512h) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    _mm512_mask_roundscale_round_ph::<IMM8, SAE>(_mm512_undefined_ph(), 0xffffffff, a)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst using writemask k (elements are copied from src when
/// the corresponding mask bit is not set). Exceptions can be suppressed by passing _MM_FROUND_NO_EXC
/// in the sae parameter
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_roundscale_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(3, 4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_roundscale_round_ph<const IMM8: i32, const SAE: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    vrndscaleph_512(a, IMM8, src, k, SAE)
}

/// Round packed half-precision (16-bit) floating-point elements in a to the number of fraction bits
/// specified by imm8, and store the results in dst using zeromask k (elements are zeroed out when the corresponding
/// mask bit is not set). Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_roundscale_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscaleph, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_roundscale_round_ph<const IMM8: i32, const SAE: i32>(
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    _mm512_mask_roundscale_round_ph::<IMM8, SAE>(_mm512_setzero_ph(), k, a)
}

/// Round the lower half-precision (16-bit) floating-point element in b to the number of fraction bits
/// specified by imm8, store the result in the lower element of dst, and copy the upper 7 packed elements
/// from a to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_roundscale_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscalesh, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_roundscale_sh<const IMM8: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm_mask_roundscale_sh::<IMM8>(_mm_undefined_ph(), 0xff, a, b)
}

/// Round the lower half-precision (16-bit) floating-point element in b to the number of fraction bits
/// specified by imm8, store the result in the lower element of dst using writemask k (the element is copied
/// from src when mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_roundscale_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscalesh, IMM8 = 0))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_roundscale_sh<const IMM8: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm_mask_roundscale_round_sh::<IMM8, _MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Round the lower half-precision (16-bit) floating-point element in b to the number of fraction bits
/// specified by imm8, store the result in the lower element of dst using zeromask k (the element is zeroed
/// out when mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_roundscale_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscalesh, IMM8 = 0))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_roundscale_sh<const IMM8: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm_mask_roundscale_sh::<IMM8>(_mm_setzero_ph(), k, a, b)
}

/// Round the lower half-precision (16-bit) floating-point element in b to the number of fraction bits
/// specified by imm8, store the result in the lower element of dst, and copy the upper 7 packed elements
/// from a to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_roundscale_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscalesh, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_roundscale_round_sh<const IMM8: i32, const SAE: i32>(
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    _mm_mask_roundscale_round_sh::<IMM8, SAE>(_mm_undefined_ph(), 0xff, a, b)
}

/// Round the lower half-precision (16-bit) floating-point element in b to the number of fraction bits
/// specified by imm8, store the result in the lower element of dst using writemask k (the element is copied
/// from src when mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_roundscale_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscalesh, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(4, 5)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_roundscale_round_sh<const IMM8: i32, const SAE: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    vrndscalesh(a, b, src, k, IMM8, SAE)
}

/// Round the lower half-precision (16-bit) floating-point element in b to the number of fraction bits
/// specified by imm8, store the result in the lower element of dst using zeromask k (the element is zeroed
/// out when mask bit 0 is not set), and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_roundscale_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vrndscalesh, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(3, 4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_roundscale_round_sh<const IMM8: i32, const SAE: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    _mm_mask_roundscale_round_sh::<IMM8, SAE>(_mm_setzero_ph(), k, a, b)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_scalef_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vscalefph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_scalef_ph(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_scalef_ph(_mm_undefined_ph(), 0xff, a, b)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_scalef_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vscalefph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_scalef_ph(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    vscalefph_128(a, b, src, k)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_scalef_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vscalefph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_scalef_ph(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_scalef_ph(_mm_setzero_ph(), k, a, b)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_scalef_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vscalefph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_scalef_ph(a: __m256h, b: __m256h) -> __m256h {
    _mm256_mask_scalef_ph(_mm256_undefined_ph(), 0xffff, a, b)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_scalef_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vscalefph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_scalef_ph(src: __m256h, k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    vscalefph_256(a, b, src, k)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_scalef_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vscalefph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_scalef_ph(k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    _mm256_mask_scalef_ph(_mm256_setzero_ph(), k, a, b)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_scalef_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_scalef_ph(a: __m512h, b: __m512h) -> __m512h {
    _mm512_mask_scalef_ph(_mm512_undefined_ph(), 0xffffffff, a, b)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_scalef_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_scalef_ph(src: __m512h, k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    _mm512_mask_scalef_round_ph::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_scalef_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefph))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_scalef_ph(k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    _mm512_mask_scalef_ph(_mm512_setzero_ph(), k, a, b)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_scalef_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefph, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_scalef_round_ph<const ROUNDING: i32>(a: __m512h, b: __m512h) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_mask_scalef_round_ph::<ROUNDING>(_mm512_undefined_ph(), 0xffffffff, a, b)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_scalef_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefph, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_scalef_round_ph<const ROUNDING: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    vscalefph_512(a, b, src, k, ROUNDING)
}

/// Scale the packed half-precision (16-bit) floating-point elements in a using values from b, and store
/// the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_scalef_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefph, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_scalef_round_ph<const ROUNDING: i32>(
    k: __mmask32,
    a: __m512h,
    b: __m512h,
) -> __m512h {
    static_assert_rounding!(ROUNDING);
    _mm512_mask_scalef_round_ph::<ROUNDING>(_mm512_setzero_ph(), k, a, b)
}

/// Scale the packed single-precision (32-bit) floating-point elements in a using values from b, store
/// the result in the lower element of dst, and copy the upper 7 packed elements from a to the upper
/// elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_scalef_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_scalef_sh(a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_scalef_sh(_mm_undefined_ph(), 0xff, a, b)
}

/// Scale the packed single-precision (32-bit) floating-point elements in a using values from b, store
/// the result in the lower element of dst using writemask k (the element is copied from src when mask bit 0 is not set),
/// and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_scalef_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_scalef_sh(src: __m128h, k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_scalef_round_sh::<_MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Scale the packed single-precision (32-bit) floating-point elements in a using values from b, store
/// the result in the lower element of dst using zeromask k (the element is zeroed out when mask bit 0 is not set),
/// and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_scalef_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefsh))]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_scalef_sh(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    _mm_mask_scalef_sh(_mm_setzero_ph(), k, a, b)
}

/// Scale the packed single-precision (32-bit) floating-point elements in a using values from b, store
/// the result in the lower element of dst, and copy the upper 7 packed elements from a to the upper
/// elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_scalef_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_scalef_round_sh<const ROUNDING: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_scalef_round_sh::<ROUNDING>(_mm_undefined_ph(), 0xff, a, b)
}

/// Scale the packed single-precision (32-bit) floating-point elements in a using values from b, store
/// the result in the lower element of dst using writemask k (the element is copied from src when mask bit 0 is not set),
/// and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_scalef_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_scalef_round_sh<const ROUNDING: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    vscalefsh(a, b, src, k, ROUNDING)
}

/// Scale the packed single-precision (32-bit) floating-point elements in a using values from b, store
/// the result in the lower element of dst using zeromask k (the element is zeroed out when mask bit 0 is not set),
/// and copy the upper 7 packed elements from a to the upper elements of dst.
///
/// Rounding is done according to the rounding parameter, which can be one of:
///
///     (_MM_FROUND_TO_NEAREST_INT |_MM_FROUND_NO_EXC) // round to nearest, and suppress exceptions
///     (_MM_FROUND_TO_NEG_INF |_MM_FROUND_NO_EXC)     // round down, and suppress exceptions
///     (_MM_FROUND_TO_POS_INF |_MM_FROUND_NO_EXC)     // round up, and suppress exceptions
///     (_MM_FROUND_TO_ZERO |_MM_FROUND_NO_EXC)        // truncate, and suppress exceptions
///     _MM_FROUND_CUR_DIRECTION                       // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_scalef_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vscalefsh, ROUNDING = 8))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_scalef_round_sh<const ROUNDING: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_rounding!(ROUNDING);
    _mm_mask_scalef_round_sh::<ROUNDING>(_mm_setzero_ph(), k, a, b)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_reduce_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_reduce_ph<const IMM8: i32>(a: __m128h) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm_mask_reduce_ph::<IMM8>(_mm_undefined_ph(), 0xff, a)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst using writemask k (elements are copied
/// from src when the corresponding mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_reduce_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_reduce_ph<const IMM8: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    vreduceph_128(a, IMM8, src, k)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst using zeromask k (elements are zeroed
/// out when the corresponding mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_reduce_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_reduce_ph<const IMM8: i32>(k: __mmask8, a: __m128h) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm_mask_reduce_ph::<IMM8>(_mm_setzero_ph(), k, a)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_reduce_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_reduce_ph<const IMM8: i32>(a: __m256h) -> __m256h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm256_mask_reduce_ph::<IMM8>(_mm256_undefined_ph(), 0xffff, a)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst using writemask k (elements are copied
/// from src when the corresponding mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_reduce_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_reduce_ph<const IMM8: i32>(
    src: __m256h,
    k: __mmask16,
    a: __m256h,
) -> __m256h {
    static_assert_uimm_bits!(IMM8, 8);
    vreduceph_256(a, IMM8, src, k)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst using zeromask k (elements are zeroed
/// out when the corresponding mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_reduce_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_maskz_reduce_ph<const IMM8: i32>(k: __mmask16, a: __m256h) -> __m256h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm256_mask_reduce_ph::<IMM8>(_mm256_setzero_ph(), k, a)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_reduce_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_reduce_ph<const IMM8: i32>(a: __m512h) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm512_mask_reduce_ph::<IMM8>(_mm512_undefined_ph(), 0xffffffff, a)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst using writemask k (elements are copied
/// from src when the corresponding mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_reduce_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_reduce_ph<const IMM8: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm512_mask_reduce_round_ph::<IMM8, _MM_FROUND_CUR_DIRECTION>(src, k, a)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst using zeromask k (elements are zeroed
/// out when the corresponding mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_reduce_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_reduce_ph<const IMM8: i32>(k: __mmask32, a: __m512h) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm512_mask_reduce_ph::<IMM8>(_mm512_setzero_ph(), k, a)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_reduce_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(1, 2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_reduce_round_ph<const IMM8: i32, const SAE: i32>(a: __m512h) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    _mm512_mask_reduce_round_ph::<IMM8, SAE>(_mm512_undefined_ph(), 0xffffffff, a)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst using writemask k (elements are copied
/// from src when the corresponding mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_reduce_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(3, 4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_reduce_round_ph<const IMM8: i32, const SAE: i32>(
    src: __m512h,
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    vreduceph_512(a, IMM8, src, k, SAE)
}

/// Extract the reduced argument of packed half-precision (16-bit) floating-point elements in a by the
/// number of bits specified by imm8, and store the results in dst using zeromask k (elements are zeroed
/// out when the corresponding mask bit is not set).
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_reduce_round_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreduceph, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_maskz_reduce_round_ph<const IMM8: i32, const SAE: i32>(
    k: __mmask32,
    a: __m512h,
) -> __m512h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    _mm512_mask_reduce_round_ph::<IMM8, SAE>(_mm512_setzero_ph(), k, a)
}

/// Extract the reduced argument of the lower half-precision (16-bit) floating-point element in b by
/// the number of bits specified by imm8, store the result in the lower element of dst, and copy the
/// upper 7 packed elements from a to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_reduce_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreducesh, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_reduce_sh<const IMM8: i32>(a: __m128h, b: __m128h) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm_mask_reduce_sh::<IMM8>(_mm_undefined_ph(), 0xff, a, b)
}

/// Extract the reduced argument of the lower half-precision (16-bit) floating-point element in b by
/// the number of bits specified by imm8, store the result in the lower element of dst using writemask k
/// (the element is copied from src when mask bit 0 is not set), and copy the upper 7 packed elements from
/// a to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_reduce_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreducesh, IMM8 = 0))]
#[rustc_legacy_const_generics(4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_reduce_sh<const IMM8: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm_mask_reduce_round_sh::<IMM8, _MM_FROUND_CUR_DIRECTION>(src, k, a, b)
}

/// Extract the reduced argument of the lower half-precision (16-bit) floating-point element in b by
/// the number of bits specified by imm8, store the result in the lower element of dst using zeromask k
/// (the element is zeroed out when mask bit 0 is not set), and copy the upper 7 packed elements from a
/// to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_reduce_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreducesh, IMM8 = 0))]
#[rustc_legacy_const_generics(3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_reduce_sh<const IMM8: i32>(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    _mm_mask_reduce_sh::<IMM8>(_mm_setzero_ph(), k, a, b)
}

/// Extract the reduced argument of the lower half-precision (16-bit) floating-point element in b by
/// the number of bits specified by imm8, store the result in the lower element of dst, and copy the upper
/// 7 packed elements from a to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_reduce_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreducesh, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(2, 3)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_reduce_round_sh<const IMM8: i32, const SAE: i32>(
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    _mm_mask_reduce_round_sh::<IMM8, SAE>(_mm_undefined_ph(), 0xff, a, b)
}

/// Extract the reduced argument of the lower half-precision (16-bit) floating-point element in b by
/// the number of bits specified by imm8, store the result in the lower element of dst using writemask k
/// (the element is copied from src when mask bit 0 is not set), and copy the upper 7 packed elements from a
/// to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_reduce_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreducesh, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(4, 5)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_reduce_round_sh<const IMM8: i32, const SAE: i32>(
    src: __m128h,
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    vreducesh(a, b, src, k, IMM8, SAE)
}

/// Extract the reduced argument of the lower half-precision (16-bit) floating-point element in b by
/// the number of bits specified by imm8, store the result in the lower element of dst using zeromask k
/// (the element is zeroed out when mask bit 0 is not set), and copy the upper 7 packed elements from a
/// to the upper elements of dst.
///
/// Rounding is done according to the imm8 parameter, which can be one of:
///
///     _MM_FROUND_TO_NEAREST_INT // round to nearest
///     _MM_FROUND_TO_NEG_INF     // round down
///     _MM_FROUND_TO_POS_INF     // round up
///     _MM_FROUND_TO_ZERO        // truncate
///     _MM_FROUND_CUR_DIRECTION  // use MXCSR.RC; see _MM_SET_ROUNDING_MODE
///
/// Exceptions can be suppressed by passing _MM_FROUND_NO_EXC in the sae parameter.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maskz_reduce_round_sh)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vreducesh, IMM8 = 0, SAE = 8))]
#[rustc_legacy_const_generics(3, 4)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_maskz_reduce_round_sh<const IMM8: i32, const SAE: i32>(
    k: __mmask8,
    a: __m128h,
    b: __m128h,
) -> __m128h {
    static_assert_uimm_bits!(IMM8, 8);
    static_assert_sae!(SAE);
    _mm_mask_reduce_round_sh::<IMM8, SAE>(_mm_setzero_ph(), k, a, b)
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by addition. Returns the
/// sum of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_reduce_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_reduce_add_ph(a: __m128h) -> f16 {
    let b = simd_shuffle!(a, a, [4, 5, 6, 7, 0, 1, 2, 3]);
    let a = _mm_add_ph(a, b);
    let b = simd_shuffle!(a, a, [2, 3, 0, 1, 4, 5, 6, 7]);
    let a = _mm_add_ph(a, b);
    simd_extract::<_, f16>(a, 0) + simd_extract::<_, f16>(a, 1)
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by addition. Returns the
/// sum of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_reduce_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_reduce_add_ph(a: __m256h) -> f16 {
    let p = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);
    let q = simd_shuffle!(a, a, [8, 9, 10, 11, 12, 13, 14, 15]);
    _mm_reduce_add_ph(_mm_add_ph(p, q))
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by addition. Returns the
/// sum of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_reduce_add_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_reduce_add_ph(a: __m512h) -> f16 {
    let p = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let q = simd_shuffle!(
        a,
        a,
        [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
    );
    _mm256_reduce_add_ph(_mm256_add_ph(p, q))
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by multiplication. Returns
/// the product of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_reduce_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_reduce_mul_ph(a: __m128h) -> f16 {
    let b = simd_shuffle!(a, a, [4, 5, 6, 7, 0, 1, 2, 3]);
    let a = _mm_mul_ph(a, b);
    let b = simd_shuffle!(a, a, [2, 3, 0, 1, 4, 5, 6, 7]);
    let a = _mm_mul_ph(a, b);
    simd_extract::<_, f16>(a, 0) * simd_extract::<_, f16>(a, 1)
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by multiplication. Returns
/// the product of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_reduce_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_reduce_mul_ph(a: __m256h) -> f16 {
    let p = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);
    let q = simd_shuffle!(a, a, [8, 9, 10, 11, 12, 13, 14, 15]);
    _mm_reduce_mul_ph(_mm_mul_ph(p, q))
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by multiplication. Returns
/// the product of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_reduce_mul_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_reduce_mul_ph(a: __m512h) -> f16 {
    let p = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let q = simd_shuffle!(
        a,
        a,
        [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
    );
    _mm256_reduce_mul_ph(_mm256_mul_ph(p, q))
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by minimum. Returns the
/// minimum of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_reduce_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_reduce_min_ph(a: __m128h) -> f16 {
    let b = simd_shuffle!(a, a, [4, 5, 6, 7, 0, 1, 2, 3]);
    let a = _mm_min_ph(a, b);
    let b = simd_shuffle!(a, a, [2, 3, 0, 1, 4, 5, 6, 7]);
    let a = _mm_min_ph(a, b);
    let b = simd_shuffle!(a, a, [1, 0, 2, 3, 4, 5, 6, 7]);
    simd_extract!(_mm_min_sh(a, b), 0)
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by minimum. Returns the
/// minimum of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_reduce_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_reduce_min_ph(a: __m256h) -> f16 {
    let p = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);
    let q = simd_shuffle!(a, a, [8, 9, 10, 11, 12, 13, 14, 15]);
    _mm_reduce_min_ph(_mm_min_ph(p, q))
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by minimum. Returns the
/// minimum of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_reduce_min_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_reduce_min_ph(a: __m512h) -> f16 {
    let p = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let q = simd_shuffle!(
        a,
        a,
        [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
    );
    _mm256_reduce_min_ph(_mm256_min_ph(p, q))
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by maximum. Returns the
/// maximum of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_reduce_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_reduce_max_ph(a: __m128h) -> f16 {
    let b = simd_shuffle!(a, a, [4, 5, 6, 7, 0, 1, 2, 3]);
    let a = _mm_max_ph(a, b);
    let b = simd_shuffle!(a, a, [2, 3, 0, 1, 4, 5, 6, 7]);
    let a = _mm_max_ph(a, b);
    let b = simd_shuffle!(a, a, [1, 0, 2, 3, 4, 5, 6, 7]);
    simd_extract!(_mm_max_sh(a, b), 0)
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by maximum. Returns the
/// maximum of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_reduce_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_reduce_max_ph(a: __m256h) -> f16 {
    let p = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7]);
    let q = simd_shuffle!(a, a, [8, 9, 10, 11, 12, 13, 14, 15]);
    _mm_reduce_max_ph(_mm_max_ph(p, q))
}

/// Reduce the packed half-precision (16-bit) floating-point elements in a by maximum. Returns the
/// maximum of all elements in a.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_reduce_max_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_reduce_max_ph(a: __m512h) -> f16 {
    let p = simd_shuffle!(a, a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let q = simd_shuffle!(
        a,
        a,
        [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
    );
    _mm256_reduce_max_ph(_mm256_max_ph(p, q))
}

macro_rules! fpclass_asm {
    ($mask_type: ty, $reg: ident, $a: expr) => {{
        let dst: $mask_type;
        crate::arch::asm!(
            "vfpclassph {k}, {src}, {imm8}",
            k = lateout(kreg) dst,
            src = in($reg) $a,
            imm8 = const IMM8,
            options(pure, nomem, nostack)
        );
        dst
    }};
    ($mask_type: ty, $mask: expr, $reg: ident, $a: expr) => {{
        let dst: $mask_type;
        crate::arch::asm!(
            "vfpclassph {k} {{ {mask} }}, {src}, {imm8}",
            k = lateout(kreg) dst,
            mask = in(kreg) $mask,
            src = in($reg) $a,
            imm8 = const IMM8,
            options(pure, nomem, nostack)
        );
        dst
    }};
}

/// Test packed half-precision (16-bit) floating-point elements in a for special categories specified
/// by imm8, and store the results in mask vector k.
/// imm can be a combination of:
///
///     0x01 // QNaN
///     0x02 // Positive Zero
///     0x04 // Negative Zero
///     0x08 // Positive Infinity
///     0x10 // Negative Infinity
///     0x20 // Denormal
///     0x40 // Negative
///     0x80 // SNaN
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fpclass_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl,avx512f,sse")]
#[cfg_attr(test, assert_instr(vfpclassph, IMM8 = 0))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fpclass_ph_mask<const IMM8: i32>(a: __m128h) -> __mmask8 {
    static_assert_uimm_bits!(IMM8, 8);
    fpclass_asm!(__mmask8, xmm_reg, a)
}

/// Test packed half-precision (16-bit) floating-point elements in a for special categories specified
/// by imm8, and store the results in mask vector k using zeromask k (elements are zeroed out when the
/// corresponding mask bit is not set).
/// imm can be a combination of:
///
///     0x01 // QNaN
///     0x02 // Positive Zero
///     0x04 // Negative Zero
///     0x08 // Positive Infinity
///     0x10 // Negative Infinity
///     0x20 // Denormal
///     0x40 // Negative
///     0x80 // SNaN
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fpclass_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl,avx512f,sse")]
#[cfg_attr(test, assert_instr(vfpclassph, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fpclass_ph_mask<const IMM8: i32>(k1: __mmask8, a: __m128h) -> __mmask8 {
    static_assert_uimm_bits!(IMM8, 8);
    fpclass_asm!(__mmask8, k1, xmm_reg, a)
}

/// Test packed half-precision (16-bit) floating-point elements in a for special categories specified
/// by imm8, and store the results in mask vector k.
/// imm can be a combination of:
///
///     0x01 // QNaN
///     0x02 // Positive Zero
///     0x04 // Negative Zero
///     0x08 // Positive Infinity
///     0x10 // Negative Infinity
///     0x20 // Denormal
///     0x40 // Negative
///     0x80 // SNaN
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_fpclass_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl,avx512f,avx")]
#[cfg_attr(test, assert_instr(vfpclassph, IMM8 = 0))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_fpclass_ph_mask<const IMM8: i32>(a: __m256h) -> __mmask16 {
    static_assert_uimm_bits!(IMM8, 8);
    fpclass_asm!(__mmask16, ymm_reg, a)
}

/// Test packed half-precision (16-bit) floating-point elements in a for special categories specified
/// by imm8, and store the results in mask vector k using zeromask k (elements are zeroed out when the
/// corresponding mask bit is not set).
/// imm can be a combination of:
///
///     0x01 // QNaN
///     0x02 // Positive Zero
///     0x04 // Negative Zero
///     0x08 // Positive Infinity
///     0x10 // Negative Infinity
///     0x20 // Denormal
///     0x40 // Negative
///     0x80 // SNaN
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_fpclass_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl,avx512f,avx")]
#[cfg_attr(test, assert_instr(vfpclassph, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_fpclass_ph_mask<const IMM8: i32>(k1: __mmask16, a: __m256h) -> __mmask16 {
    static_assert_uimm_bits!(IMM8, 8);
    fpclass_asm!(__mmask16, k1, ymm_reg, a)
}

/// Test packed half-precision (16-bit) floating-point elements in a for special categories specified
/// by imm8, and store the results in mask vector k.
/// imm can be a combination of:
///
///     0x01 // QNaN
///     0x02 // Positive Zero
///     0x04 // Negative Zero
///     0x08 // Positive Infinity
///     0x10 // Negative Infinity
///     0x20 // Denormal
///     0x40 // Negative
///     0x80 // SNaN
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_fpclass_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512bw,avx512f")]
#[cfg_attr(test, assert_instr(vfpclassph, IMM8 = 0))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_fpclass_ph_mask<const IMM8: i32>(a: __m512h) -> __mmask32 {
    static_assert_uimm_bits!(IMM8, 8);
    fpclass_asm!(__mmask32, zmm_reg, a)
}

/// Test packed half-precision (16-bit) floating-point elements in a for special categories specified
/// by imm8, and store the results in mask vector k using zeromask k (elements are zeroed out when the
/// corresponding mask bit is not set).
/// imm can be a combination of:
///
///     0x01 // QNaN
///     0x02 // Positive Zero
///     0x04 // Negative Zero
///     0x08 // Positive Infinity
///     0x10 // Negative Infinity
///     0x20 // Denormal
///     0x40 // Negative
///     0x80 // SNaN
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_fpclass_ph_mask)
#[inline]
#[target_feature(enable = "avx512fp16,avx512bw,avx512f")]
#[cfg_attr(test, assert_instr(vfpclassph, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_fpclass_ph_mask<const IMM8: i32>(k1: __mmask32, a: __m512h) -> __mmask32 {
    static_assert_uimm_bits!(IMM8, 8);
    fpclass_asm!(__mmask32, k1, zmm_reg, a)
}

/// Test the lower half-precision (16-bit) floating-point element in a for special categories specified
/// by imm8, and store the result in mask vector k.
/// imm can be a combination of:
///
///     0x01 // QNaN
///     0x02 // Positive Zero
///     0x04 // Negative Zero
///     0x08 // Positive Infinity
///     0x10 // Negative Infinity
///     0x20 // Denormal
///     0x40 // Negative
///     0x80 // SNaN
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_fpclass_sh_mask)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfpclasssh, IMM8 = 0))]
#[rustc_legacy_const_generics(1)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_fpclass_sh_mask<const IMM8: i32>(a: __m128h) -> __mmask8 {
    _mm_mask_fpclass_sh_mask::<IMM8>(0xff, a)
}

/// Test the lower half-precision (16-bit) floating-point element in a for special categories specified
/// by imm8, and store the result in mask vector k using zeromask k (elements are zeroed out when the
/// corresponding mask bit is not set).
/// imm can be a combination of:
///
///     0x01 // QNaN
///     0x02 // Positive Zero
///     0x04 // Negative Zero
///     0x08 // Positive Infinity
///     0x10 // Negative Infinity
///     0x20 // Denormal
///     0x40 // Negative
///     0x80 // SNaN
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_fpclass_sh_mask)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[cfg_attr(test, assert_instr(vfpclasssh, IMM8 = 0))]
#[rustc_legacy_const_generics(2)]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_fpclass_sh_mask<const IMM8: i32>(k1: __mmask8, a: __m128h) -> __mmask8 {
    static_assert_uimm_bits!(IMM8, 8);
    vfpclasssh(a, IMM8, k1)
}

/// Blend packed half-precision (16-bit) floating-point elements from a and b using control mask k,
/// and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mask_blend_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_mask_blend_ph(k: __mmask8, a: __m128h, b: __m128h) -> __m128h {
    simd_select_bitmask(k, b, a)
}

/// Blend packed half-precision (16-bit) floating-point elements from a and b using control mask k,
/// and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_mask_blend_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_mask_blend_ph(k: __mmask16, a: __m256h, b: __m256h) -> __m256h {
    simd_select_bitmask(k, b, a)
}

/// Blend packed half-precision (16-bit) floating-point elements from a and b using control mask k,
/// and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_blend_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_mask_blend_ph(k: __mmask32, a: __m512h, b: __m512h) -> __m512h {
    simd_select_bitmask(k, b, a)
}

/// Shuffle half-precision (16-bit) floating-point elements in a and b using the corresponding selector
/// and index in idx, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_permutex2var_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_permutex2var_ph(a: __m128h, idx: __m128i, b: __m128h) -> __m128h {
    _mm_castsi128_ph(_mm_permutex2var_epi16(
        _mm_castph_si128(a),
        idx,
        _mm_castph_si128(b),
    ))
}

/// Shuffle half-precision (16-bit) floating-point elements in a and b using the corresponding selector
/// and index in idx, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_permutex2var_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_permutex2var_ph(a: __m256h, idx: __m256i, b: __m256h) -> __m256h {
    _mm256_castsi256_ph(_mm256_permutex2var_epi16(
        _mm256_castph_si256(a),
        idx,
        _mm256_castph_si256(b),
    ))
}

/// Shuffle half-precision (16-bit) floating-point elements in a and b using the corresponding selector
/// and index in idx, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_permutex2var_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_permutex2var_ph(a: __m512h, idx: __m512i, b: __m512h) -> __m512h {
    _mm512_castsi512_ph(_mm512_permutex2var_epi16(
        _mm512_castph_si512(a),
        idx,
        _mm512_castph_si512(b),
    ))
}

/// Shuffle half-precision (16-bit) floating-point elements in a using the corresponding index in idx,
/// and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_permutexvar_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm_permutexvar_ph(idx: __m128i, a: __m128h) -> __m128h {
    _mm_castsi128_ph(_mm_permutexvar_epi16(idx, _mm_castph_si128(a)))
}

/// Shuffle half-precision (16-bit) floating-point elements in a using the corresponding index in idx,
/// and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_permutexvar_ph)
#[inline]
#[target_feature(enable = "avx512fp16,avx512vl")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm256_permutexvar_ph(idx: __m256i, a: __m256h) -> __m256h {
    _mm256_castsi256_ph(_mm256_permutexvar_epi16(idx, _mm256_castph_si256(a)))
}

/// Shuffle half-precision (16-bit) floating-point elements in a using the corresponding index in idx,
/// and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_permutexvar_ph)
#[inline]
#[target_feature(enable = "avx512fp16")]
#[unstable(feature = "stdarch_x86_avx512_f16", issue = "127213")]
pub unsafe fn _mm512_permutexvar_ph(idx: __m512i, a: __m512h) -> __m512h {
    _mm512_castsi512_ph(_mm512_permutexvar_epi16(idx, _mm512_castph_si512(a)))
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.x86.avx512fp16.mask.cmp.sh"]
    fn vcmpsh(a: __m128h, b: __m128h, imm8: i32, mask: __mmask8, sae: i32) -> __mmask8;
    #[link_name = "llvm.x86.avx512fp16.vcomi.sh"]
    fn vcomish(a: __m128h, b: __m128h, imm8: i32, sae: i32) -> i32;

    #[link_name = "llvm.x86.avx512fp16.add.ph.512"]
    fn vaddph(a: __m512h, b: __m512h, rounding: i32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.sub.ph.512"]
    fn vsubph(a: __m512h, b: __m512h, rounding: i32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.mul.ph.512"]
    fn vmulph(a: __m512h, b: __m512h, rounding: i32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.div.ph.512"]
    fn vdivph(a: __m512h, b: __m512h, rounding: i32) -> __m512h;

    #[link_name = "llvm.x86.avx512fp16.mask.add.sh.round"]
    fn vaddsh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8, rounding: i32) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.mask.sub.sh.round"]
    fn vsubsh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8, rounding: i32) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.mask.mul.sh.round"]
    fn vmulsh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8, rounding: i32) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.mask.div.sh.round"]
    fn vdivsh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8, rounding: i32) -> __m128h;

    #[link_name = "llvm.x86.avx512fp16.mask.vfmul.cph.128"]
    fn vfmulcph_128(a: __m128, b: __m128, src: __m128, k: __mmask8) -> __m128;
    #[link_name = "llvm.x86.avx512fp16.mask.vfmul.cph.256"]
    fn vfmulcph_256(a: __m256, b: __m256, src: __m256, k: __mmask8) -> __m256;
    #[link_name = "llvm.x86.avx512fp16.mask.vfmul.cph.512"]
    fn vfmulcph_512(a: __m512, b: __m512, src: __m512, k: __mmask16, rounding: i32) -> __m512;
    #[link_name = "llvm.x86.avx512fp16.mask.vfmul.csh"]
    fn vfmulcsh(a: __m128, b: __m128, src: __m128, k: __mmask8, rounding: i32) -> __m128;

    #[link_name = "llvm.x86.avx512fp16.mask.vfcmul.cph.128"]
    fn vfcmulcph_128(a: __m128, b: __m128, src: __m128, k: __mmask8) -> __m128;
    #[link_name = "llvm.x86.avx512fp16.mask.vfcmul.cph.256"]
    fn vfcmulcph_256(a: __m256, b: __m256, src: __m256, k: __mmask8) -> __m256;
    #[link_name = "llvm.x86.avx512fp16.mask.vfcmul.cph.512"]
    fn vfcmulcph_512(a: __m512, b: __m512, src: __m512, k: __mmask16, rounding: i32) -> __m512;
    #[link_name = "llvm.x86.avx512fp16.mask.vfcmul.csh"]
    fn vfcmulcsh(a: __m128, b: __m128, src: __m128, k: __mmask8, rounding: i32) -> __m128;

    #[link_name = "llvm.x86.avx512fp16.mask.vfmadd.cph.128"]
    fn vfmaddcph_mask3_128(a: __m128, b: __m128, c: __m128, k: __mmask8) -> __m128;
    #[link_name = "llvm.x86.avx512fp16.maskz.vfmadd.cph.128"]
    fn vfmaddcph_maskz_128(a: __m128, b: __m128, c: __m128, k: __mmask8) -> __m128;
    #[link_name = "llvm.x86.avx512fp16.mask.vfmadd.cph.256"]
    fn vfmaddcph_mask3_256(a: __m256, b: __m256, c: __m256, k: __mmask8) -> __m256;
    #[link_name = "llvm.x86.avx512fp16.maskz.vfmadd.cph.256"]
    fn vfmaddcph_maskz_256(a: __m256, b: __m256, c: __m256, k: __mmask8) -> __m256;
    #[link_name = "llvm.x86.avx512fp16.mask.vfmadd.cph.512"]
    fn vfmaddcph_mask3_512(a: __m512, b: __m512, c: __m512, k: __mmask16, rounding: i32) -> __m512;
    #[link_name = "llvm.x86.avx512fp16.maskz.vfmadd.cph.512"]
    fn vfmaddcph_maskz_512(a: __m512, b: __m512, c: __m512, k: __mmask16, rounding: i32) -> __m512;
    #[link_name = "llvm.x86.avx512fp16.mask.vfmadd.csh"]
    fn vfmaddcsh_mask(a: __m128, b: __m128, c: __m128, k: __mmask8, rounding: i32) -> __m128;
    #[link_name = "llvm.x86.avx512fp16.maskz.vfmadd.csh"]
    fn vfmaddcsh_maskz(a: __m128, b: __m128, c: __m128, k: __mmask8, rounding: i32) -> __m128;

    #[link_name = "llvm.x86.avx512fp16.mask.vfcmadd.cph.128"]
    fn vfcmaddcph_mask3_128(a: __m128, b: __m128, c: __m128, k: __mmask8) -> __m128;
    #[link_name = "llvm.x86.avx512fp16.maskz.vfcmadd.cph.128"]
    fn vfcmaddcph_maskz_128(a: __m128, b: __m128, c: __m128, k: __mmask8) -> __m128;
    #[link_name = "llvm.x86.avx512fp16.mask.vfcmadd.cph.256"]
    fn vfcmaddcph_mask3_256(a: __m256, b: __m256, c: __m256, k: __mmask8) -> __m256;
    #[link_name = "llvm.x86.avx512fp16.maskz.vfcmadd.cph.256"]
    fn vfcmaddcph_maskz_256(a: __m256, b: __m256, c: __m256, k: __mmask8) -> __m256;
    #[link_name = "llvm.x86.avx512fp16.mask.vfcmadd.cph.512"]
    fn vfcmaddcph_mask3_512(a: __m512, b: __m512, c: __m512, k: __mmask16, rounding: i32)
        -> __m512;
    #[link_name = "llvm.x86.avx512fp16.maskz.vfcmadd.cph.512"]
    fn vfcmaddcph_maskz_512(a: __m512, b: __m512, c: __m512, k: __mmask16, rounding: i32)
        -> __m512;
    #[link_name = "llvm.x86.avx512fp16.mask.vfcmadd.csh"]
    fn vfcmaddcsh_mask(a: __m128, b: __m128, c: __m128, k: __mmask8, rounding: i32) -> __m128;
    #[link_name = "llvm.x86.avx512fp16.maskz.vfcmadd.csh"]
    fn vfcmaddcsh_maskz(a: __m128, b: __m128, c: __m128, k: __mmask8, rounding: i32) -> __m128;

    #[link_name = "llvm.x86.avx512fp16.vfmadd.ph.512"]
    fn vfmaddph_512(a: __m512h, b: __m512h, c: __m512h, rounding: i32) -> __m512h;
    #[link_name = "llvm.fma.f16"]
    fn fmaf16(a: f16, b: f16, c: f16) -> f16; // TODO: use `crate::intrinsics::fmaf16` when it's available
    #[link_name = "llvm.x86.avx512fp16.vfmadd.f16"]
    fn vfmaddsh(a: f16, b: f16, c: f16, rounding: i32) -> f16;

    #[link_name = "llvm.x86.avx512fp16.vfmaddsub.ph.128"]
    fn vfmaddsubph_128(a: __m128h, b: __m128h, c: __m128h) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.vfmaddsub.ph.256"]
    fn vfmaddsubph_256(a: __m256h, b: __m256h, c: __m256h) -> __m256h;
    #[link_name = "llvm.x86.avx512fp16.vfmaddsub.ph.512"]
    fn vfmaddsubph_512(a: __m512h, b: __m512h, c: __m512h, rounding: i32) -> __m512h;

    #[link_name = "llvm.x86.avx512fp16.mask.rcp.ph.128"]
    fn vrcpph_128(a: __m128h, src: __m128h, k: __mmask8) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.mask.rcp.ph.256"]
    fn vrcpph_256(a: __m256h, src: __m256h, k: __mmask16) -> __m256h;
    #[link_name = "llvm.x86.avx512fp16.mask.rcp.ph.512"]
    fn vrcpph_512(a: __m512h, src: __m512h, k: __mmask32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.mask.rcp.sh"]
    fn vrcpsh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8) -> __m128h;

    #[link_name = "llvm.x86.avx512fp16.mask.rsqrt.ph.128"]
    fn vrsqrtph_128(a: __m128h, src: __m128h, k: __mmask8) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.mask.rsqrt.ph.256"]
    fn vrsqrtph_256(a: __m256h, src: __m256h, k: __mmask16) -> __m256h;
    #[link_name = "llvm.x86.avx512fp16.mask.rsqrt.ph.512"]
    fn vrsqrtph_512(a: __m512h, src: __m512h, k: __mmask32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.mask.rsqrt.sh"]
    fn vrsqrtsh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8) -> __m128h;

    #[link_name = "llvm.x86.avx512fp16.sqrt.ph.512"]
    fn vsqrtph_512(a: __m512h, rounding: i32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.mask.sqrt.sh"]
    fn vsqrtsh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8, rounding: i32) -> __m128h;

    #[link_name = "llvm.x86.avx512fp16.max.ph.128"]
    fn vmaxph_128(a: __m128h, b: __m128h) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.max.ph.256"]
    fn vmaxph_256(a: __m256h, b: __m256h) -> __m256h;
    #[link_name = "llvm.x86.avx512fp16.max.ph.512"]
    fn vmaxph_512(a: __m512h, b: __m512h, sae: i32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.mask.max.sh.round"]
    fn vmaxsh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8, sae: i32) -> __m128h;

    #[link_name = "llvm.x86.avx512fp16.min.ph.128"]
    fn vminph_128(a: __m128h, b: __m128h) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.min.ph.256"]
    fn vminph_256(a: __m256h, b: __m256h) -> __m256h;
    #[link_name = "llvm.x86.avx512fp16.min.ph.512"]
    fn vminph_512(a: __m512h, b: __m512h, sae: i32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.mask.min.sh.round"]
    fn vminsh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8, sae: i32) -> __m128h;

    #[link_name = "llvm.x86.avx512fp16.mask.getexp.ph.128"]
    fn vgetexpph_128(a: __m128h, src: __m128h, k: __mmask8) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.mask.getexp.ph.256"]
    fn vgetexpph_256(a: __m256h, src: __m256h, k: __mmask16) -> __m256h;
    #[link_name = "llvm.x86.avx512fp16.mask.getexp.ph.512"]
    fn vgetexpph_512(a: __m512h, src: __m512h, k: __mmask32, sae: i32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.mask.getexp.sh"]
    fn vgetexpsh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8, sae: i32) -> __m128h;

    #[link_name = "llvm.x86.avx512fp16.mask.getmant.ph.128"]
    fn vgetmantph_128(a: __m128h, imm8: i32, src: __m128h, k: __mmask8) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.mask.getmant.ph.256"]
    fn vgetmantph_256(a: __m256h, imm8: i32, src: __m256h, k: __mmask16) -> __m256h;
    #[link_name = "llvm.x86.avx512fp16.mask.getmant.ph.512"]
    fn vgetmantph_512(a: __m512h, imm8: i32, src: __m512h, k: __mmask32, sae: i32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.mask.getmant.sh"]
    fn vgetmantsh(
        a: __m128h,
        b: __m128h,
        imm8: i32,
        src: __m128h,
        k: __mmask8,
        sae: i32,
    ) -> __m128h;

    #[link_name = "llvm.x86.avx512fp16.mask.rndscale.ph.128"]
    fn vrndscaleph_128(a: __m128h, imm8: i32, src: __m128h, k: __mmask8) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.mask.rndscale.ph.256"]
    fn vrndscaleph_256(a: __m256h, imm8: i32, src: __m256h, k: __mmask16) -> __m256h;
    #[link_name = "llvm.x86.avx512fp16.mask.rndscale.ph.512"]
    fn vrndscaleph_512(a: __m512h, imm8: i32, src: __m512h, k: __mmask32, sae: i32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.mask.rndscale.sh"]
    fn vrndscalesh(
        a: __m128h,
        b: __m128h,
        src: __m128h,
        k: __mmask8,
        imm8: i32,
        sae: i32,
    ) -> __m128h;

    #[link_name = "llvm.x86.avx512fp16.mask.scalef.ph.128"]
    fn vscalefph_128(a: __m128h, b: __m128h, src: __m128h, k: __mmask8) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.mask.scalef.ph.256"]
    fn vscalefph_256(a: __m256h, b: __m256h, src: __m256h, k: __mmask16) -> __m256h;
    #[link_name = "llvm.x86.avx512fp16.mask.scalef.ph.512"]
    fn vscalefph_512(a: __m512h, b: __m512h, src: __m512h, k: __mmask32, rounding: i32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.mask.scalef.sh"]
    fn vscalefsh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8, rounding: i32) -> __m128h;

    #[link_name = "llvm.x86.avx512fp16.mask.reduce.ph.128"]
    fn vreduceph_128(a: __m128h, imm8: i32, src: __m128h, k: __mmask8) -> __m128h;
    #[link_name = "llvm.x86.avx512fp16.mask.reduce.ph.256"]
    fn vreduceph_256(a: __m256h, imm8: i32, src: __m256h, k: __mmask16) -> __m256h;
    #[link_name = "llvm.x86.avx512fp16.mask.reduce.ph.512"]
    fn vreduceph_512(a: __m512h, imm8: i32, src: __m512h, k: __mmask32, sae: i32) -> __m512h;
    #[link_name = "llvm.x86.avx512fp16.mask.reduce.sh"]
    fn vreducesh(a: __m128h, b: __m128h, src: __m128h, k: __mmask8, imm8: i32, sae: i32)
        -> __m128h;

    #[link_name = "llvm.x86.avx512fp16.mask.fpclass.sh"]
    fn vfpclasssh(a: __m128h, imm8: i32, k: __mmask8) -> __mmask8;
}

#[cfg(test)]
mod tests {
    use crate::core_arch::x86::*;
    use crate::mem::transmute;
    use crate::ptr::{addr_of, addr_of_mut};
    use stdarch_test::simd_test;

    #[target_feature(enable = "avx512fp16")]
    unsafe fn _mm_set1_pch(re: f16, im: f16) -> __m128h {
        _mm_setr_ph(re, im, re, im, re, im, re, im)
    }

    #[target_feature(enable = "avx512fp16")]
    unsafe fn _mm256_set1_pch(re: f16, im: f16) -> __m256h {
        _mm256_setr_ph(
            re, im, re, im, re, im, re, im, re, im, re, im, re, im, re, im,
        )
    }

    #[target_feature(enable = "avx512fp16")]
    unsafe fn _mm512_set1_pch(re: f16, im: f16) -> __m512h {
        _mm512_setr_ph(
            re, im, re, im, re, im, re, im, re, im, re, im, re, im, re, im, re, im, re, im, re, im,
            re, im, re, im, re, im, re, im, re, im,
        )
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_set_ph() {
        let r = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let e = _mm_setr_ph(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_set_ph() {
        let r = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let e = _mm256_setr_ph(
            16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_set_ph() {
        let r = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let e = _mm512_setr_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_set_sh() {
        let r = _mm_set_sh(1.0);
        let e = _mm_set_ph(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_set1_ph() {
        let r = _mm_set1_ph(1.0);
        let e = _mm_set_ph(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_set1_ph() {
        let r = _mm256_set1_ph(1.0);
        let e = _mm256_set_ph(
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_set1_ph() {
        let r = _mm512_set1_ph(1.0);
        let e = _mm512_set_ph(
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_setr_ph() {
        let r = _mm_setr_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let e = _mm_set_ph(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_setr_ph() {
        let r = _mm256_setr_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let e = _mm256_set_ph(
            16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_setr_ph() {
        let r = _mm512_setr_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let e = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_setzero_ph() {
        let r = _mm_setzero_ph();
        let e = _mm_set1_ph(0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_setzero_ph() {
        let r = _mm256_setzero_ph();
        let e = _mm256_set1_ph(0.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_setzero_ph() {
        let r = _mm512_setzero_ph();
        let e = _mm512_set1_ph(0.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_castsi128_ph() {
        let a = _mm_set1_epi16(0x3c00);
        let r = _mm_castsi128_ph(a);
        let e = _mm_set1_ph(1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_castsi256_ph() {
        let a = _mm256_set1_epi16(0x3c00);
        let r = _mm256_castsi256_ph(a);
        let e = _mm256_set1_ph(1.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_castsi512_ph() {
        let a = _mm512_set1_epi16(0x3c00);
        let r = _mm512_castsi512_ph(a);
        let e = _mm512_set1_ph(1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_castph_si128() {
        let a = _mm_set1_ph(1.0);
        let r = _mm_castph_si128(a);
        let e = _mm_set1_epi16(0x3c00);
        assert_eq_m128i(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_castph_si256() {
        let a = _mm256_set1_ph(1.0);
        let r = _mm256_castph_si256(a);
        let e = _mm256_set1_epi16(0x3c00);
        assert_eq_m256i(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_castph_si512() {
        let a = _mm512_set1_ph(1.0);
        let r = _mm512_castph_si512(a);
        let e = _mm512_set1_epi16(0x3c00);
        assert_eq_m512i(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_castps_ph() {
        let a = _mm_castsi128_ps(_mm_set1_epi16(0x3c00));
        let r = _mm_castps_ph(a);
        let e = _mm_set1_ph(1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_castps_ph() {
        let a = _mm256_castsi256_ps(_mm256_set1_epi16(0x3c00));
        let r = _mm256_castps_ph(a);
        let e = _mm256_set1_ph(1.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_castps_ph() {
        let a = _mm512_castsi512_ps(_mm512_set1_epi16(0x3c00));
        let r = _mm512_castps_ph(a);
        let e = _mm512_set1_ph(1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_castph_ps() {
        let a = _mm_castsi128_ph(_mm_set1_epi32(0x3f800000));
        let r = _mm_castph_ps(a);
        let e = _mm_set1_ps(1.0);
        assert_eq_m128(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_castph_ps() {
        let a = _mm256_castsi256_ph(_mm256_set1_epi32(0x3f800000));
        let r = _mm256_castph_ps(a);
        let e = _mm256_set1_ps(1.0);
        assert_eq_m256(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_castph_ps() {
        let a = _mm512_castsi512_ph(_mm512_set1_epi32(0x3f800000));
        let r = _mm512_castph_ps(a);
        let e = _mm512_set1_ps(1.0);
        assert_eq_m512(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_castpd_ph() {
        let a = _mm_castsi128_pd(_mm_set1_epi16(0x3c00));
        let r = _mm_castpd_ph(a);
        let e = _mm_set1_ph(1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_castpd_ph() {
        let a = _mm256_castsi256_pd(_mm256_set1_epi16(0x3c00));
        let r = _mm256_castpd_ph(a);
        let e = _mm256_set1_ph(1.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_castpd_ph() {
        let a = _mm512_castsi512_pd(_mm512_set1_epi16(0x3c00));
        let r = _mm512_castpd_ph(a);
        let e = _mm512_set1_ph(1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_castph_pd() {
        let a = _mm_castsi128_ph(_mm_set1_epi64x(0x3ff0000000000000));
        let r = _mm_castph_pd(a);
        let e = _mm_set1_pd(1.0);
        assert_eq_m128d(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_castph_pd() {
        let a = _mm256_castsi256_ph(_mm256_set1_epi64x(0x3ff0000000000000));
        let r = _mm256_castph_pd(a);
        let e = _mm256_set1_pd(1.0);
        assert_eq_m256d(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_castph_pd() {
        let a = _mm512_castsi512_ph(_mm512_set1_epi64(0x3ff0000000000000));
        let r = _mm512_castph_pd(a);
        let e = _mm512_set1_pd(1.0);
        assert_eq_m512d(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_castph256_ph128() {
        let a = _mm256_setr_ph(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        );
        let r = _mm256_castph256_ph128(a);
        let e = _mm_setr_ph(1., 2., 3., 4., 5., 6., 7., 8.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_castph512_ph128() {
        let a = _mm512_setr_ph(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19.,
            20., 21., 22., 23., 24., 25., 26., 27., 28., 29., 30., 31., 32.,
        );
        let r = _mm512_castph512_ph128(a);
        let e = _mm_setr_ph(1., 2., 3., 4., 5., 6., 7., 8.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_castph512_ph256() {
        let a = _mm512_setr_ph(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19.,
            20., 21., 22., 23., 24., 25., 26., 27., 28., 29., 30., 31., 32.,
        );
        let r = _mm512_castph512_ph256(a);
        let e = _mm256_setr_ph(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_castph128_ph256() {
        let a = _mm_setr_ph(1., 2., 3., 4., 5., 6., 7., 8.);
        let r = _mm256_castph128_ph256(a);
        assert_eq_m128h(_mm256_castph256_ph128(r), a);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_castph128_ph512() {
        let a = _mm_setr_ph(1., 2., 3., 4., 5., 6., 7., 8.);
        let r = _mm512_castph128_ph512(a);
        assert_eq_m128h(_mm512_castph512_ph128(r), a);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_castph256_ph512() {
        let a = _mm256_setr_ph(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        );
        let r = _mm512_castph256_ph512(a);
        assert_eq_m256h(_mm512_castph512_ph256(r), a);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm256_zextph128_ph256() {
        let a = _mm_setr_ph(1., 2., 3., 4., 5., 6., 7., 8.);
        let r = _mm256_zextph128_ph256(a);
        let e = _mm256_setr_ph(
            1., 2., 3., 4., 5., 6., 7., 8., 0., 0., 0., 0., 0., 0., 0., 0.,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_zextph128_ph512() {
        let a = _mm_setr_ph(1., 2., 3., 4., 5., 6., 7., 8.);
        let r = _mm512_zextph128_ph512(a);
        let e = _mm512_setr_ph(
            1., 2., 3., 4., 5., 6., 7., 8., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_zextph256_ph512() {
        let a = _mm256_setr_ph(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        );
        let r = _mm512_zextph256_ph512(a);
        let e = _mm512_setr_ph(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 0., 0., 0., 0.,
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_cmp_ph_mask() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(1.0, 2.0, 3.0, 4.0, -5.0, -6.0, -7.0, -8.0);
        let r = _mm_cmp_ph_mask::<_CMP_EQ_OQ>(a, b);
        assert_eq!(r, 0b11110000);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_cmp_ph_mask() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(1.0, 2.0, 3.0, 4.0, -5.0, -6.0, -7.0, -8.0);
        let r = _mm_mask_cmp_ph_mask::<_CMP_EQ_OQ>(0b01010101, a, b);
        assert_eq!(r, 0b01010000);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_cmp_ph_mask() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, -5.0, -6.0, -7.0, -8.0, 9.0, 10.0, 11.0, 12.0, -13.0, -14.0, -15.0,
            -16.0,
        );
        let r = _mm256_cmp_ph_mask::<_CMP_EQ_OQ>(a, b);
        assert_eq!(r, 0b1111000011110000);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_cmp_ph_mask() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, -5.0, -6.0, -7.0, -8.0, 9.0, 10.0, 11.0, 12.0, -13.0, -14.0, -15.0,
            -16.0,
        );
        let r = _mm256_mask_cmp_ph_mask::<_CMP_EQ_OQ>(0b0101010101010101, a, b);
        assert_eq!(r, 0b0101000001010000);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_cmp_ph_mask() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, -5.0, -6.0, -7.0, -8.0, 9.0, 10.0, 11.0, 12.0, -13.0, -14.0, -15.0,
            -16.0, 17.0, 18.0, 19.0, 20.0, -21.0, -22.0, -23.0, -24.0, 25.0, 26.0, 27.0, 28.0,
            -29.0, -30.0, -31.0, -32.0,
        );
        let r = _mm512_cmp_ph_mask::<_CMP_EQ_OQ>(a, b);
        assert_eq!(r, 0b11110000111100001111000011110000);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_cmp_ph_mask() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, -5.0, -6.0, -7.0, -8.0, 9.0, 10.0, 11.0, 12.0, -13.0, -14.0, -15.0,
            -16.0, 17.0, 18.0, 19.0, 20.0, -21.0, -22.0, -23.0, -24.0, 25.0, 26.0, 27.0, 28.0,
            -29.0, -30.0, -31.0, -32.0,
        );
        let r = _mm512_mask_cmp_ph_mask::<_CMP_EQ_OQ>(0b01010101010101010101010101010101, a, b);
        assert_eq!(r, 0b01010000010100000101000001010000);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_cmp_round_sh_mask() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_cmp_round_sh_mask::<_CMP_EQ_OQ, _MM_FROUND_NO_EXC>(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_cmp_round_sh_mask() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_mask_cmp_round_sh_mask::<_CMP_EQ_OQ, _MM_FROUND_NO_EXC>(0, a, b);
        assert_eq!(r, 0);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_cmp_sh_mask() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_cmp_sh_mask::<_CMP_EQ_OQ>(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_cmp_sh_mask() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_mask_cmp_sh_mask::<_CMP_EQ_OQ>(0, a, b);
        assert_eq!(r, 0);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_comi_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_comi_round_sh::<_CMP_EQ_OQ, _MM_FROUND_NO_EXC>(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_comi_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_comi_sh::<_CMP_EQ_OQ>(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_comieq_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_comieq_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_comige_sh() {
        let a = _mm_set_sh(2.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_comige_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_comigt_sh() {
        let a = _mm_set_sh(2.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_comigt_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_comile_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_comile_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_comilt_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_comilt_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_comineq_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_comineq_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_ucomieq_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_ucomieq_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_ucomige_sh() {
        let a = _mm_set_sh(2.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_ucomige_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_ucomigt_sh() {
        let a = _mm_set_sh(2.0);
        let b = _mm_set_sh(1.0);
        let r = _mm_ucomigt_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_ucomile_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_ucomile_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_ucomilt_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_ucomilt_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_ucomineq_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_ucomineq_sh(a, b);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_load_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_load_ph(addr_of!(a).cast());
        assert_eq_m128h(a, b);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_load_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_load_ph(addr_of!(a).cast());
        assert_eq_m256h(a, b);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_load_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_load_ph(addr_of!(a).cast());
        assert_eq_m512h(a, b);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_load_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_load_sh(addr_of!(a).cast());
        assert_eq_m128h(a, b);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_load_sh() {
        let a = _mm_set_sh(1.0);
        let src = _mm_set_sh(2.);
        let b = _mm_mask_load_sh(src, 1, addr_of!(a).cast());
        assert_eq_m128h(a, b);
        let b = _mm_mask_load_sh(src, 0, addr_of!(a).cast());
        assert_eq_m128h(src, b);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_load_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_maskz_load_sh(1, addr_of!(a).cast());
        assert_eq_m128h(a, b);
        let b = _mm_maskz_load_sh(0, addr_of!(a).cast());
        assert_eq_m128h(_mm_setzero_ph(), b);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_loadu_ph() {
        let array = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let r = _mm_loadu_ph(array.as_ptr());
        let e = _mm_setr_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_loadu_ph() {
        let array = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ];
        let r = _mm256_loadu_ph(array.as_ptr());
        let e = _mm256_setr_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_loadu_ph() {
        let array = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        ];
        let r = _mm512_loadu_ph(array.as_ptr());
        let e = _mm512_setr_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_move_sh() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_sh(9.0);
        let r = _mm_move_sh(a, b);
        let e = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_move_sh() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_sh(9.0);
        let src = _mm_set_sh(10.0);
        let r = _mm_mask_move_sh(src, 0, a, b);
        let e = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 10.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_move_sh() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_sh(9.0);
        let r = _mm_maskz_move_sh(0, a, b);
        let e = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_store_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let mut b = _mm_setzero_ph();
        _mm_store_ph(addr_of_mut!(b).cast(), a);
        assert_eq_m128h(a, b);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_store_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let mut b = _mm256_setzero_ph();
        _mm256_store_ph(addr_of_mut!(b).cast(), a);
        assert_eq_m256h(a, b);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_store_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let mut b = _mm512_setzero_ph();
        _mm512_store_ph(addr_of_mut!(b).cast(), a);
        assert_eq_m512h(a, b);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_store_sh() {
        let a = _mm_set_sh(1.0);
        let mut b = _mm_setzero_ph();
        _mm_store_sh(addr_of_mut!(b).cast(), a);
        assert_eq_m128h(a, b);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_store_sh() {
        let a = _mm_set_sh(1.0);
        let mut b = _mm_setzero_ph();
        _mm_mask_store_sh(addr_of_mut!(b).cast(), 0, a);
        assert_eq_m128h(_mm_setzero_ph(), b);
        _mm_mask_store_sh(addr_of_mut!(b).cast(), 1, a);
        assert_eq_m128h(a, b);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_storeu_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let mut array = [0.0; 8];
        _mm_storeu_ph(array.as_mut_ptr(), a);
        assert_eq_m128h(a, _mm_loadu_ph(array.as_ptr()));
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_storeu_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let mut array = [0.0; 16];
        _mm256_storeu_ph(array.as_mut_ptr(), a);
        assert_eq_m256h(a, _mm256_loadu_ph(array.as_ptr()));
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_storeu_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let mut array = [0.0; 32];
        _mm512_storeu_ph(array.as_mut_ptr(), a);
        assert_eq_m512h(a, _mm512_loadu_ph(array.as_ptr()));
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_add_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let r = _mm_add_ph(a, b);
        let e = _mm_set1_ph(9.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_add_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let src = _mm_set_ph(10., 11., 12., 13., 14., 15., 16., 17.);
        let r = _mm_mask_add_ph(src, 0b01010101, a, b);
        let e = _mm_set_ph(10., 9., 12., 9., 14., 9., 16., 9.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_add_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let r = _mm_maskz_add_ph(0b01010101, a, b);
        let e = _mm_set_ph(0., 9., 0., 9., 0., 9., 0., 9.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_add_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        let r = _mm256_add_ph(a, b);
        let e = _mm256_set1_ph(17.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_add_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        let src = _mm256_set_ph(
            18., 19., 20., 21., 22., 23., 24., 25., 26., 27., 28., 29., 30., 31., 32., 33.,
        );
        let r = _mm256_mask_add_ph(src, 0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            18., 17., 20., 17., 22., 17., 24., 17., 26., 17., 28., 17., 30., 17., 32., 17.,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_add_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        let r = _mm256_maskz_add_ph(0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            0., 17., 0., 17., 0., 17., 0., 17., 0., 17., 0., 17., 0., 17., 0., 17.,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_add_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_add_ph(a, b);
        let e = _mm512_set1_ph(33.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_add_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let src = _mm512_set_ph(
            34., 35., 36., 37., 38., 39., 40., 41., 42., 43., 44., 45., 46., 47., 48., 49., 50.,
            51., 52., 53., 54., 55., 56., 57., 58., 59., 60., 61., 62., 63., 64., 65.,
        );
        let r = _mm512_mask_add_ph(src, 0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            34., 33., 36., 33., 38., 33., 40., 33., 42., 33., 44., 33., 46., 33., 48., 33., 50.,
            33., 52., 33., 54., 33., 56., 33., 58., 33., 60., 33., 62., 33., 64., 33.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_add_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_maskz_add_ph(0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0.,
            33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_add_round_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_add_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm512_set1_ph(33.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_add_round_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let src = _mm512_set_ph(
            34., 35., 36., 37., 38., 39., 40., 41., 42., 43., 44., 45., 46., 47., 48., 49., 50.,
            51., 52., 53., 54., 55., 56., 57., 58., 59., 60., 61., 62., 63., 64., 65.,
        );
        let r = _mm512_mask_add_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            34., 33., 36., 33., 38., 33., 40., 33., 42., 33., 44., 33., 46., 33., 48., 33., 50.,
            33., 52., 33., 54., 33., 56., 33., 58., 33., 60., 33., 62., 33., 64., 33.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_add_round_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_maskz_add_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0.,
            33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33., 0., 33.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_add_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_add_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_set_sh(3.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_add_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let src = _mm_set_sh(4.0);
        let r = _mm_mask_add_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_set_sh(4.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_add_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 1, a, b,
        );
        let e = _mm_set_sh(3.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_add_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r =
            _mm_maskz_add_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_set_sh(0.0);
        assert_eq_m128h(r, e);
        let r =
            _mm_maskz_add_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(1, a, b);
        let e = _mm_set_sh(3.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_add_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_add_sh(a, b);
        let e = _mm_set_sh(3.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_add_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let src = _mm_set_sh(4.0);
        let r = _mm_mask_add_sh(src, 0, a, b);
        let e = _mm_set_sh(4.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_add_sh(src, 1, a, b);
        let e = _mm_set_sh(3.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_add_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_maskz_add_sh(0, a, b);
        let e = _mm_set_sh(0.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_add_sh(1, a, b);
        let e = _mm_set_sh(3.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_sub_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let r = _mm_sub_ph(a, b);
        let e = _mm_set_ph(-7.0, -5.0, -3.0, -1.0, 1.0, 3.0, 5.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_sub_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let src = _mm_set_ph(10., 11., 12., 13., 14., 15., 16., 17.);
        let r = _mm_mask_sub_ph(src, 0b01010101, a, b);
        let e = _mm_set_ph(10., -5., 12., -1., 14., 3., 16., 7.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_sub_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let r = _mm_maskz_sub_ph(0b01010101, a, b);
        let e = _mm_set_ph(0., -5., 0., -1., 0., 3., 0., 7.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_sub_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        let r = _mm256_sub_ph(a, b);
        let e = _mm256_set_ph(
            -15.0, -13.0, -11.0, -9.0, -7.0, -5.0, -3.0, -1.0, 1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0,
            15.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_sub_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        let src = _mm256_set_ph(
            18., 19., 20., 21., 22., 23., 24., 25., 26., 27., 28., 29., 30., 31., 32., 33.,
        );
        let r = _mm256_mask_sub_ph(src, 0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            18., -13., 20., -9., 22., -5., 24., -1., 26., 3., 28., 7., 30., 11., 32., 15.,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_sub_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        let r = _mm256_maskz_sub_ph(0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            0., -13., 0., -9., 0., -5., 0., -1., 0., 3., 0., 7., 0., 11., 0., 15.,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_sub_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_sub_ph(a, b);
        let e = _mm512_set_ph(
            -31.0, -29.0, -27.0, -25.0, -23.0, -21.0, -19.0, -17.0, -15.0, -13.0, -11.0, -9.0,
            -7.0, -5.0, -3.0, -1.0, 1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0, 19.0, 21.0,
            23.0, 25.0, 27.0, 29.0, 31.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_sub_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let src = _mm512_set_ph(
            34., 35., 36., 37., 38., 39., 40., 41., 42., 43., 44., 45., 46., 47., 48., 49., 50.,
            51., 52., 53., 54., 55., 56., 57., 58., 59., 60., 61., 62., 63., 64., 65.,
        );
        let r = _mm512_mask_sub_ph(src, 0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            34., -29., 36., -25., 38., -21., 40., -17., 42., -13., 44., -9., 46., -5., 48., -1.,
            50., 3., 52., 7., 54., 11., 56., 15., 58., 19., 60., 23., 62., 27., 64., 31.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_sub_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_maskz_sub_ph(0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            0., -29., 0., -25., 0., -21., 0., -17., 0., -13., 0., -9., 0., -5., 0., -1., 0., 3.,
            0., 7., 0., 11., 0., 15., 0., 19., 0., 23., 0., 27., 0., 31.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_sub_round_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_sub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm512_set_ph(
            -31.0, -29.0, -27.0, -25.0, -23.0, -21.0, -19.0, -17.0, -15.0, -13.0, -11.0, -9.0,
            -7.0, -5.0, -3.0, -1.0, 1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0, 19.0, 21.0,
            23.0, 25.0, 27.0, 29.0, 31.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_sub_round_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let src = _mm512_set_ph(
            34., 35., 36., 37., 38., 39., 40., 41., 42., 43., 44., 45., 46., 47., 48., 49., 50.,
            51., 52., 53., 54., 55., 56., 57., 58., 59., 60., 61., 62., 63., 64., 65.,
        );
        let r = _mm512_mask_sub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            34., -29., 36., -25., 38., -21., 40., -17., 42., -13., 44., -9., 46., -5., 48., -1.,
            50., 3., 52., 7., 54., 11., 56., 15., 58., 19., 60., 23., 62., 27., 64., 31.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_sub_round_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_maskz_sub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            0., -29., 0., -25., 0., -21., 0., -17., 0., -13., 0., -9., 0., -5., 0., -1., 0., 3.,
            0., 7., 0., 11., 0., 15., 0., 19., 0., 23., 0., 27., 0., 31.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_sub_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_sub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_set_sh(-1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_sub_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let src = _mm_set_sh(4.0);
        let r = _mm_mask_sub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_set_sh(4.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_sub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 1, a, b,
        );
        let e = _mm_set_sh(-1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_sub_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r =
            _mm_maskz_sub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_set_sh(0.0);
        assert_eq_m128h(r, e);
        let r =
            _mm_maskz_sub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(1, a, b);
        let e = _mm_set_sh(-1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_sub_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_sub_sh(a, b);
        let e = _mm_set_sh(-1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_sub_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let src = _mm_set_sh(4.0);
        let r = _mm_mask_sub_sh(src, 0, a, b);
        let e = _mm_set_sh(4.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_sub_sh(src, 1, a, b);
        let e = _mm_set_sh(-1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_sub_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_maskz_sub_sh(0, a, b);
        let e = _mm_set_sh(0.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_sub_sh(1, a, b);
        let e = _mm_set_sh(-1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mul_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let r = _mm_mul_ph(a, b);
        let e = _mm_set_ph(8.0, 14.0, 18.0, 20.0, 20.0, 18.0, 14.0, 8.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_mul_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let src = _mm_set_ph(10., 11., 12., 13., 14., 15., 16., 17.);
        let r = _mm_mask_mul_ph(src, 0b01010101, a, b);
        let e = _mm_set_ph(10., 14., 12., 20., 14., 18., 16., 8.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_mul_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        let r = _mm_maskz_mul_ph(0b01010101, a, b);
        let e = _mm_set_ph(0., 14., 0., 20., 0., 18., 0., 8.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mul_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        let r = _mm256_mul_ph(a, b);
        let e = _mm256_set_ph(
            16.0, 30.0, 42.0, 52.0, 60.0, 66.0, 70.0, 72.0, 72.0, 70.0, 66.0, 60.0, 52.0, 42.0,
            30.0, 16.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_mul_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        let src = _mm256_set_ph(
            18., 19., 20., 21., 22., 23., 24., 25., 26., 27., 28., 29., 30., 31., 32., 33.,
        );
        let r = _mm256_mask_mul_ph(src, 0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            18., 30., 20., 52., 22., 66., 24., 72., 26., 70., 28., 60., 30., 42., 32., 16.,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_mul_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        let r = _mm256_maskz_mul_ph(0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            0., 30., 0., 52., 0., 66., 0., 72., 0., 70., 0., 60., 0., 42., 0., 16.,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mul_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_mul_ph(a, b);
        let e = _mm512_set_ph(
            32.0, 62.0, 90.0, 116.0, 140.0, 162.0, 182.0, 200.0, 216.0, 230.0, 242.0, 252.0, 260.0,
            266.0, 270.0, 272.0, 272.0, 270.0, 266.0, 260.0, 252.0, 242.0, 230.0, 216.0, 200.0,
            182.0, 162.0, 140.0, 116.0, 90.0, 62.0, 32.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_mul_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let src = _mm512_set_ph(
            34., 35., 36., 37., 38., 39., 40., 41., 42., 43., 44., 45., 46., 47., 48., 49., 50.,
            51., 52., 53., 54., 55., 56., 57., 58., 59., 60., 61., 62., 63., 64., 65.,
        );
        let r = _mm512_mask_mul_ph(src, 0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            34., 62., 36., 116., 38., 162., 40., 200., 42., 230., 44., 252., 46., 266., 48., 272.,
            50., 270., 52., 260., 54., 242., 56., 216., 58., 182., 60., 140., 62., 90., 64., 32.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_mul_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_maskz_mul_ph(0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            0., 62., 0., 116., 0., 162., 0., 200., 0., 230., 0., 252., 0., 266., 0., 272., 0.,
            270., 0., 260., 0., 242., 0., 216., 0., 182., 0., 140., 0., 90., 0., 32.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mul_round_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_mul_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm512_set_ph(
            32.0, 62.0, 90.0, 116.0, 140.0, 162.0, 182.0, 200.0, 216.0, 230.0, 242.0, 252.0, 260.0,
            266.0, 270.0, 272.0, 272.0, 270.0, 266.0, 260.0, 252.0, 242.0, 230.0, 216.0, 200.0,
            182.0, 162.0, 140.0, 116.0, 90.0, 62.0, 32.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_mul_round_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let src = _mm512_set_ph(
            34., 35., 36., 37., 38., 39., 40., 41., 42., 43., 44., 45., 46., 47., 48., 49., 50.,
            51., 52., 53., 54., 55., 56., 57., 58., 59., 60., 61., 62., 63., 64., 65.,
        );
        let r = _mm512_mask_mul_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            34., 62., 36., 116., 38., 162., 40., 200., 42., 230., 44., 252., 46., 266., 48., 272.,
            50., 270., 52., 260., 54., 242., 56., 216., 58., 182., 60., 140., 62., 90., 64., 32.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_mul_round_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            32.0, 31.0, 30.0, 29.0, 28.0, 27.0, 26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0,
            18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        );
        let r = _mm512_maskz_mul_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            0., 62., 0., 116., 0., 162., 0., 200., 0., 230., 0., 252., 0., 266., 0., 272., 0.,
            270., 0., 260., 0., 242., 0., 216., 0., 182., 0., 140., 0., 90., 0., 32.,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mul_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_mul_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_set_sh(2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_mul_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let src = _mm_set_sh(4.0);
        let r = _mm_mask_mul_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_set_sh(4.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_mul_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 1, a, b,
        );
        let e = _mm_set_sh(2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_mul_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r =
            _mm_maskz_mul_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_set_sh(0.0);
        assert_eq_m128h(r, e);
        let r =
            _mm_maskz_mul_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(1, a, b);
        let e = _mm_set_sh(2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mul_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_mul_sh(a, b);
        let e = _mm_set_sh(2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_mul_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let src = _mm_set_sh(4.0);
        let r = _mm_mask_mul_sh(src, 0, a, b);
        let e = _mm_set_sh(4.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_mul_sh(src, 1, a, b);
        let e = _mm_set_sh(2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_mul_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_maskz_mul_sh(0, a, b);
        let e = _mm_set_sh(0.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_mul_sh(1, a, b);
        let e = _mm_set_sh(2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_div_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let r = _mm_div_ph(a, b);
        let e = _mm_set1_ph(0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_div_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let src = _mm_set_ph(4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0);
        let r = _mm_mask_div_ph(src, 0b01010101, a, b);
        let e = _mm_set_ph(4.0, 0.5, 6.0, 0.5, 8.0, 0.5, 10.0, 0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_div_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let r = _mm_maskz_div_ph(0b01010101, a, b);
        let e = _mm_set_ph(0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_div_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let r = _mm256_div_ph(a, b);
        let e = _mm256_set1_ph(0.5);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_div_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let src = _mm256_set_ph(
            4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0,
            19.0,
        );
        let r = _mm256_mask_div_ph(src, 0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            4.0, 0.5, 6.0, 0.5, 8.0, 0.5, 10.0, 0.5, 12.0, 0.5, 14.0, 0.5, 16.0, 0.5, 18.0, 0.5,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_div_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let r = _mm256_maskz_div_ph(0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_div_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let r = _mm512_div_ph(a, b);
        let e = _mm512_set1_ph(0.5);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_div_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let src = _mm512_set_ph(
            4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0,
            19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0,
            33.0, 34.0, 35.0,
        );
        let r = _mm512_mask_div_ph(src, 0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            4.0, 0.5, 6.0, 0.5, 8.0, 0.5, 10.0, 0.5, 12.0, 0.5, 14.0, 0.5, 16.0, 0.5, 18.0, 0.5,
            20.0, 0.5, 22.0, 0.5, 24.0, 0.5, 26.0, 0.5, 28.0, 0.5, 30.0, 0.5, 32.0, 0.5, 34.0, 0.5,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_div_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let r = _mm512_maskz_div_ph(0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0,
            0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_div_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let r = _mm512_div_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm512_set1_ph(0.5);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_div_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let src = _mm512_set_ph(
            4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0,
            19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0,
            33.0, 34.0, 35.0,
        );
        let r = _mm512_mask_div_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            4.0, 0.5, 6.0, 0.5, 8.0, 0.5, 10.0, 0.5, 12.0, 0.5, 14.0, 0.5, 16.0, 0.5, 18.0, 0.5,
            20.0, 0.5, 22.0, 0.5, 24.0, 0.5, 26.0, 0.5, 28.0, 0.5, 30.0, 0.5, 32.0, 0.5, 34.0, 0.5,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_div_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let r = _mm512_maskz_div_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0,
            0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_div_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_div_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_set_sh(0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_div_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let src = _mm_set_sh(4.0);
        let r = _mm_mask_div_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_set_sh(4.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_div_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 1, a, b,
        );
        let e = _mm_set_sh(0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_div_round_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r =
            _mm_maskz_div_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_set_sh(0.0);
        assert_eq_m128h(r, e);
        let r =
            _mm_maskz_div_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(1, a, b);
        let e = _mm_set_sh(0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_div_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_div_sh(a, b);
        let e = _mm_set_sh(0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_div_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let src = _mm_set_sh(4.0);
        let r = _mm_mask_div_sh(src, 0, a, b);
        let e = _mm_set_sh(4.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_div_sh(src, 1, a, b);
        let e = _mm_set_sh(0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_div_sh() {
        let a = _mm_set_sh(1.0);
        let b = _mm_set_sh(2.0);
        let r = _mm_maskz_div_sh(0, a, b);
        let e = _mm_set_sh(0.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_div_sh(1, a, b);
        let e = _mm_set_sh(0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 1.0);
        let r = _mm_mul_pch(a, b);
        let e = _mm_set1_pch(-1.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_mul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 1.0);
        let src = _mm_setr_ph(2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let r = _mm_mask_mul_pch(src, 0b0101, a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_mul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 1.0);
        let r = _mm_maskz_mul_pch(0b0101, a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 1.0);
        let r = _mm256_mul_pch(a, b);
        let e = _mm256_set1_pch(-1.0, 0.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_mul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 1.0);
        let src = _mm256_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
        );
        let r = _mm256_mask_mul_pch(src, 0b01010101, a, b);
        let e = _mm256_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_mul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 1.0);
        let r = _mm256_maskz_mul_pch(0b01010101, a, b);
        let e = _mm256_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let r = _mm512_mul_pch(a, b);
        let e = _mm512_set1_pch(-1.0, 0.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_mul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let src = _mm512_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
            18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0,
            32.0, 33.0,
        );
        let r = _mm512_mask_mul_pch(src, 0b0101010101010101, a, b);
        let e = _mm512_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
            -1.0, 0.0, 20.0, 21.0, -1.0, 0.0, 24.0, 25.0, -1.0, 0.0, 28.0, 29.0, -1.0, 0.0, 32.0,
            33.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_mul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let r = _mm512_maskz_mul_pch(0b0101010101010101, a, b);
        let e = _mm512_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let r = _mm512_mul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm512_set1_pch(-1.0, 0.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_mul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let src = _mm512_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
            18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0,
            32.0, 33.0,
        );
        let r = _mm512_mask_mul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b0101010101010101,
            a,
            b,
        );
        let e = _mm512_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
            -1.0, 0.0, 20.0, 21.0, -1.0, 0.0, 24.0, 25.0, -1.0, 0.0, 28.0, 29.0, -1.0, 0.0, 32.0,
            33.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_mul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let r = _mm512_maskz_mul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b0101010101010101,
            a,
            b,
        );
        let e = _mm512_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let r = _mm_mul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_mul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let src = _mm_setr_ph(14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0);
        let r = _mm_mask_mul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_setr_ph(14.0, 15.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_mul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let r =
            _mm_maskz_mul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let r = _mm_mul_sch(a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_mul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let src = _mm_setr_ph(14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0);
        let r = _mm_mask_mul_sch(src, 0, a, b);
        let e = _mm_setr_ph(14.0, 15.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_mul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let r = _mm_maskz_mul_sch(0, a, b);
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_fmul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 1.0);
        let r = _mm_fmul_pch(a, b);
        let e = _mm_set1_pch(-1.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_fmul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 1.0);
        let src = _mm_setr_ph(2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let r = _mm_mask_fmul_pch(src, 0b0101, a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_fmul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 1.0);
        let r = _mm_maskz_fmul_pch(0b0101, a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_fmul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 1.0);
        let r = _mm256_fmul_pch(a, b);
        let e = _mm256_set1_pch(-1.0, 0.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_fmul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 1.0);
        let src = _mm256_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
        );
        let r = _mm256_mask_fmul_pch(src, 0b01010101, a, b);
        let e = _mm256_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_fmul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 1.0);
        let r = _mm256_maskz_fmul_pch(0b01010101, a, b);
        let e = _mm256_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let r = _mm512_fmul_pch(a, b);
        let e = _mm512_set1_pch(-1.0, 0.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let src = _mm512_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
            18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0,
            32.0, 33.0,
        );
        let r = _mm512_mask_fmul_pch(src, 0b0101010101010101, a, b);
        let e = _mm512_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
            -1.0, 0.0, 20.0, 21.0, -1.0, 0.0, 24.0, 25.0, -1.0, 0.0, 28.0, 29.0, -1.0, 0.0, 32.0,
            33.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let r = _mm512_maskz_fmul_pch(0b0101010101010101, a, b);
        let e = _mm512_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let r = _mm512_fmul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm512_set1_pch(-1.0, 0.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let src = _mm512_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
            18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0,
            32.0, 33.0,
        );
        let r = _mm512_mask_fmul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b0101010101010101,
            a,
            b,
        );
        let e = _mm512_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
            -1.0, 0.0, 20.0, 21.0, -1.0, 0.0, 24.0, 25.0, -1.0, 0.0, 28.0, 29.0, -1.0, 0.0, 32.0,
            33.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 1.0);
        let r = _mm512_maskz_fmul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b0101010101010101,
            a,
            b,
        );
        let e = _mm512_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fmul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let r = _mm_fmul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fmul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let src = _mm_setr_ph(14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0);
        let r = _mm_mask_fmul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_setr_ph(14.0, 15.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fmul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let r =
            _mm_maskz_fmul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fmul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let r = _mm_fmul_sch(a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fmul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let src = _mm_setr_ph(14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0);
        let r = _mm_mask_fmul_sch(src, 0, a, b);
        let e = _mm_setr_ph(14.0, 15.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fmul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 1.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let r = _mm_maskz_fmul_sch(0, a, b);
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_cmul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, -1.0);
        let r = _mm_cmul_pch(a, b);
        let e = _mm_set1_pch(-1.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_cmul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, -1.0);
        let src = _mm_setr_ph(2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let r = _mm_mask_cmul_pch(src, 0b0101, a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_cmul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, -1.0);
        let r = _mm_maskz_cmul_pch(0b0101, a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_cmul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, -1.0);
        let r = _mm256_cmul_pch(a, b);
        let e = _mm256_set1_pch(-1.0, 0.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_cmul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, -1.0);
        let src = _mm256_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
        );
        let r = _mm256_mask_cmul_pch(src, 0b01010101, a, b);
        let e = _mm256_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_cmul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, -1.0);
        let r = _mm256_maskz_cmul_pch(0b01010101, a, b);
        let e = _mm256_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_cmul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let r = _mm512_cmul_pch(a, b);
        let e = _mm512_set1_pch(-1.0, 0.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_cmul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let src = _mm512_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
            18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0,
            32.0, 33.0,
        );
        let r = _mm512_mask_cmul_pch(src, 0b0101010101010101, a, b);
        let e = _mm512_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
            -1.0, 0.0, 20.0, 21.0, -1.0, 0.0, 24.0, 25.0, -1.0, 0.0, 28.0, 29.0, -1.0, 0.0, 32.0,
            33.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_cmul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let r = _mm512_maskz_cmul_pch(0b0101010101010101, a, b);
        let e = _mm512_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_cmul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let r = _mm512_cmul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm512_set1_pch(-1.0, 0.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_cmul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let src = _mm512_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
            18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0,
            32.0, 33.0,
        );
        let r = _mm512_mask_cmul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b0101010101010101,
            a,
            b,
        );
        let e = _mm512_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
            -1.0, 0.0, 20.0, 21.0, -1.0, 0.0, 24.0, 25.0, -1.0, 0.0, 28.0, 29.0, -1.0, 0.0, 32.0,
            33.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_cmul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let r = _mm512_maskz_cmul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b0101010101010101,
            a,
            b,
        );
        let e = _mm512_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_cmul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let r = _mm_cmul_sch(a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_cmul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let src = _mm_setr_ph(14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0);
        let r = _mm_mask_cmul_sch(src, 0, a, b);
        let e = _mm_setr_ph(14.0, 15.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_cmul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let r = _mm_maskz_cmul_sch(0, a, b);
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_cmul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let r = _mm_cmul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_cmul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let src = _mm_setr_ph(14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0);
        let r = _mm_mask_cmul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_setr_ph(14.0, 15.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_cmul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let r =
            _mm_maskz_cmul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_fcmul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, -1.0);
        let r = _mm_fcmul_pch(a, b);
        let e = _mm_set1_pch(-1.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_fcmul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, -1.0);
        let src = _mm_setr_ph(2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let r = _mm_mask_fcmul_pch(src, 0b0101, a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_fcmul_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, -1.0);
        let r = _mm_maskz_fcmul_pch(0b0101, a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_fcmul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, -1.0);
        let r = _mm256_fcmul_pch(a, b);
        let e = _mm256_set1_pch(-1.0, 0.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_fcmul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, -1.0);
        let src = _mm256_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
        );
        let r = _mm256_mask_fcmul_pch(src, 0b01010101, a, b);
        let e = _mm256_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_fcmul_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, -1.0);
        let r = _mm256_maskz_fcmul_pch(0b01010101, a, b);
        let e = _mm256_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fcmul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let r = _mm512_fcmul_pch(a, b);
        let e = _mm512_set1_pch(-1.0, 0.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fcmul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let src = _mm512_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
            18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0,
            32.0, 33.0,
        );
        let r = _mm512_mask_fcmul_pch(src, 0b0101010101010101, a, b);
        let e = _mm512_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
            -1.0, 0.0, 20.0, 21.0, -1.0, 0.0, 24.0, 25.0, -1.0, 0.0, 28.0, 29.0, -1.0, 0.0, 32.0,
            33.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fcmul_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let r = _mm512_maskz_fcmul_pch(0b0101010101010101, a, b);
        let e = _mm512_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fcmul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let r = _mm512_fcmul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm512_set1_pch(-1.0, 0.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fcmul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let src = _mm512_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
            18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0,
            32.0, 33.0,
        );
        let r = _mm512_mask_fcmul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b0101010101010101,
            a,
            b,
        );
        let e = _mm512_setr_ph(
            -1.0, 0.0, 4.0, 5.0, -1.0, 0.0, 8.0, 9.0, -1.0, 0.0, 12.0, 13.0, -1.0, 0.0, 16.0, 17.0,
            -1.0, 0.0, 20.0, 21.0, -1.0, 0.0, 24.0, 25.0, -1.0, 0.0, 28.0, 29.0, -1.0, 0.0, 32.0,
            33.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fcmul_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, -1.0);
        let r = _mm512_maskz_fcmul_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b0101010101010101,
            a,
            b,
        );
        let e = _mm512_setr_ph(
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
            -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fcmul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let r = _mm_fcmul_sch(a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fcmul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let src = _mm_setr_ph(14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0);
        let r = _mm_mask_fcmul_sch(src, 0, a, b);
        let e = _mm_setr_ph(14.0, 15.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fcmul_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let r = _mm_maskz_fcmul_sch(0, a, b);
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fcmul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let r = _mm_fcmul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_setr_ph(-1.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fcmul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let src = _mm_setr_ph(14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0);
        let r = _mm_mask_fcmul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_setr_ph(14.0, 15.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fcmul_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, -1.0, 8.0, -9.0, 10.0, -11.0, 12.0, -13.0);
        let r =
            _mm_maskz_fcmul_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_abs_ph() {
        let a = _mm_set_ph(-1.0, 0.0, 1.0, -2.0, 3.0, -4.0, 5.0, -6.0);
        let r = _mm_abs_ph(a);
        let e = _mm_set_ph(1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_abs_ph() {
        let a = _mm256_set_ph(
            -1.0, 0.0, 1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0, 9.0, -10.0, 11.0, -12.0, 13.0,
            -14.0,
        );
        let r = _mm256_abs_ph(a);
        let e = _mm256_set_ph(
            1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_abs_ph() {
        let a = _mm512_set_ph(
            -1.0, 0.0, 1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0, 9.0, -10.0, 11.0, -12.0, 13.0,
            -14.0, 15.0, -16.0, 17.0, -18.0, 19.0, -20.0, 21.0, -22.0, 23.0, -24.0, 25.0, -26.0,
            27.0, -28.0, 29.0, -30.0,
        );
        let r = _mm512_abs_ph(a);
        let e = _mm512_set_ph(
            1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0,
            15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0,
            29.0, 30.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_conj_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let r = _mm_conj_pch(a);
        let e = _mm_set1_pch(0.0, -1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_conj_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let src = _mm_setr_ph(2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let r = _mm_mask_conj_pch(src, 0b0101, a);
        let e = _mm_setr_ph(0.0, -1.0, 4.0, 5.0, 0.0, -1.0, 8.0, 9.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_conj_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let r = _mm_maskz_conj_pch(0b0101, a);
        let e = _mm_setr_ph(0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_conj_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let r = _mm256_conj_pch(a);
        let e = _mm256_set1_pch(0.0, -1.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_conj_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let src = _mm256_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
        );
        let r = _mm256_mask_conj_pch(src, 0b01010101, a);
        let e = _mm256_setr_ph(
            0.0, -1.0, 4.0, 5.0, 0.0, -1.0, 8.0, 9.0, 0.0, -1.0, 12.0, 13.0, 0.0, -1.0, 16.0, 17.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_conj_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let r = _mm256_maskz_conj_pch(0b01010101, a);
        let e = _mm256_setr_ph(
            0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_conj_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let r = _mm512_conj_pch(a);
        let e = _mm512_set1_pch(0.0, -1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_conj_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let src = _mm512_setr_ph(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
            18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0,
            32.0, 33.0,
        );
        let r = _mm512_mask_conj_pch(src, 0b0101010101010101, a);
        let e = _mm512_setr_ph(
            0.0, -1.0, 4.0, 5.0, 0.0, -1.0, 8.0, 9.0, 0.0, -1.0, 12.0, 13.0, 0.0, -1.0, 16.0, 17.0,
            0.0, -1.0, 20.0, 21.0, 0.0, -1.0, 24.0, 25.0, 0.0, -1.0, 28.0, 29.0, 0.0, -1.0, 32.0,
            33.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_conj_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let r = _mm512_maskz_conj_pch(0b0101010101010101, a);
        let e = _mm512_setr_ph(
            0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_fmadd_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 2.0);
        let c = _mm_set1_pch(0.0, 3.0);
        let r = _mm_fmadd_pch(a, b, c);
        let e = _mm_set1_pch(-2.0, 3.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_fmadd_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 2.0);
        let c = _mm_set1_pch(0.0, 3.0);
        let r = _mm_mask_fmadd_pch(a, 0b0101, b, c);
        let e = _mm_setr_ph(-2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask3_fmadd_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 2.0);
        let c = _mm_set1_pch(0.0, 3.0);
        let r = _mm_mask3_fmadd_pch(a, b, c, 0b0101);
        let e = _mm_setr_ph(-2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_fmadd_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 2.0);
        let c = _mm_set1_pch(0.0, 3.0);
        let r = _mm_maskz_fmadd_pch(0b0101, a, b, c);
        let e = _mm_setr_ph(-2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_fmadd_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 2.0);
        let c = _mm256_set1_pch(0.0, 3.0);
        let r = _mm256_fmadd_pch(a, b, c);
        let e = _mm256_set1_pch(-2.0, 3.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_fmadd_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 2.0);
        let c = _mm256_set1_pch(0.0, 3.0);
        let r = _mm256_mask_fmadd_pch(a, 0b01010101, b, c);
        let e = _mm256_setr_ph(
            -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask3_fmadd_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 2.0);
        let c = _mm256_set1_pch(0.0, 3.0);
        let r = _mm256_mask3_fmadd_pch(a, b, c, 0b01010101);
        let e = _mm256_setr_ph(
            -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_fmadd_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 2.0);
        let c = _mm256_set1_pch(0.0, 3.0);
        let r = _mm256_maskz_fmadd_pch(0b01010101, a, b, c);
        let e = _mm256_setr_ph(
            -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmadd_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_fmadd_pch(a, b, c);
        let e = _mm512_set1_pch(-2.0, 3.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmadd_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_mask_fmadd_pch(a, 0b0101010101010101, b, c);
        let e = _mm512_setr_ph(
            -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0,
            -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fmadd_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_mask3_fmadd_pch(a, b, c, 0b0101010101010101);
        let e = _mm512_setr_ph(
            -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0,
            -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmadd_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_maskz_fmadd_pch(0b0101010101010101, a, b, c);
        let e = _mm512_setr_ph(
            -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0,
            -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmadd_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r =
            _mm512_fmadd_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm512_set1_pch(-2.0, 3.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmadd_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_mask_fmadd_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            0b0101010101010101,
            b,
            c,
        );
        let e = _mm512_setr_ph(
            -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0,
            -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0, -2.0, 3.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fmadd_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_mask3_fmadd_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            b,
            c,
            0b0101010101010101,
        );
        let e = _mm512_setr_ph(
            -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0,
            -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0, -2.0, 3.0, 0.0, 3.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmadd_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_maskz_fmadd_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b0101010101010101,
            a,
            b,
            c,
        );
        let e = _mm512_setr_ph(
            -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0,
            -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0, -2.0, 3.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fmadd_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_fmadd_sch(a, b, c);
        let e = _mm_setr_ph(-2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fmadd_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_mask_fmadd_sch(a, 0, b, c);
        let e = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fmadd_sch(a, 1, b, c);
        let e = _mm_setr_ph(-2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fmadd_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_mask3_fmadd_sch(a, b, c, 0);
        let e = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fmadd_sch(a, b, c, 1);
        let e = _mm_setr_ph(-2.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fmadd_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_maskz_fmadd_sch(0, a, b, c);
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fmadd_sch(1, a, b, c);
        let e = _mm_setr_ph(-2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fmadd_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_fmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm_setr_ph(-2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fmadd_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_mask_fmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 0, b, c,
        );
        let e = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 1, b, c,
        );
        let e = _mm_setr_ph(-2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fmadd_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_mask3_fmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 0,
        );
        let e = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 1,
        );
        let e = _mm_setr_ph(-2.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fmadd_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_maskz_fmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0, a, b, c,
        );
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            1, a, b, c,
        );
        let e = _mm_setr_ph(-2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_fcmadd_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 2.0);
        let c = _mm_set1_pch(0.0, 3.0);
        let r = _mm_fcmadd_pch(a, b, c);
        let e = _mm_set1_pch(2.0, 3.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_fcmadd_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 2.0);
        let c = _mm_set1_pch(0.0, 3.0);
        let r = _mm_mask_fcmadd_pch(a, 0b0101, b, c);
        let e = _mm_setr_ph(2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask3_fcmadd_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 2.0);
        let c = _mm_set1_pch(0.0, 3.0);
        let r = _mm_mask3_fcmadd_pch(a, b, c, 0b0101);
        let e = _mm_setr_ph(2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_fcmadd_pch() {
        let a = _mm_set1_pch(0.0, 1.0);
        let b = _mm_set1_pch(0.0, 2.0);
        let c = _mm_set1_pch(0.0, 3.0);
        let r = _mm_maskz_fcmadd_pch(0b0101, a, b, c);
        let e = _mm_setr_ph(2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_fcmadd_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 2.0);
        let c = _mm256_set1_pch(0.0, 3.0);
        let r = _mm256_fcmadd_pch(a, b, c);
        let e = _mm256_set1_pch(2.0, 3.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_fcmadd_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 2.0);
        let c = _mm256_set1_pch(0.0, 3.0);
        let r = _mm256_mask_fcmadd_pch(a, 0b01010101, b, c);
        let e = _mm256_setr_ph(
            2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask3_fcmadd_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 2.0);
        let c = _mm256_set1_pch(0.0, 3.0);
        let r = _mm256_mask3_fcmadd_pch(a, b, c, 0b01010101);
        let e = _mm256_setr_ph(
            2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_fcmadd_pch() {
        let a = _mm256_set1_pch(0.0, 1.0);
        let b = _mm256_set1_pch(0.0, 2.0);
        let c = _mm256_set1_pch(0.0, 3.0);
        let r = _mm256_maskz_fcmadd_pch(0b01010101, a, b, c);
        let e = _mm256_setr_ph(
            2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fcmadd_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_fcmadd_pch(a, b, c);
        let e = _mm512_set1_pch(2.0, 3.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fcmadd_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_mask_fcmadd_pch(a, 0b0101010101010101, b, c);
        let e = _mm512_setr_ph(
            2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0,
            3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fcmadd_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_mask3_fcmadd_pch(a, b, c, 0b0101010101010101);
        let e = _mm512_setr_ph(
            2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0,
            3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fcmadd_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_maskz_fcmadd_pch(0b0101010101010101, a, b, c);
        let e = _mm512_setr_ph(
            2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0,
            3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fcmadd_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r =
            _mm512_fcmadd_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm512_set1_pch(2.0, 3.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fcmadd_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_mask_fcmadd_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            0b0101010101010101,
            b,
            c,
        );
        let e = _mm512_setr_ph(
            2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0,
            3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fcmadd_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_mask3_fcmadd_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            b,
            c,
            0b0101010101010101,
        );
        let e = _mm512_setr_ph(
            2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0,
            3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0, 2.0, 3.0, 0.0, 3.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fcmadd_round_pch() {
        let a = _mm512_set1_pch(0.0, 1.0);
        let b = _mm512_set1_pch(0.0, 2.0);
        let c = _mm512_set1_pch(0.0, 3.0);
        let r = _mm512_maskz_fcmadd_round_pch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b0101010101010101,
            a,
            b,
            c,
        );
        let e = _mm512_setr_ph(
            2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0,
            3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fcmadd_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_fcmadd_sch(a, b, c);
        let e = _mm_setr_ph(2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fcmadd_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_mask_fcmadd_sch(a, 0, b, c);
        let e = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fcmadd_sch(a, 1, b, c);
        let e = _mm_setr_ph(2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fcmadd_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_mask3_fcmadd_sch(a, b, c, 0);
        let e = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fcmadd_sch(a, b, c, 1);
        let e = _mm_setr_ph(2.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fcmadd_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_maskz_fcmadd_sch(0, a, b, c);
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fcmadd_sch(1, a, b, c);
        let e = _mm_setr_ph(2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fcmadd_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_fcmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm_setr_ph(2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fcmadd_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_mask_fcmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 0, b, c,
        );
        let e = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fcmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 1, b, c,
        );
        let e = _mm_setr_ph(2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fcmadd_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_mask3_fcmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 0,
        );
        let e = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fcmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 1,
        );
        let e = _mm_setr_ph(2.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fcmadd_round_sch() {
        let a = _mm_setr_ph(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let b = _mm_setr_ph(0.0, 2.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0);
        let c = _mm_setr_ph(0.0, 3.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0);
        let r = _mm_maskz_fcmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0, a, b, c,
        );
        let e = _mm_setr_ph(0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fcmadd_round_sch::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            1, a, b, c,
        );
        let e = _mm_setr_ph(2.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_fmadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_fmadd_ph(a, b, c);
        let e = _mm_set1_ph(5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_fmadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask_fmadd_ph(a, 0b01010101, b, c);
        let e = _mm_set_ph(1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask3_fmadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask3_fmadd_ph(a, b, c, 0b01010101);
        let e = _mm_set_ph(3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_fmadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_maskz_fmadd_ph(0b01010101, a, b, c);
        let e = _mm_set_ph(0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_fmadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_fmadd_ph(a, b, c);
        let e = _mm256_set1_ph(5.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_fmadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask_fmadd_ph(a, 0b0101010101010101, b, c);
        let e = _mm256_set_ph(
            1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask3_fmadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask3_fmadd_ph(a, b, c, 0b0101010101010101);
        let e = _mm256_set_ph(
            3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_fmadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_maskz_fmadd_ph(0b0101010101010101, a, b, c);
        let e = _mm256_set_ph(
            0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_fmadd_ph(a, b, c);
        let e = _mm512_set1_ph(5.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fmadd_ph(a, 0b01010101010101010101010101010101, b, c);
        let e = _mm512_set_ph(
            1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0,
            5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fmadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fmadd_ph(a, b, c, 0b01010101010101010101010101010101);
        let e = _mm512_set_ph(
            3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0,
            5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fmadd_ph(0b01010101010101010101010101010101, a, b, c);
        let e = _mm512_set_ph(
            0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0,
            5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_fmadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm512_set1_ph(5.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fmadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            0b01010101010101010101010101010101,
            b,
            c,
        );
        let e = _mm512_set_ph(
            1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0,
            5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0, 1.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fmadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fmadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            b,
            c,
            0b01010101010101010101010101010101,
        );
        let e = _mm512_set_ph(
            3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0,
            5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fmadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
            b,
            c,
        );
        let e = _mm512_set_ph(
            0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0,
            5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0, 0.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fmadd_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_fmadd_sh(a, b, c);
        let e = _mm_setr_ph(5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fmadd_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_fmadd_sh(a, 0, b, c);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fmadd_sh(a, 1, b, c);
        let e = _mm_setr_ph(5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fmadd_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask3_fmadd_sh(a, b, c, 0);
        let e = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fmadd_sh(a, b, c, 1);
        let e = _mm_setr_ph(5.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fmadd_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_maskz_fmadd_sh(0, a, b, c);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fmadd_sh(1, a, b, c);
        let e = _mm_setr_ph(5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fmadd_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_fmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm_setr_ph(5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fmadd_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_fmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 0, b, c,
        );
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 1, b, c,
        );
        let e = _mm_setr_ph(5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fmadd_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask3_fmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 0,
        );
        let e = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 1,
        );
        let e = _mm_setr_ph(5.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fmadd_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_maskz_fmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0, a, b, c,
        );
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            1, a, b, c,
        );
        let e = _mm_setr_ph(5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_fmsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_fmsub_ph(a, b, c);
        let e = _mm_set1_ph(-1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_fmsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask_fmsub_ph(a, 0b01010101, b, c);
        let e = _mm_set_ph(1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask3_fmsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask3_fmsub_ph(a, b, c, 0b01010101);
        let e = _mm_set_ph(3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_fmsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_maskz_fmsub_ph(0b01010101, a, b, c);
        let e = _mm_set_ph(0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_fmsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_fmsub_ph(a, b, c);
        let e = _mm256_set1_ph(-1.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_fmsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask_fmsub_ph(a, 0b0101010101010101, b, c);
        let e = _mm256_set_ph(
            1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask3_fmsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask3_fmsub_ph(a, b, c, 0b0101010101010101);
        let e = _mm256_set_ph(
            3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_fmsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_maskz_fmsub_ph(0b0101010101010101, a, b, c);
        let e = _mm256_set_ph(
            0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_fmsub_ph(a, b, c);
        let e = _mm512_set1_ph(-1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fmsub_ph(a, 0b01010101010101010101010101010101, b, c);
        let e = _mm512_set_ph(
            1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0,
            1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fmsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fmsub_ph(a, b, c, 0b01010101010101010101010101010101);
        let e = _mm512_set_ph(
            3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0,
            3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fmsub_ph(0b01010101010101010101010101010101, a, b, c);
        let e = _mm512_set_ph(
            0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0,
            0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_fmsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm512_set1_ph(-1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fmsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            0b01010101010101010101010101010101,
            b,
            c,
        );
        let e = _mm512_set_ph(
            1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0,
            1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fmsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fmsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            b,
            c,
            0b01010101010101010101010101010101,
        );
        let e = _mm512_set_ph(
            3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0,
            3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0, 3.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fmsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
            b,
            c,
        );
        let e = _mm512_set_ph(
            0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0,
            0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fmsub_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_fmsub_sh(a, b, c);
        let e = _mm_setr_ph(-1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fmsub_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_fmsub_sh(a, 0, b, c);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fmsub_sh(a, 1, b, c);
        let e = _mm_setr_ph(-1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fmsub_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask3_fmsub_sh(a, b, c, 0);
        let e = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fmsub_sh(a, b, c, 1);
        let e = _mm_setr_ph(-1.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fmsub_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_maskz_fmsub_sh(0, a, b, c);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fmsub_sh(1, a, b, c);
        let e = _mm_setr_ph(-1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fmsub_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_fmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm_setr_ph(-1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fmsub_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_fmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 0, b, c,
        );
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 1, b, c,
        );
        let e = _mm_setr_ph(-1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fmsub_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask3_fmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 0,
        );
        let e = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 1,
        );
        let e = _mm_setr_ph(-1.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fmsub_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_maskz_fmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0, a, b, c,
        );
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            1, a, b, c,
        );
        let e = _mm_setr_ph(-1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_fnmadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_fnmadd_ph(a, b, c);
        let e = _mm_set1_ph(1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_fnmadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask_fnmadd_ph(a, 0b01010101, b, c);
        let e = _mm_set_ph(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask3_fnmadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask3_fnmadd_ph(a, b, c, 0b01010101);
        let e = _mm_set_ph(3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_fnmadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_maskz_fnmadd_ph(0b01010101, a, b, c);
        let e = _mm_set_ph(0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_fnmadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_fnmadd_ph(a, b, c);
        let e = _mm256_set1_ph(1.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_fnmadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask_fnmadd_ph(a, 0b0101010101010101, b, c);
        let e = _mm256_set_ph(
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask3_fnmadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask3_fnmadd_ph(a, b, c, 0b0101010101010101);
        let e = _mm256_set_ph(
            3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_fnmadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_maskz_fnmadd_ph(0b0101010101010101, a, b, c);
        let e = _mm256_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fnmadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_fnmadd_ph(a, b, c);
        let e = _mm512_set1_ph(1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fnmadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fnmadd_ph(a, 0b01010101010101010101010101010101, b, c);
        let e = _mm512_set_ph(
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fnmadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fnmadd_ph(a, b, c, 0b01010101010101010101010101010101);
        let e = _mm512_set_ph(
            3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0,
            1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fnmadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fnmadd_ph(0b01010101010101010101010101010101, a, b, c);
        let e = _mm512_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fnmadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r =
            _mm512_fnmadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm512_set1_ph(1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fnmadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fnmadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            0b01010101010101010101010101010101,
            b,
            c,
        );
        let e = _mm512_set_ph(
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fnmadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fnmadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            b,
            c,
            0b01010101010101010101010101010101,
        );
        let e = _mm512_set_ph(
            3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0,
            1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fnmadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fnmadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
            b,
            c,
        );
        let e = _mm512_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fnmadd_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_fnmadd_sh(a, b, c);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fnmadd_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_fnmadd_sh(a, 0, b, c);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fnmadd_sh(a, 1, b, c);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fnmadd_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask3_fnmadd_sh(a, b, c, 0);
        let e = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fnmadd_sh(a, b, c, 1);
        let e = _mm_setr_ph(1.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fnmadd_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_maskz_fnmadd_sh(0, a, b, c);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fnmadd_sh(1, a, b, c);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fnmadd_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_fnmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fnmadd_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_fnmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 0, b, c,
        );
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fnmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 1, b, c,
        );
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fnmadd_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask3_fnmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 0,
        );
        let e = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fnmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 1,
        );
        let e = _mm_setr_ph(1.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fnmadd_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_maskz_fnmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0, a, b, c,
        );
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fnmadd_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            1, a, b, c,
        );
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_fnmsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_fnmsub_ph(a, b, c);
        let e = _mm_set1_ph(-5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_fnmsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask_fnmsub_ph(a, 0b01010101, b, c);
        let e = _mm_set_ph(1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask3_fnmsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask3_fnmsub_ph(a, b, c, 0b01010101);
        let e = _mm_set_ph(3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_fnmsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_maskz_fnmsub_ph(0b01010101, a, b, c);
        let e = _mm_set_ph(0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_fnmsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_fnmsub_ph(a, b, c);
        let e = _mm256_set1_ph(-5.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_fnmsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask_fnmsub_ph(a, 0b0101010101010101, b, c);
        let e = _mm256_set_ph(
            1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask3_fnmsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask3_fnmsub_ph(a, b, c, 0b0101010101010101);
        let e = _mm256_set_ph(
            3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_fnmsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_maskz_fnmsub_ph(0b0101010101010101, a, b, c);
        let e = _mm256_set_ph(
            0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fnmsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_fnmsub_ph(a, b, c);
        let e = _mm512_set1_ph(-5.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fnmsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fnmsub_ph(a, 0b01010101010101010101010101010101, b, c);
        let e = _mm512_set_ph(
            1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0,
            1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fnmsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fnmsub_ph(a, b, c, 0b01010101010101010101010101010101);
        let e = _mm512_set_ph(
            3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0,
            3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fnmsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fnmsub_ph(0b01010101010101010101010101010101, a, b, c);
        let e = _mm512_set_ph(
            0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0,
            0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fnmsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r =
            _mm512_fnmsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm512_set1_ph(-5.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fnmsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fnmsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            0b01010101010101010101010101010101,
            b,
            c,
        );
        let e = _mm512_set_ph(
            1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0,
            1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0, 1.0, -5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fnmsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fnmsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            b,
            c,
            0b01010101010101010101010101010101,
        );
        let e = _mm512_set_ph(
            3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0,
            3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0, 3.0, -5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fnmsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fnmsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
            b,
            c,
        );
        let e = _mm512_set_ph(
            0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0,
            0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0, 0.0, -5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fnmsub_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_fnmsub_sh(a, b, c);
        let e = _mm_setr_ph(-5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fnmsub_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_fnmsub_sh(a, 0, b, c);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fnmsub_sh(a, 1, b, c);
        let e = _mm_setr_ph(-5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fnmsub_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask3_fnmsub_sh(a, b, c, 0);
        let e = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fnmsub_sh(a, b, c, 1);
        let e = _mm_setr_ph(-5.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fnmsub_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_maskz_fnmsub_sh(0, a, b, c);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fnmsub_sh(1, a, b, c);
        let e = _mm_setr_ph(-5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fnmsub_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_fnmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm_setr_ph(-5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fnmsub_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_fnmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 0, b, c,
        );
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_fnmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, 1, b, c,
        );
        let e = _mm_setr_ph(-5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask3_fnmsub_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask3_fnmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 0,
        );
        let e = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
        let r = _mm_mask3_fnmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a, b, c, 1,
        );
        let e = _mm_setr_ph(-5.0, 30., 31., 32., 33., 34., 35., 36.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_fnmsub_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(2.0, 20., 21., 22., 23., 24., 25., 26.);
        let c = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_maskz_fnmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0, a, b, c,
        );
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_fnmsub_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            1, a, b, c,
        );
        let e = _mm_setr_ph(-5.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_fmaddsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_fmaddsub_ph(a, b, c);
        let e = _mm_set_ph(5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_fmaddsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask_fmaddsub_ph(a, 0b00110011, b, c);
        let e = _mm_set_ph(1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask3_fmaddsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask3_fmaddsub_ph(a, b, c, 0b00110011);
        let e = _mm_set_ph(3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_fmaddsub_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_maskz_fmaddsub_ph(0b00110011, a, b, c);
        let e = _mm_set_ph(0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_fmaddsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_fmaddsub_ph(a, b, c);
        let e = _mm256_set_ph(
            5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_fmaddsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask_fmaddsub_ph(a, 0b0011001100110011, b, c);
        let e = _mm256_set_ph(
            1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask3_fmaddsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask3_fmaddsub_ph(a, b, c, 0b0011001100110011);
        let e = _mm256_set_ph(
            3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_fmaddsub_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_maskz_fmaddsub_ph(0b0011001100110011, a, b, c);
        let e = _mm256_set_ph(
            0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmaddsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_fmaddsub_ph(a, b, c);
        let e = _mm512_set_ph(
            5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0,
            5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmaddsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fmaddsub_ph(a, 0b00110011001100110011001100110011, b, c);
        let e = _mm512_set_ph(
            1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0,
            1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fmaddsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fmaddsub_ph(a, b, c, 0b00110011001100110011001100110011);
        let e = _mm512_set_ph(
            3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0,
            3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmaddsub_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fmaddsub_ph(0b00110011001100110011001100110011, a, b, c);
        let e = _mm512_set_ph(
            0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0,
            0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmaddsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r =
            _mm512_fmaddsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm512_set_ph(
            5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0,
            5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmaddsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fmaddsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            0b00110011001100110011001100110011,
            b,
            c,
        );
        let e = _mm512_set_ph(
            1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0,
            1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0, 1.0, 1.0, 5.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fmaddsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fmaddsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            b,
            c,
            0b00110011001100110011001100110011,
        );
        let e = _mm512_set_ph(
            3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0,
            3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0, 3.0, 3.0, 5.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmaddsub_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fmaddsub_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b00110011001100110011001100110011,
            a,
            b,
            c,
        );
        let e = _mm512_set_ph(
            0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0,
            0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0, 0.0, 0.0, 5.0, -1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_fmsubadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_fmsubadd_ph(a, b, c);
        let e = _mm_set_ph(-1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_fmsubadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask_fmsubadd_ph(a, 0b00110011, b, c);
        let e = _mm_set_ph(1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask3_fmsubadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_mask3_fmsubadd_ph(a, b, c, 0b00110011);
        let e = _mm_set_ph(3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_fmsubadd_ph() {
        let a = _mm_set1_ph(1.0);
        let b = _mm_set1_ph(2.0);
        let c = _mm_set1_ph(3.0);
        let r = _mm_maskz_fmsubadd_ph(0b00110011, a, b, c);
        let e = _mm_set_ph(0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_fmsubadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_fmsubadd_ph(a, b, c);
        let e = _mm256_set_ph(
            -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_fmsubadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask_fmsubadd_ph(a, 0b0011001100110011, b, c);
        let e = _mm256_set_ph(
            1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask3_fmsubadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_mask3_fmsubadd_ph(a, b, c, 0b0011001100110011);
        let e = _mm256_set_ph(
            3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_fmsubadd_ph() {
        let a = _mm256_set1_ph(1.0);
        let b = _mm256_set1_ph(2.0);
        let c = _mm256_set1_ph(3.0);
        let r = _mm256_maskz_fmsubadd_ph(0b0011001100110011, a, b, c);
        let e = _mm256_set_ph(
            0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmsubadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_fmsubadd_ph(a, b, c);
        let e = _mm512_set_ph(
            -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0,
            -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmsubadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fmsubadd_ph(a, 0b00110011001100110011001100110011, b, c);
        let e = _mm512_set_ph(
            1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0,
            1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fmsubadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fmsubadd_ph(a, b, c, 0b00110011001100110011001100110011);
        let e = _mm512_set_ph(
            3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0,
            3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmsubadd_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fmsubadd_ph(0b00110011001100110011001100110011, a, b, c);
        let e = _mm512_set_ph(
            0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0,
            0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fmsubadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r =
            _mm512_fmsubadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b, c);
        let e = _mm512_set_ph(
            -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0,
            -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0, -1.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fmsubadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask_fmsubadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            0b00110011001100110011001100110011,
            b,
            c,
        );
        let e = _mm512_set_ph(
            1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0,
            1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0, 1.0, 1.0, -1.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask3_fmsubadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_mask3_fmsubadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            a,
            b,
            c,
            0b00110011001100110011001100110011,
        );
        let e = _mm512_set_ph(
            3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0,
            3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0, 3.0, 3.0, -1.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_fmsubadd_round_ph() {
        let a = _mm512_set1_ph(1.0);
        let b = _mm512_set1_ph(2.0);
        let c = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_fmsubadd_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b00110011001100110011001100110011,
            a,
            b,
            c,
        );
        let e = _mm512_set_ph(
            0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0,
            0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0, 0.0, 0.0, -1.0, 5.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_rcp_ph() {
        let a = _mm_set1_ph(2.0);
        let r = _mm_rcp_ph(a);
        let e = _mm_set1_ph(0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_rcp_ph() {
        let a = _mm_set1_ph(2.0);
        let src = _mm_set1_ph(1.0);
        let r = _mm_mask_rcp_ph(src, 0b01010101, a);
        let e = _mm_set_ph(1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_rcp_ph() {
        let a = _mm_set1_ph(2.0);
        let r = _mm_maskz_rcp_ph(0b01010101, a);
        let e = _mm_set_ph(0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_rcp_ph() {
        let a = _mm256_set1_ph(2.0);
        let r = _mm256_rcp_ph(a);
        let e = _mm256_set1_ph(0.5);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_rcp_ph() {
        let a = _mm256_set1_ph(2.0);
        let src = _mm256_set1_ph(1.0);
        let r = _mm256_mask_rcp_ph(src, 0b0101010101010101, a);
        let e = _mm256_set_ph(
            1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_rcp_ph() {
        let a = _mm256_set1_ph(2.0);
        let r = _mm256_maskz_rcp_ph(0b0101010101010101, a);
        let e = _mm256_set_ph(
            0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_rcp_ph() {
        let a = _mm512_set1_ph(2.0);
        let r = _mm512_rcp_ph(a);
        let e = _mm512_set1_ph(0.5);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_rcp_ph() {
        let a = _mm512_set1_ph(2.0);
        let src = _mm512_set1_ph(1.0);
        let r = _mm512_mask_rcp_ph(src, 0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0,
            0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_rcp_ph() {
        let a = _mm512_set1_ph(2.0);
        let r = _mm512_maskz_rcp_ph(0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0,
            0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_rcp_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let r = _mm_rcp_sh(a, b);
        let e = _mm_setr_ph(0.5, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_rcp_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let src = _mm_setr_ph(3.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0);
        let r = _mm_mask_rcp_sh(src, 0, a, b);
        let e = _mm_setr_ph(3.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_rcp_sh(src, 1, a, b);
        let e = _mm_setr_ph(0.5, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_rcp_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let r = _mm_maskz_rcp_sh(0, a, b);
        let e = _mm_setr_ph(0.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_rcp_sh(1, a, b);
        let e = _mm_setr_ph(0.5, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_rsqrt_ph() {
        let a = _mm_set1_ph(4.0);
        let r = _mm_rsqrt_ph(a);
        let e = _mm_set1_ph(0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_rsqrt_ph() {
        let a = _mm_set1_ph(4.0);
        let src = _mm_set1_ph(1.0);
        let r = _mm_mask_rsqrt_ph(src, 0b01010101, a);
        let e = _mm_set_ph(1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_rsqrt_ph() {
        let a = _mm_set1_ph(4.0);
        let r = _mm_maskz_rsqrt_ph(0b01010101, a);
        let e = _mm_set_ph(0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_rsqrt_ph() {
        let a = _mm256_set1_ph(4.0);
        let r = _mm256_rsqrt_ph(a);
        let e = _mm256_set1_ph(0.5);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_rsqrt_ph() {
        let a = _mm256_set1_ph(4.0);
        let src = _mm256_set1_ph(1.0);
        let r = _mm256_mask_rsqrt_ph(src, 0b0101010101010101, a);
        let e = _mm256_set_ph(
            1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_rsqrt_ph() {
        let a = _mm256_set1_ph(4.0);
        let r = _mm256_maskz_rsqrt_ph(0b0101010101010101, a);
        let e = _mm256_set_ph(
            0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_rsqrt_ph() {
        let a = _mm512_set1_ph(4.0);
        let r = _mm512_rsqrt_ph(a);
        let e = _mm512_set1_ph(0.5);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_rsqrt_ph() {
        let a = _mm512_set1_ph(4.0);
        let src = _mm512_set1_ph(1.0);
        let r = _mm512_mask_rsqrt_ph(src, 0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0,
            0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0, 0.5,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_rsqrt_ph() {
        let a = _mm512_set1_ph(4.0);
        let r = _mm512_maskz_rsqrt_ph(0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0,
            0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_rsqrt_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(4.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0);
        let r = _mm_rsqrt_sh(a, b);
        let e = _mm_setr_ph(0.5, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_rsqrt_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(4.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0);
        let src = _mm_setr_ph(3.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0);
        let r = _mm_mask_rsqrt_sh(src, 0, a, b);
        let e = _mm_setr_ph(3.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_rsqrt_sh(src, 1, a, b);
        let e = _mm_setr_ph(0.5, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_rsqrt_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(4.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0);
        let r = _mm_maskz_rsqrt_sh(0, a, b);
        let e = _mm_setr_ph(0.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_rsqrt_sh(1, a, b);
        let e = _mm_setr_ph(0.5, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_sqrt_ph() {
        let a = _mm_set1_ph(4.0);
        let r = _mm_sqrt_ph(a);
        let e = _mm_set1_ph(2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_sqrt_ph() {
        let a = _mm_set1_ph(4.0);
        let src = _mm_set1_ph(1.0);
        let r = _mm_mask_sqrt_ph(src, 0b01010101, a);
        let e = _mm_set_ph(1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_sqrt_ph() {
        let a = _mm_set1_ph(4.0);
        let r = _mm_maskz_sqrt_ph(0b01010101, a);
        let e = _mm_set_ph(0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_sqrt_ph() {
        let a = _mm256_set1_ph(4.0);
        let r = _mm256_sqrt_ph(a);
        let e = _mm256_set1_ph(2.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_sqrt_ph() {
        let a = _mm256_set1_ph(4.0);
        let src = _mm256_set1_ph(1.0);
        let r = _mm256_mask_sqrt_ph(src, 0b0101010101010101, a);
        let e = _mm256_set_ph(
            1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_sqrt_ph() {
        let a = _mm256_set1_ph(4.0);
        let r = _mm256_maskz_sqrt_ph(0b0101010101010101, a);
        let e = _mm256_set_ph(
            0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_sqrt_ph() {
        let a = _mm512_set1_ph(4.0);
        let r = _mm512_sqrt_ph(a);
        let e = _mm512_set1_ph(2.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_sqrt_ph() {
        let a = _mm512_set1_ph(4.0);
        let src = _mm512_set1_ph(1.0);
        let r = _mm512_mask_sqrt_ph(src, 0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0,
            2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_sqrt_ph() {
        let a = _mm512_set1_ph(4.0);
        let r = _mm512_maskz_sqrt_ph(0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0,
            2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_sqrt_round_ph() {
        let a = _mm512_set1_ph(4.0);
        let r = _mm512_sqrt_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a);
        let e = _mm512_set1_ph(2.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_sqrt_round_ph() {
        let a = _mm512_set1_ph(4.0);
        let src = _mm512_set1_ph(1.0);
        let r = _mm512_mask_sqrt_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0,
            2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_sqrt_round_ph() {
        let a = _mm512_set1_ph(4.0);
        let r = _mm512_maskz_sqrt_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0,
            2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_sqrt_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(4.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0);
        let r = _mm_sqrt_sh(a, b);
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_sqrt_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(4.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0);
        let src = _mm_setr_ph(3.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0);
        let r = _mm_mask_sqrt_sh(src, 0, a, b);
        let e = _mm_setr_ph(3.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_sqrt_sh(src, 1, a, b);
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_sqrt_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(4.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0);
        let r = _mm_maskz_sqrt_sh(0, a, b);
        let e = _mm_setr_ph(0.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_sqrt_sh(1, a, b);
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_sqrt_round_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(4.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0);
        let r = _mm_sqrt_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_sqrt_round_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(4.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0);
        let src = _mm_setr_ph(3.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0);
        let r = _mm_mask_sqrt_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_setr_ph(3.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_sqrt_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 1, a, b,
        );
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_sqrt_round_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(4.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0);
        let r =
            _mm_maskz_sqrt_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_setr_ph(0.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r =
            _mm_maskz_sqrt_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(1, a, b);
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_max_ph() {
        let a = _mm_set1_ph(2.0);
        let b = _mm_set1_ph(1.0);
        let r = _mm_max_ph(a, b);
        let e = _mm_set1_ph(2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_max_ph() {
        let a = _mm_set1_ph(2.0);
        let b = _mm_set1_ph(1.0);
        let src = _mm_set1_ph(3.0);
        let r = _mm_mask_max_ph(src, 0b01010101, a, b);
        let e = _mm_set_ph(3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_max_ph() {
        let a = _mm_set1_ph(2.0);
        let b = _mm_set1_ph(1.0);
        let r = _mm_maskz_max_ph(0b01010101, a, b);
        let e = _mm_set_ph(0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_max_ph() {
        let a = _mm256_set1_ph(2.0);
        let b = _mm256_set1_ph(1.0);
        let r = _mm256_max_ph(a, b);
        let e = _mm256_set1_ph(2.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_max_ph() {
        let a = _mm256_set1_ph(2.0);
        let b = _mm256_set1_ph(1.0);
        let src = _mm256_set1_ph(3.0);
        let r = _mm256_mask_max_ph(src, 0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_max_ph() {
        let a = _mm256_set1_ph(2.0);
        let b = _mm256_set1_ph(1.0);
        let r = _mm256_maskz_max_ph(0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_max_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let r = _mm512_max_ph(a, b);
        let e = _mm512_set1_ph(2.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_max_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let src = _mm512_set1_ph(3.0);
        let r = _mm512_mask_max_ph(src, 0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0,
            2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_max_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let r = _mm512_maskz_max_ph(0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0,
            2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_max_round_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let r = _mm512_max_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm512_set1_ph(2.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_max_round_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let src = _mm512_set1_ph(3.0);
        let r = _mm512_mask_max_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0,
            2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_max_round_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let r = _mm512_maskz_max_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0,
            2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_max_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let r = _mm_max_sh(a, b);
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_max_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let src = _mm_setr_ph(3.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0);
        let r = _mm_mask_max_sh(src, 0, a, b);
        let e = _mm_setr_ph(3.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_max_sh(src, 1, a, b);
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_max_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let r = _mm_maskz_max_sh(0, a, b);
        let e = _mm_setr_ph(0.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_max_sh(1, a, b);
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_max_round_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let r = _mm_max_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_max_round_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let src = _mm_setr_ph(3.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0);
        let r = _mm_mask_max_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_setr_ph(3.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_max_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 1, a, b,
        );
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_max_round_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let r =
            _mm_maskz_max_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_setr_ph(0.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r =
            _mm_maskz_max_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(1, a, b);
        let e = _mm_setr_ph(2.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_min_ph() {
        let a = _mm_set1_ph(2.0);
        let b = _mm_set1_ph(1.0);
        let r = _mm_min_ph(a, b);
        let e = _mm_set1_ph(1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_min_ph() {
        let a = _mm_set1_ph(2.0);
        let b = _mm_set1_ph(1.0);
        let src = _mm_set1_ph(3.0);
        let r = _mm_mask_min_ph(src, 0b01010101, a, b);
        let e = _mm_set_ph(3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_min_ph() {
        let a = _mm_set1_ph(2.0);
        let b = _mm_set1_ph(1.0);
        let r = _mm_maskz_min_ph(0b01010101, a, b);
        let e = _mm_set_ph(0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_min_ph() {
        let a = _mm256_set1_ph(2.0);
        let b = _mm256_set1_ph(1.0);
        let r = _mm256_min_ph(a, b);
        let e = _mm256_set1_ph(1.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_min_ph() {
        let a = _mm256_set1_ph(2.0);
        let b = _mm256_set1_ph(1.0);
        let src = _mm256_set1_ph(3.0);
        let r = _mm256_mask_min_ph(src, 0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_min_ph() {
        let a = _mm256_set1_ph(2.0);
        let b = _mm256_set1_ph(1.0);
        let r = _mm256_maskz_min_ph(0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_min_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let r = _mm512_min_ph(a, b);
        let e = _mm512_set1_ph(1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_min_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let src = _mm512_set1_ph(3.0);
        let r = _mm512_mask_min_ph(src, 0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0,
            1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_min_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let r = _mm512_maskz_min_ph(0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_min_round_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let r = _mm512_min_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm512_set1_ph(1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_min_round_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let src = _mm512_set1_ph(3.0);
        let r = _mm512_mask_min_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0,
            1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_min_round_ph() {
        let a = _mm512_set1_ph(2.0);
        let b = _mm512_set1_ph(1.0);
        let r = _mm512_maskz_min_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_min_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let r = _mm_min_sh(a, b);
        let e = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_min_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let src = _mm_setr_ph(3.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0);
        let r = _mm_mask_min_sh(src, 0, a, b);
        let e = _mm_setr_ph(3.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_min_sh(src, 1, a, b);
        let e = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_min_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let r = _mm_maskz_min_sh(0, a, b);
        let e = _mm_setr_ph(0.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_min_sh(1, a, b);
        let e = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_min_round_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let r = _mm_min_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_min_round_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let src = _mm_setr_ph(3.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0);
        let r = _mm_mask_min_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_setr_ph(3.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r = _mm_mask_min_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 1, a, b,
        );
        let e = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_min_round_sh() {
        let a = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = _mm_setr_ph(2.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0);
        let r =
            _mm_maskz_min_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_setr_ph(0.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
        let r =
            _mm_maskz_min_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(1, a, b);
        let e = _mm_setr_ph(1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_getexp_ph() {
        let a = _mm_set1_ph(3.0);
        let r = _mm_getexp_ph(a);
        let e = _mm_set1_ph(1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_getexp_ph() {
        let a = _mm_set1_ph(3.0);
        let src = _mm_set1_ph(4.0);
        let r = _mm_mask_getexp_ph(src, 0b01010101, a);
        let e = _mm_set_ph(4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_getexp_ph() {
        let a = _mm_set1_ph(3.0);
        let r = _mm_maskz_getexp_ph(0b01010101, a);
        let e = _mm_set_ph(0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_getexp_ph() {
        let a = _mm256_set1_ph(3.0);
        let r = _mm256_getexp_ph(a);
        let e = _mm256_set1_ph(1.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_getexp_ph() {
        let a = _mm256_set1_ph(3.0);
        let src = _mm256_set1_ph(4.0);
        let r = _mm256_mask_getexp_ph(src, 0b0101010101010101, a);
        let e = _mm256_set_ph(
            4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_getexp_ph() {
        let a = _mm256_set1_ph(3.0);
        let r = _mm256_maskz_getexp_ph(0b0101010101010101, a);
        let e = _mm256_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_getexp_ph() {
        let a = _mm512_set1_ph(3.0);
        let r = _mm512_getexp_ph(a);
        let e = _mm512_set1_ph(1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_getexp_ph() {
        let a = _mm512_set1_ph(3.0);
        let src = _mm512_set1_ph(4.0);
        let r = _mm512_mask_getexp_ph(src, 0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0,
            1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_getexp_ph() {
        let a = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_getexp_ph(0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_getexp_round_ph() {
        let a = _mm512_set1_ph(3.0);
        let r = _mm512_getexp_round_ph::<_MM_FROUND_NO_EXC>(a);
        let e = _mm512_set1_ph(1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_getexp_round_ph() {
        let a = _mm512_set1_ph(3.0);
        let src = _mm512_set1_ph(4.0);
        let r = _mm512_mask_getexp_round_ph::<_MM_FROUND_NO_EXC>(
            src,
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0,
            1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0, 4.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_getexp_round_ph() {
        let a = _mm512_set1_ph(3.0);
        let r = _mm512_maskz_getexp_round_ph::<_MM_FROUND_NO_EXC>(
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_getexp_sh() {
        let a = _mm_setr_ph(4.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_getexp_sh(a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_getexp_sh() {
        let a = _mm_setr_ph(4.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let src = _mm_setr_ph(4.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_getexp_sh(src, 0, a, b);
        let e = _mm_setr_ph(4.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_getexp_sh(src, 1, a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_getexp_sh() {
        let a = _mm_setr_ph(4.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_maskz_getexp_sh(0, a, b);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_getexp_sh(1, a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_getexp_round_sh() {
        let a = _mm_setr_ph(4.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_getexp_round_sh::<_MM_FROUND_NO_EXC>(a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_getexp_round_sh() {
        let a = _mm_setr_ph(4.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let src = _mm_setr_ph(4.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_getexp_round_sh::<_MM_FROUND_NO_EXC>(src, 0, a, b);
        let e = _mm_setr_ph(4.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_getexp_round_sh::<_MM_FROUND_NO_EXC>(src, 1, a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_getexp_round_sh() {
        let a = _mm_setr_ph(4.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_maskz_getexp_round_sh::<_MM_FROUND_NO_EXC>(0, a, b);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_getexp_round_sh::<_MM_FROUND_NO_EXC>(1, a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_getmant_ph() {
        let a = _mm_set1_ph(10.0);
        let r = _mm_getmant_ph::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(a);
        let e = _mm_set1_ph(1.25);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_getmant_ph() {
        let a = _mm_set1_ph(10.0);
        let src = _mm_set1_ph(20.0);
        let r = _mm_mask_getmant_ph::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(src, 0b01010101, a);
        let e = _mm_set_ph(20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_getmant_ph() {
        let a = _mm_set1_ph(10.0);
        let r = _mm_maskz_getmant_ph::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(0b01010101, a);
        let e = _mm_set_ph(0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_getmant_ph() {
        let a = _mm256_set1_ph(10.0);
        let r = _mm256_getmant_ph::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(a);
        let e = _mm256_set1_ph(1.25);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_getmant_ph() {
        let a = _mm256_set1_ph(10.0);
        let src = _mm256_set1_ph(20.0);
        let r = _mm256_mask_getmant_ph::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(
            src,
            0b0101010101010101,
            a,
        );
        let e = _mm256_set_ph(
            20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25,
            20.0, 1.25,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_getmant_ph() {
        let a = _mm256_set1_ph(10.0);
        let r = _mm256_maskz_getmant_ph::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(
            0b0101010101010101,
            a,
        );
        let e = _mm256_set_ph(
            0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_getmant_ph() {
        let a = _mm512_set1_ph(10.0);
        let r = _mm512_getmant_ph::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(a);
        let e = _mm512_set1_ph(1.25);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_getmant_ph() {
        let a = _mm512_set1_ph(10.0);
        let src = _mm512_set1_ph(20.0);
        let r = _mm512_mask_getmant_ph::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(
            src,
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25,
            20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25,
            20.0, 1.25, 20.0, 1.25,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_getmant_ph() {
        let a = _mm512_set1_ph(10.0);
        let r = _mm512_maskz_getmant_ph::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25,
            0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_getmant_round_ph() {
        let a = _mm512_set1_ph(10.0);
        let r =
            _mm512_getmant_round_ph::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN, _MM_FROUND_NO_EXC>(
                a,
            );
        let e = _mm512_set1_ph(1.25);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_getmant_round_ph() {
        let a = _mm512_set1_ph(10.0);
        let src = _mm512_set1_ph(20.0);
        let r = _mm512_mask_getmant_round_ph::<
            _MM_MANT_NORM_P75_1P5,
            _MM_MANT_SIGN_NAN,
            _MM_FROUND_NO_EXC,
        >(src, 0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25,
            20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25, 20.0, 1.25,
            20.0, 1.25, 20.0, 1.25,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_getmant_round_ph() {
        let a = _mm512_set1_ph(10.0);
        let r = _mm512_maskz_getmant_round_ph::<
            _MM_MANT_NORM_P75_1P5,
            _MM_MANT_SIGN_NAN,
            _MM_FROUND_NO_EXC,
        >(0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25,
            0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25, 0.0, 1.25,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_getmant_sh() {
        let a = _mm_setr_ph(15.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(10.0, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_getmant_sh::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(a, b);
        let e = _mm_setr_ph(1.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_getmant_sh() {
        let a = _mm_setr_ph(15.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(10.0, 20., 21., 22., 23., 24., 25., 26.);
        let src = _mm_setr_ph(20.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_getmant_sh::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(src, 0, a, b);
        let e = _mm_setr_ph(20.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_getmant_sh::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(src, 1, a, b);
        let e = _mm_setr_ph(1.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_getmant_sh() {
        let a = _mm_setr_ph(15.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(10.0, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_maskz_getmant_sh::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(0, a, b);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_getmant_sh::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN>(1, a, b);
        let e = _mm_setr_ph(1.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_getmant_round_sh() {
        let a = _mm_setr_ph(15.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(10.0, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_getmant_round_sh::<_MM_MANT_NORM_P75_1P5, _MM_MANT_SIGN_NAN, _MM_FROUND_NO_EXC>(
            a, b,
        );
        let e = _mm_setr_ph(1.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_getmant_round_sh() {
        let a = _mm_setr_ph(15.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(10.0, 20., 21., 22., 23., 24., 25., 26.);
        let src = _mm_setr_ph(20.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_getmant_round_sh::<
            _MM_MANT_NORM_P75_1P5,
            _MM_MANT_SIGN_NAN,
            _MM_FROUND_NO_EXC,
        >(src, 0, a, b);
        let e = _mm_setr_ph(20.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_getmant_round_sh::<
            _MM_MANT_NORM_P75_1P5,
            _MM_MANT_SIGN_NAN,
            _MM_FROUND_NO_EXC,
        >(src, 1, a, b);
        let e = _mm_setr_ph(1.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_getmant_round_sh() {
        let a = _mm_setr_ph(15.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(10.0, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_maskz_getmant_round_sh::<
            _MM_MANT_NORM_P75_1P5,
            _MM_MANT_SIGN_NAN,
            _MM_FROUND_NO_EXC,
        >(0, a, b);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_getmant_round_sh::<
            _MM_MANT_NORM_P75_1P5,
            _MM_MANT_SIGN_NAN,
            _MM_FROUND_NO_EXC,
        >(1, a, b);
        let e = _mm_setr_ph(1.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_roundscale_ph() {
        let a = _mm_set1_ph(1.1);
        let r = _mm_roundscale_ph::<0>(a);
        let e = _mm_set1_ph(1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_roundscale_ph() {
        let a = _mm_set1_ph(1.1);
        let src = _mm_set1_ph(2.0);
        let r = _mm_mask_roundscale_ph::<0>(src, 0b01010101, a);
        let e = _mm_set_ph(2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_roundscale_ph() {
        let a = _mm_set1_ph(1.1);
        let r = _mm_maskz_roundscale_ph::<0>(0b01010101, a);
        let e = _mm_set_ph(0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_roundscale_ph() {
        let a = _mm256_set1_ph(1.1);
        let r = _mm256_roundscale_ph::<0>(a);
        let e = _mm256_set1_ph(1.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_roundscale_ph() {
        let a = _mm256_set1_ph(1.1);
        let src = _mm256_set1_ph(2.0);
        let r = _mm256_mask_roundscale_ph::<0>(src, 0b0101010101010101, a);
        let e = _mm256_set_ph(
            2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_roundscale_ph() {
        let a = _mm256_set1_ph(1.1);
        let r = _mm256_maskz_roundscale_ph::<0>(0b0101010101010101, a);
        let e = _mm256_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_roundscale_ph() {
        let a = _mm512_set1_ph(1.1);
        let r = _mm512_roundscale_ph::<0>(a);
        let e = _mm512_set1_ph(1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_roundscale_ph() {
        let a = _mm512_set1_ph(1.1);
        let src = _mm512_set1_ph(2.0);
        let r = _mm512_mask_roundscale_ph::<0>(src, 0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0,
            1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_roundscale_ph() {
        let a = _mm512_set1_ph(1.1);
        let r = _mm512_maskz_roundscale_ph::<0>(0b01010101010101010101010101010101, a);
        let e = _mm512_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_roundscale_round_ph() {
        let a = _mm512_set1_ph(1.1);
        let r = _mm512_roundscale_round_ph::<0, _MM_FROUND_NO_EXC>(a);
        let e = _mm512_set1_ph(1.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_roundscale_round_ph() {
        let a = _mm512_set1_ph(1.1);
        let src = _mm512_set1_ph(2.0);
        let r = _mm512_mask_roundscale_round_ph::<0, _MM_FROUND_NO_EXC>(
            src,
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0,
            1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_roundscale_round_ph() {
        let a = _mm512_set1_ph(1.1);
        let r = _mm512_maskz_roundscale_round_ph::<0, _MM_FROUND_NO_EXC>(
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_roundscale_sh() {
        let a = _mm_setr_ph(2.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.1, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_roundscale_sh::<0>(a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_roundscale_sh() {
        let a = _mm_setr_ph(2.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.1, 20., 21., 22., 23., 24., 25., 26.);
        let src = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_roundscale_sh::<0>(src, 0, a, b);
        let e = _mm_setr_ph(3.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_roundscale_sh::<0>(src, 1, a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_roundscale_sh() {
        let a = _mm_setr_ph(2.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.1, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_maskz_roundscale_sh::<0>(0, a, b);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_roundscale_sh::<0>(1, a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_roundscale_round_sh() {
        let a = _mm_setr_ph(2.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.1, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_roundscale_round_sh::<0, _MM_FROUND_NO_EXC>(a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_roundscale_round_sh() {
        let a = _mm_setr_ph(2.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.1, 20., 21., 22., 23., 24., 25., 26.);
        let src = _mm_setr_ph(3.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_roundscale_round_sh::<0, _MM_FROUND_NO_EXC>(src, 0, a, b);
        let e = _mm_setr_ph(3.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_roundscale_round_sh::<0, _MM_FROUND_NO_EXC>(src, 1, a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_roundscale_round_sh() {
        let a = _mm_setr_ph(2.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.1, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_maskz_roundscale_round_sh::<0, _MM_FROUND_NO_EXC>(0, a, b);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_roundscale_round_sh::<0, _MM_FROUND_NO_EXC>(1, a, b);
        let e = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_scalef_ph() {
        let a = _mm_set1_ph(1.);
        let b = _mm_set1_ph(3.);
        let r = _mm_scalef_ph(a, b);
        let e = _mm_set1_ph(8.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_scalef_ph() {
        let a = _mm_set1_ph(1.);
        let b = _mm_set1_ph(3.);
        let src = _mm_set1_ph(2.);
        let r = _mm_mask_scalef_ph(src, 0b01010101, a, b);
        let e = _mm_set_ph(2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_scalef_ph() {
        let a = _mm_set1_ph(1.);
        let b = _mm_set1_ph(3.);
        let r = _mm_maskz_scalef_ph(0b01010101, a, b);
        let e = _mm_set_ph(0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_scalef_ph() {
        let a = _mm256_set1_ph(1.);
        let b = _mm256_set1_ph(3.);
        let r = _mm256_scalef_ph(a, b);
        let e = _mm256_set1_ph(8.0);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_scalef_ph() {
        let a = _mm256_set1_ph(1.);
        let b = _mm256_set1_ph(3.);
        let src = _mm256_set1_ph(2.);
        let r = _mm256_mask_scalef_ph(src, 0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_scalef_ph() {
        let a = _mm256_set1_ph(1.);
        let b = _mm256_set1_ph(3.);
        let r = _mm256_maskz_scalef_ph(0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_scalef_ph() {
        let a = _mm512_set1_ph(1.);
        let b = _mm512_set1_ph(3.);
        let r = _mm512_scalef_ph(a, b);
        let e = _mm512_set1_ph(8.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_scalef_ph() {
        let a = _mm512_set1_ph(1.);
        let b = _mm512_set1_ph(3.);
        let src = _mm512_set1_ph(2.);
        let r = _mm512_mask_scalef_ph(src, 0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0,
            8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_scalef_ph() {
        let a = _mm512_set1_ph(1.);
        let b = _mm512_set1_ph(3.);
        let r = _mm512_maskz_scalef_ph(0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0,
            8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_scalef_round_ph() {
        let a = _mm512_set1_ph(1.);
        let b = _mm512_set1_ph(3.);
        let r = _mm512_scalef_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm512_set1_ph(8.0);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_scalef_round_ph() {
        let a = _mm512_set1_ph(1.);
        let b = _mm512_set1_ph(3.);
        let src = _mm512_set1_ph(2.);
        let r = _mm512_mask_scalef_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src,
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0,
            8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0, 2.0, 8.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_scalef_round_ph() {
        let a = _mm512_set1_ph(1.);
        let b = _mm512_set1_ph(3.);
        let r = _mm512_maskz_scalef_round_ph::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            0b01010101010101010101010101010101,
            a,
            b,
        );
        let e = _mm512_set_ph(
            0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0,
            8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0, 0.0, 8.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_scalef_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_scalef_sh(a, b);
        let e = _mm_setr_ph(8.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_scalef_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let src = _mm_setr_ph(2.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_scalef_sh(src, 0, a, b);
        let e = _mm_setr_ph(2.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_scalef_sh(src, 1, a, b);
        let e = _mm_setr_ph(8.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_scalef_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_maskz_scalef_sh(0, a, b);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_scalef_sh(1, a, b);
        let e = _mm_setr_ph(8.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_scalef_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_scalef_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(a, b);
        let e = _mm_setr_ph(8.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_scalef_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let src = _mm_setr_ph(2.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_scalef_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 0, a, b,
        );
        let e = _mm_setr_ph(2.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_scalef_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(
            src, 1, a, b,
        );
        let e = _mm_setr_ph(8.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_scalef_round_sh() {
        let a = _mm_setr_ph(1.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(3.0, 20., 21., 22., 23., 24., 25., 26.);
        let r =
            _mm_maskz_scalef_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(0, a, b);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r =
            _mm_maskz_scalef_round_sh::<{ _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC }>(1, a, b);
        let e = _mm_setr_ph(8.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_reduce_ph() {
        let a = _mm_set1_ph(1.25);
        let r = _mm_reduce_ph::<{ 16 | _MM_FROUND_TO_ZERO }>(a);
        let e = _mm_set1_ph(0.25);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_reduce_ph() {
        let a = _mm_set1_ph(1.25);
        let src = _mm_set1_ph(2.0);
        let r = _mm_mask_reduce_ph::<{ 16 | _MM_FROUND_TO_ZERO }>(src, 0b01010101, a);
        let e = _mm_set_ph(2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_maskz_reduce_ph() {
        let a = _mm_set1_ph(1.25);
        let r = _mm_maskz_reduce_ph::<{ 16 | _MM_FROUND_TO_ZERO }>(0b01010101, a);
        let e = _mm_set_ph(0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_reduce_ph() {
        let a = _mm256_set1_ph(1.25);
        let r = _mm256_reduce_ph::<{ 16 | _MM_FROUND_TO_ZERO }>(a);
        let e = _mm256_set1_ph(0.25);
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_reduce_ph() {
        let a = _mm256_set1_ph(1.25);
        let src = _mm256_set1_ph(2.0);
        let r = _mm256_mask_reduce_ph::<{ 16 | _MM_FROUND_TO_ZERO }>(src, 0b0101010101010101, a);
        let e = _mm256_set_ph(
            2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_maskz_reduce_ph() {
        let a = _mm256_set1_ph(1.25);
        let r = _mm256_maskz_reduce_ph::<{ 16 | _MM_FROUND_TO_ZERO }>(0b0101010101010101, a);
        let e = _mm256_set_ph(
            0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_reduce_ph() {
        let a = _mm512_set1_ph(1.25);
        let r = _mm512_reduce_ph::<{ 16 | _MM_FROUND_TO_ZERO }>(a);
        let e = _mm512_set1_ph(0.25);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_reduce_ph() {
        let a = _mm512_set1_ph(1.25);
        let src = _mm512_set1_ph(2.0);
        let r = _mm512_mask_reduce_ph::<{ 16 | _MM_FROUND_TO_ZERO }>(
            src,
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25,
            2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_reduce_ph() {
        let a = _mm512_set1_ph(1.25);
        let r = _mm512_maskz_reduce_ph::<{ 16 | _MM_FROUND_TO_ZERO }>(
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25,
            0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_reduce_round_ph() {
        let a = _mm512_set1_ph(1.25);
        let r = _mm512_reduce_round_ph::<{ 16 | _MM_FROUND_TO_ZERO }, _MM_FROUND_NO_EXC>(a);
        let e = _mm512_set1_ph(0.25);
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_reduce_round_ph() {
        let a = _mm512_set1_ph(1.25);
        let src = _mm512_set1_ph(2.0);
        let r = _mm512_mask_reduce_round_ph::<{ 16 | _MM_FROUND_TO_ZERO }, _MM_FROUND_NO_EXC>(
            src,
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25,
            2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25, 2.0, 0.25,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_maskz_reduce_round_ph() {
        let a = _mm512_set1_ph(1.25);
        let r = _mm512_maskz_reduce_round_ph::<{ 16 | _MM_FROUND_TO_ZERO }, _MM_FROUND_NO_EXC>(
            0b01010101010101010101010101010101,
            a,
        );
        let e = _mm512_set_ph(
            0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25,
            0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_reduce_sh() {
        let a = _mm_setr_ph(3.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.25, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_reduce_sh::<{ 16 | _MM_FROUND_TO_ZERO }>(a, b);
        let e = _mm_setr_ph(0.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_reduce_sh() {
        let a = _mm_setr_ph(3.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.25, 20., 21., 22., 23., 24., 25., 26.);
        let src = _mm_setr_ph(2.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_reduce_sh::<{ 16 | _MM_FROUND_TO_ZERO }>(src, 0, a, b);
        let e = _mm_setr_ph(2.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_reduce_sh::<{ 16 | _MM_FROUND_TO_ZERO }>(src, 1, a, b);
        let e = _mm_setr_ph(0.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_reduce_sh() {
        let a = _mm_setr_ph(3.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.25, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_maskz_reduce_sh::<{ 16 | _MM_FROUND_TO_ZERO }>(0, a, b);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_maskz_reduce_sh::<{ 16 | _MM_FROUND_TO_ZERO }>(1, a, b);
        let e = _mm_setr_ph(0.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_reduce_round_sh() {
        let a = _mm_setr_ph(3.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.25, 20., 21., 22., 23., 24., 25., 26.);
        let r = _mm_reduce_round_sh::<{ 16 | _MM_FROUND_TO_ZERO }, _MM_FROUND_NO_EXC>(a, b);
        let e = _mm_setr_ph(0.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_reduce_round_sh() {
        let a = _mm_setr_ph(3.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.25, 20., 21., 22., 23., 24., 25., 26.);
        let src = _mm_setr_ph(2.0, 30., 31., 32., 33., 34., 35., 36.);
        let r = _mm_mask_reduce_round_sh::<{ 16 | _MM_FROUND_TO_ZERO }, _MM_FROUND_NO_EXC>(
            src, 0, a, b,
        );
        let e = _mm_setr_ph(2.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r = _mm_mask_reduce_round_sh::<{ 16 | _MM_FROUND_TO_ZERO }, _MM_FROUND_NO_EXC>(
            src, 1, a, b,
        );
        let e = _mm_setr_ph(0.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_maskz_reduce_round_sh() {
        let a = _mm_setr_ph(3.0, 10., 11., 12., 13., 14., 15., 16.);
        let b = _mm_setr_ph(1.25, 20., 21., 22., 23., 24., 25., 26.);
        let r =
            _mm_maskz_reduce_round_sh::<{ 16 | _MM_FROUND_TO_ZERO }, _MM_FROUND_NO_EXC>(0, a, b);
        let e = _mm_setr_ph(0.0, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
        let r =
            _mm_maskz_reduce_round_sh::<{ 16 | _MM_FROUND_TO_ZERO }, _MM_FROUND_NO_EXC>(1, a, b);
        let e = _mm_setr_ph(0.25, 10., 11., 12., 13., 14., 15., 16.);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_reduce_add_ph() {
        let a = _mm_set1_ph(2.0);
        let r = _mm_reduce_add_ph(a);
        assert_eq!(r, 16.0);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_reduce_add_ph() {
        let a = _mm256_set1_ph(2.0);
        let r = _mm256_reduce_add_ph(a);
        assert_eq!(r, 32.0);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_reduce_add_ph() {
        let a = _mm512_set1_ph(2.0);
        let r = _mm512_reduce_add_ph(a);
        assert_eq!(r, 64.0);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_reduce_mul_ph() {
        let a = _mm_set1_ph(2.0);
        let r = _mm_reduce_mul_ph(a);
        assert_eq!(r, 256.0);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_reduce_mul_ph() {
        let a = _mm256_set1_ph(2.0);
        let r = _mm256_reduce_mul_ph(a);
        assert_eq!(r, 65536.0);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_reduce_mul_ph() {
        let a = _mm512_set1_ph(2.0);
        let r = _mm512_reduce_mul_ph(a);
        assert_eq!(r, 16777216.0);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_reduce_max_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let r = _mm_reduce_max_ph(a);
        assert_eq!(r, 8.0);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_reduce_max_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let r = _mm256_reduce_max_ph(a);
        assert_eq!(r, 16.0);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_reduce_max_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let r = _mm512_reduce_max_ph(a);
        assert_eq!(r, 32.0);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_reduce_min_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let r = _mm_reduce_min_ph(a);
        assert_eq!(r, 1.0);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_reduce_min_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let r = _mm256_reduce_min_ph(a);
        assert_eq!(r, 1.0);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_reduce_min_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let r = _mm512_reduce_min_ph(a);
        assert_eq!(r, 1.0);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_fpclass_ph_mask() {
        let a = _mm_set_ph(
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
        );
        let r = _mm_fpclass_ph_mask::<0x18>(a); // infinities
        assert_eq!(r, 0b01100000);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_fpclass_ph_mask() {
        let a = _mm_set_ph(
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
        );
        let r = _mm_mask_fpclass_ph_mask::<0x18>(0b01010101, a);
        assert_eq!(r, 0b01000000);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_fpclass_ph_mask() {
        let a = _mm256_set_ph(
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
        );
        let r = _mm256_fpclass_ph_mask::<0x18>(a); // infinities
        assert_eq!(r, 0b0110000001100000);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_fpclass_ph_mask() {
        let a = _mm256_set_ph(
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
        );
        let r = _mm256_mask_fpclass_ph_mask::<0x18>(0b0101010101010101, a);
        assert_eq!(r, 0b0100000001000000);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_fpclass_ph_mask() {
        let a = _mm512_set_ph(
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
        );
        let r = _mm512_fpclass_ph_mask::<0x18>(a); // infinities
        assert_eq!(r, 0b01100000011000000110000001100000);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_fpclass_ph_mask() {
        let a = _mm512_set_ph(
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
            1.,
            f16::INFINITY,
            f16::NEG_INFINITY,
            0.0,
            -0.0,
            -2.0,
            f16::NAN,
            5.9e-8, // Denormal
        );
        let r = _mm512_mask_fpclass_ph_mask::<0x18>(0b01010101010101010101010101010101, a);
        assert_eq!(r, 0b01000000010000000100000001000000);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_fpclass_sh_mask() {
        let a = _mm_set_sh(f16::INFINITY);
        let r = _mm_fpclass_sh_mask::<0x18>(a);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm_mask_fpclass_sh_mask() {
        let a = _mm_set_sh(f16::INFINITY);
        let r = _mm_mask_fpclass_sh_mask::<0x18>(0, a);
        assert_eq!(r, 0);
        let r = _mm_mask_fpclass_sh_mask::<0x18>(1, a);
        assert_eq!(r, 1);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_mask_blend_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_set_ph(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0);
        let r = _mm_mask_blend_ph(0b01010101, a, b);
        let e = _mm_set_ph(1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_mask_blend_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_set_ph(
            -1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -9.0, -10.0, -11.0, -12.0, -13.0,
            -14.0, -15.0, -16.0,
        );
        let r = _mm256_mask_blend_ph(0b0101010101010101, a, b);
        let e = _mm256_set_ph(
            1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0, 9.0, -10.0, 11.0, -12.0, 13.0, -14.0, 15.0,
            -16.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_mask_blend_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_set_ph(
            -1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -9.0, -10.0, -11.0, -12.0, -13.0,
            -14.0, -15.0, -16.0, -17.0, -18.0, -19.0, -20.0, -21.0, -22.0, -23.0, -24.0, -25.0,
            -26.0, -27.0, -28.0, -29.0, -30.0, -31.0, -32.0,
        );
        let r = _mm512_mask_blend_ph(0b01010101010101010101010101010101, a, b);
        let e = _mm512_set_ph(
            1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0, 9.0, -10.0, 11.0, -12.0, 13.0, -14.0, 15.0,
            -16.0, 17.0, -18.0, 19.0, -20.0, 21.0, -22.0, 23.0, -24.0, 25.0, -26.0, 27.0, -28.0,
            29.0, -30.0, 31.0, -32.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_permutex2var_ph() {
        let a = _mm_setr_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm_setr_ph(9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let idx = _mm_setr_epi16(0, 2, 4, 6, 8, 10, 12, 14);
        let r = _mm_permutex2var_ph(a, idx, b);
        let e = _mm_setr_ph(1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_permutex2var_ph() {
        let a = _mm256_setr_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let b = _mm256_setr_ph(
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let idx = _mm256_setr_epi16(0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30);
        let r = _mm256_permutex2var_ph(a, idx, b);
        let e = _mm256_setr_ph(
            1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0, 19.0, 21.0, 23.0, 25.0, 27.0, 29.0,
            31.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_permutex2var_ph() {
        let a = _mm512_setr_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let b = _mm512_setr_ph(
            33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0,
            47.0, 48.0, 49.0, 50.0, 51.0, 52.0, 53.0, 54.0, 55.0, 56.0, 57.0, 58.0, 59.0, 60.0,
            61.0, 62.0, 63.0, 64.0,
        );
        let idx = _mm512_set_epi16(
            62, 60, 58, 56, 54, 52, 50, 48, 46, 44, 42, 40, 38, 36, 34, 32, 30, 28, 26, 24, 22, 20,
            18, 16, 14, 12, 10, 8, 6, 4, 2, 0,
        );
        let r = _mm512_permutex2var_ph(a, idx, b);
        let e = _mm512_setr_ph(
            1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0, 19.0, 21.0, 23.0, 25.0, 27.0, 29.0,
            31.0, 33.0, 35.0, 37.0, 39.0, 41.0, 43.0, 45.0, 47.0, 49.0, 51.0, 53.0, 55.0, 57.0,
            59.0, 61.0, 63.0,
        );
        assert_eq_m512h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm_permutexvar_ph() {
        let a = _mm_set_ph(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let idx = _mm_set_epi16(0, 2, 4, 6, 1, 3, 5, 7);
        let r = _mm_permutexvar_ph(idx, a);
        let e = _mm_setr_ph(1.0, 3.0, 5.0, 7.0, 2.0, 4.0, 6.0, 8.0);
        assert_eq_m128h(r, e);
    }

    #[simd_test(enable = "avx512fp16,avx512vl")]
    unsafe fn test_mm256_permutexvar_ph() {
        let a = _mm256_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        let idx = _mm256_set_epi16(0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15);
        let r = _mm256_permutexvar_ph(idx, a);
        let e = _mm256_setr_ph(
            1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0,
        );
        assert_eq_m256h(r, e);
    }

    #[simd_test(enable = "avx512fp16")]
    unsafe fn test_mm512_permutexvar_ph() {
        let a = _mm512_set_ph(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        );
        let idx = _mm512_set_epi16(
            0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 1, 3, 5, 7, 9, 11, 13, 15,
            17, 19, 21, 23, 25, 27, 29, 31,
        );
        let r = _mm512_permutexvar_ph(idx, a);
        let e = _mm512_setr_ph(
            1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0, 19.0, 21.0, 23.0, 25.0, 27.0, 29.0,
            31.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0, 22.0, 24.0, 26.0, 28.0,
            30.0, 32.0,
        );
        assert_eq_m512h(r, e);
    }
}
