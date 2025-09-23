#include "aes.h"

void Add(uint8_t state[4][Nb], uint8_t RoundKey[4][Nb]) {
	for (int i = 0; i < 4; ++i)
		for (int j = 0; j < Nb; ++j)
		{
			_mm_prefetch(&RoundKey[i][j], _MM_HINT_T0);
			state[i][j] ^= RoundKey[i][j];
		}
}

void Expansion(uint8_t Key[4 * Nk], uint8_t w[4 * Nb * (Nr + 1)]) {
	int i;
	__m128i temp;
	uint8_t temp_bytes[16] = {0};
	memset(w, 0, 4 * Nb * (Nr + 1));
	for (i = 0; i < Nk * 4; i += 16) {
		_mm_prefetch(&Key[i] + 16, _MM_HINT_T0);
		temp = _mm_loadu_si128((__m128i*)&Key[i]);
		_mm_storeu_si128((__m128i*)&w[i], temp);
		_mm_storeu_si128((__m128i*)&w[i + 4], temp);
		_mm_storeu_si128((__m128i*)&w[i + 8], temp);
		_mm_storeu_si128((__m128i*)&w[i + 12], temp);
	}

	i = Nk;
	while (i < Nb * (Nr + 1)) {
		temp = _mm_loadu_si128((__m128i*)&w[4 * (i - 1)]);
		if (i % Nk == 0) {
			temp = _mm_shuffle_epi32(temp, _MM_SHUFFLE(3, 0, 1, 2));
			_mm_storeu_si128((__m128i*)temp_bytes, temp);
			for (int j = 0; j < 4; j++) {
				_mm_prefetch(&sbox[temp_bytes[j]], _MM_HINT_T0);
				temp_bytes[j] = sbox[temp_bytes[j]];
			}
			temp_bytes[0] ^= Rcon[i / Nk];
			temp = _mm_loadu_si128((__m128i*)temp_bytes);
		} else if (Nk > 6 && (i % Nk == 4)) {
			_mm_storeu_si128((__m128i*)temp_bytes, temp);
			for (int j = 0; j < 4; j++) {
				_mm_prefetch(&sbox[temp_bytes[j]], _MM_HINT_T0);
				temp_bytes[j] = sbox[temp_bytes[j]];
				temp_bytes[j + 1] = sbox[temp_bytes[j + 1]];
				temp_bytes[j + 2] = sbox[temp_bytes[j + 2]];
				temp_bytes[j + 3] = sbox[temp_bytes[j + 3]];
			}
			temp = _mm_loadu_si128((__m128i*)temp_bytes);
		}

		__m128i w_i_Nk = _mm_loadu_si128((__m128i*)&w[4 * (i - Nk)]);
		temp = _mm_xor_si128(temp, w_i_Nk);
		_mm_storeu_si128((__m128i*)&w[4 * i], temp);

		i += 4; 
	}
}
