#include "aes.h"

/*
	* the Cipher and InvCipher functions are the main functions of the AES algorithm
	* they encrypt and decrypt the input respectively
	* the input is a 4xNb matrix
	* the output is a 4xNb matrix
	* the key is a 4xNk matrix
	* the w matrix is the expanded key
	* the expanded key is a 4xNb*(Nr+1) matrix
	* the w matrix is generated using the KeyExpansion function
*/

void Cipher(uint8_t in[4 * Nb], uint8_t out[4 * Nb], uint8_t w[4 * Nb * (Nr + 1)]) {
	uint8_t state[4][Nb];
	for (int i = 0; i < 4 * Nb; ++i)
		state[i % 4][i / 4] = in[i];
	Add(state, (uint8_t(*)[Nb])&w[0]);
	for (int round = 1; round < Nr; ++round) {
		Sub(state);
		Shift(state);
		Columns(state);
		Add(state, (uint8_t(*)[Nb])&w[round * 4 * Nb]);
	}
	Sub(state);
	Shift(state);
	Add(state, (uint8_t(*)[Nb])&w[Nr * 4 * Nb]);
	for (int i = 0; i < 4 * Nb; ++i)
		out[i] = state[i % 4][i / 4];
}

void InvCipher(uint8_t in[4 * Nb], uint8_t out[4 * Nb], uint8_t w[4 * Nb * (Nr + 1)]) {
	uint8_t state[4][Nb];
	for (int i = 0; i < 4 * Nb; ++i)
		state[i % 4][i / 4] = in[i];
	Add(state, (uint8_t(*)[Nb])&w[Nr * 4 * Nb]);
	for (int round = Nr - 1; round > 0; --round) {
		InvShift(state);
		InvSub(state);
		Add(state, (uint8_t(*)[Nb])&w[round * 4 * Nb]);
		InvColumns(state);
	}
	InvShift(state);
	InvSub(state);
	Add(state, (uint8_t(*)[Nb])&w[0]);
	for (int i = 0; i < 4 * Nb; ++i)
		out[i] = state[i % 4][i / 4];
}
