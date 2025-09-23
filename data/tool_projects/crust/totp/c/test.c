#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>
#include "totp.h"

static void
to_hex(const uint8_t *a, size_t len, char *buf)
{
	size_t i;

	for (i=0; i<len; i++) {
		buf[i*2]   = "0123456789abcdef"[a[i] >> 4];
		buf[i*2+1] = "0123456789abcdef"[a[i] & 0xF];
	}

	buf[len*2] = '\0';
}

static void
test_pack(void)
{
	uint8_t a[8];

	unpack32(0x12345678, a);
	assert(a[0] == 0x12);
	assert(a[1] == 0x34);
	assert(a[2] == 0x56);
	assert(a[3] == 0x78);

	unpack64(0x123456789ABCDEF0, a);
	assert(a[0] == 0x12);
	assert(a[1] == 0x34);
	assert(a[2] == 0x56);
	assert(a[3] == 0x78);
	assert(a[4] == 0x9A);
	assert(a[5] == 0xBC);
	assert(a[6] == 0xDE);
	assert(a[7] == 0xF0);

	assert(pack32(a) == 0x12345678);
}

static void
test_sha1(void)
{
	uint8_t buf[512], hash[20];
	char str[41];

	buf[0] = 0;
	sha1(buf, 0, sizeof(buf), hash);
	to_hex(hash, 20, str);
	assert(!strcmp(str,
	    "da39a3ee5e6b4b0d3255bfef95601890afd80709"));

	snprintf((char *)buf, sizeof(buf), "%s", "abc");
	sha1(buf, strlen((char *)buf), sizeof(buf), hash);
	to_hex(hash, 20, str);
	assert(!strcmp(str,
	    "a9993e364706816aba3e25717850c26c9cd0d89d"));

	snprintf((char *)buf, sizeof(buf), "%s",
	    "The quick brown fox jumps over the lazy dog");
	sha1(buf, strlen((char *)buf), sizeof(buf), hash);
	to_hex(hash, 20, str);
	assert(!strcmp(str,
	    "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12"));
}

static void
test_hmac_sha1(void)
{
	uint8_t key[64], text[64], hash[20];
	char str[41];
	size_t i;

	/* RFC 2202 */
	memset(key, 0, sizeof(key));
	memset(text, 0, sizeof(text));
	for (i=0; i<20; i++) key[i] = 0xAA;
	for (i=0; i<50; i++) text[i] = 0xDD;

	hmac_sha1(key, text, 50, hash);
	to_hex(hash, 20, str);
	assert(!strcmp(str,
	    "125d7342b9ac11cd91a39af48aa17b4f63f175d3"));
}

static void
test_hotp(void)
{
	/* Appendix D */
	static const uint8_t secret[64] = {
	    0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
	    0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
	    0x37, 0x38, 0x39, 0x30 };
	
	assert(hotp(secret, 0) == 755224);
	assert(hotp(secret, 1) == 287082);
	assert(hotp(secret, 2) == 359152);
}

static void
test_from_base32(void)
{
	uint8_t buf[10];

	assert(from_base32("MZxw6===", buf, sizeof(buf)) == 3);
	assert(from_base32("MZxw6YQ=", buf, sizeof(buf)) == 4);
	assert(from_base32("MZxw6YTB", buf, sizeof(buf)) == 5);
	assert(from_base32("MZxw6YTBOI======", buf, sizeof(buf)) == 6);

	assert(!strncmp("foobar", (char *)buf, 6));
}

int
main()
{
	test_pack();
	test_sha1();
	test_hmac_sha1();
	test_hotp();
	test_from_base32();
	printf("All tests passed.\n");
	return 0;
}
