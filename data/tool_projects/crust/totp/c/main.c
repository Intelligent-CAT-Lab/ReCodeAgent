#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <time.h>
#include "totp.h"

int
main(int argc, char **argv)
{
	const char *seed;
	uint8_t key[64];

	if (argc != 2) {
		fprintf(stderr, "usage: totp [seed in base32]\n");
		return 64; /* EX_USAGE */
	}

	seed = argv[1];
	memset(key, 0, sizeof(key));

	if (!from_base32(seed, key, sizeof(key))) {
		fprintf(stderr, "invalid seed\n");
		return 64; /* EX_USAGE */
	}

	printf("%06d\n", totp(key, time(NULL)));
	return 0;
}
