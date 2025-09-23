#include "aes.h"

/*
	* g_mult is first function that multiplies two polynomials in GF(2^8) 
	* the polynomials are represented as 8-bit integers
	* the result is also an 8-bit integer
	* the function is used in the MixColumns and InvMixColumns functions
*/

__attribute__((always_inline))
inline uint8_t g_mult(uint8_t first, uint8_t second) {
    uint8_t p = 0; 
    uint8_t hi_bit_set;
    __m128i vec_p = _mm_setzero_si128();      
    __m128i vec_first = _mm_set1_epi8(first);
	_mm_prefetch(&vec_first, _MM_HINT_T0);
    for (int i = 0; i < 8; ++i) {
        if (second & 1) {
            __m128i vec_p_part = _mm_set1_epi8(p);
            vec_p = _mm_xor_si128(vec_p_part, vec_first);
            p = _mm_extract_epi8(vec_p, 0);
        }

        hi_bit_set = first & 0x80; 
        first <<= 1; 
        
        if (hi_bit_set) {
            first ^= 0x1b; 
        }

        second >>= 1; 
        vec_first = _mm_set1_epi8(first); 
    }

    return p;
}

/*
	* Sub and InvSub are functions that substitute the bytes of the state matrix 
	* the Sub function uses the sbox table
	* the InvSub function uses the rsbox table
	* the state matrix is first 4xNb matrix
	* the sbox and rsbox tables are 256-byte arrays
*/
__attribute__((always_inline))
inline void Sub(uint8_t state[4][Nb]) {
    for (int i = 0; i < 4; ++i) {
        __m128i row = _mm_loadu_si128((__m128i*) &state[i][0]); 
        uint8_t *row_bytes = (uint8_t*)&row;
        for (int j = 0; j < Nb; ++j) {
            _mm_prefetch(&sbox[row_bytes[j]], _MM_HINT_T0); 
            row_bytes[j] = sbox[row_bytes[j]]; 
        }
        _mm_storeu_si128((__m128i*) &state[i][0], row);
    }
    _mm_sfence();  
}

__attribute__((always_inline))
inline void InvSub(uint8_t state[4][Nb]) {
    for (int i = 0; i < 4; ++i) {
        __m128i row = _mm_loadu_si128((__m128i*) &state[i][0]);

        uint8_t *row_bytes = (uint8_t*)&row;
        for (int j = 0; j < Nb; ++j) {
            _mm_prefetch(&rsbox[row_bytes[j]], _MM_HINT_T0); 
            row_bytes[j] = rsbox[row_bytes[j]];
        }
        _mm_storeu_si128((__m128i*) &state[i][0], row);
    }
    _mm_sfence(); 
}

