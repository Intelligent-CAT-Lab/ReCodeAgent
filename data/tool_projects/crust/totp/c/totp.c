#ifdef NO_STD
# include "std.h"
#else
# include <stddef.h>
# include <stdint.h>
# include <string.h>
#endif

#include "totp.h"

/* FIPS 180-3 6.1.1 */
int
sha1(uint8_t *buf, size_t len, size_t cap, uint8_t hash[20])
{
	/* 4.2.1, use k[t/20] */
	static const uint32_t k[] = { 0x5A827999, 0x6ED9EBA1,
	    0x8F1BBCDC, 0xCA62C1D6 };

	size_t new_len, i,t;		/* new_len = after padding */
	uint32_t h[5], w[80];		/* hash values, msg schedule */
	uint32_t a,b,c,d,e, f, T;	/* working variables */

	/* 5.1.1 (padding) */

	/* add 1 byte for stop bit, 8 for length, pad to 64 bytes */
	if (len > SIZE_MAX-9-63)
		return TOTP_EBOUNDS;
	new_len = (len+9+63)/64*64;	/* ceil len+9 to 64 multiple */
	if (new_len > cap)
		return TOTP_EBOUNDS;

	memset(buf+len, 0, new_len-len);
	buf[len] = 1<<7;
	unpack64(len*8, &buf[new_len-8]);

	/* 5.3.1 */
	h[0] = 0x67452301; h[1] = 0xEFCDAB89; h[2] = 0x98BADCFE;
	h[3] = 0x10325476; h[4] = 0xC3D2E1F0;

	/* 6.1.2 */
	for (i=0; i < new_len/64; i++) {
		for (t=0; t<16; t++)
			w[t] = pack32(&buf[i*64 + t*4]);
		for (; t<80; t++)
			w[t] = rotl(w[t-3]^w[t-8]^w[t-14]^w[t-16], 1);

		a=h[0]; b=h[1]; c=h[2]; d=h[3]; e=h[4];

		for (t=0; t<80; t++) {
			/* 4.1.1 (f function) */
			f = t < 20 ? (b&c) ^ (~b&d) :
			    t < 40 ? b^c^d :
			    t < 60 ? (b&c) ^ (b&d) ^ (c&d) : b^c^d;

			T = rotl(a,5) + f + e + k[t/20] + w[t];
			e = d; d = c; c = rotl(b,30);
			b = a; a = T;
		}

		h[0] += a; h[1] += b; h[2] += c; h[3] += d; h[4] += e;
	}

	for (i=0; i<5; i++)
		unpack32(h[i], &hash[i*4]);

	return TOTP_OK;
}

int
hmac_sha1(const uint8_t key[64], const uint8_t *data, size_t len,
    uint8_t hash[20])
{
	uint8_t buf[196];
	size_t i;

	if (len > 64)
		return TOTP_EBOUNDS;

	for (i=0; i<64; i++) buf[i] = key[i] ^ 0x36;
	memcpy(&buf[64], data, len);
	sha1(buf, 64+len, sizeof(buf), hash);

	for (i=0; i<64; i++) buf[i] = key[i] ^ 0x5C;
	memcpy(&buf[64], hash, 20);
	sha1(buf, 64+20, sizeof(buf), hash);

	return TOTP_OK;
}

int
hotp(const uint8_t key[64], uint64_t counter)
{
	uint8_t data[8], hash[20];
	uint32_t trunc;

	unpack64(counter, data);
	hmac_sha1(key, data, 8, hash);

	trunc = pack32(&hash[hash[19] & 0xF]) & 0x7FFFFFFF;

	return (int)(trunc % 1000000);
}

int
totp(const uint8_t key[64], uint64_t time)
{
	return hotp(key, time / 30);
}

size_t
from_base32(const char *s, uint8_t *buf, size_t cap)
{
	size_t i,j;
	uint8_t v[8];
	char c;

	if (strlen(s) % 8)
		return 0;
	if (cap < (strlen(s)+1)/8*5)
		return 0;

	for (i=0; s[i*8]; i++) {
		for (j=0; j<8; j++)
			if ((c = s[i*8+j]) == '=') v[j] = 0;
			else if (c>='A' && c<='Z') v[j] = c-'A';
			else if (c>='a' && c<='z') v[j] = c-'a';
			else if (c>='2' && c<='7') v[j] = c-'2' + 26;
			else return 0;

		buf[i*5]   = (v[0] << 3) | (v[1] >> 2);
		buf[i*5+1] = (v[1] << 6) | (v[2] << 1) | (v[3] >> 4);
		buf[i*5+2] = (v[3] << 4) | (v[4] >> 1);
		buf[i*5+3] = (v[4] << 7) | (v[5] << 2) | (v[6] >> 3);
		buf[i*5+4] = (v[6] << 5) | v[7];

		if (s[i*8+2] == '=') return i*5 + 1;
		if (s[i*8+4] == '=') return i*5 + 2;
		if (s[i*8+5] == '=') return i*5 + 3;
		if (s[i*8+7] == '=') return i*5 + 4;
	}

	return i*5;
}
