#include "aes.h"

/*
 * theses functions are used to shift the rows of the state matrix
 * the Shift function shifts the rows to the left
 * the InvShift function shifts the rows to the right
 * the state matrix is a 4x4 matrix
 * the rows are shifted by the row number
 */


__attribute__((always_inline))
inline void Shift(uint8_t state[4][Nb]) {
    for (int i = 1; i < 4; i++) {
        uint8_t temp[Nb];
        for (int j = 0; j < Nb; j++)
            temp[j] = state[i][(j + i) % Nb];
		memcpy(state[i], temp, Nb);    
	}
	_mm_sfence();
}


__attribute__((always_inline))
inline void InvShift(uint8_t state[4][Nb]) {
    for (int i = 1; i < 4; i++) {
        uint8_t temp[Nb];
        for (int j = 0; j < Nb; j++) {
            temp[j] = state[i][(j - i + Nb) % Nb];
        }
		memcpy(state[i], temp, Nb);	
    }
	_mm_sfence();
}
