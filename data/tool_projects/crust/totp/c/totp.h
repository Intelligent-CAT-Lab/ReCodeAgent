#define TOTP_EXPORT	__attribute__((visibility("default")))

enum {
	TOTP_OK,
	TOTP_EBOUNDS	/* argument out-of-bounds, overflow */
};

/* convert u32 to 4 bytes, big-endian */
static inline void
unpack32(uint32_t x, uint8_t a[4])
{
	a[0] = (uint8_t)(x >> 24);
	a[1] = (uint8_t)(x >> 16);
	a[2] = (uint8_t)(x >> 8);
	a[3] = (uint8_t)x;
}

/* convert u64 to 8 bytes, big-endian */
static inline void
unpack64(uint64_t x, uint8_t a[8])
{
	unpack32((uint32_t)(x >> 32), &a[0]);
	unpack32((uint32_t)x, &a[4]);
}

/* convert 4 bytes to u32, big-endian */
static inline uint32_t
pack32(const uint8_t a[4])
{
	return (a[0] << 24) | (a[1] << 16) | (a[2] << 8) | a[3];
}

/* FIPS 180-3 2.2.2 */
static inline uint32_t
rotl(uint32_t x, int n)
{
	return x << n | x >> (32-n);
}

/*
 * FIPS 180-3
 *
 * Parameters:
 *   buf  - input buffer, clobbered, allow for 128 bytes extra
 *   len  - len of buf, in bytes
 *   cap  - capacity of buf, in bytes
 *   hash - output buffer
 *
 * Returns TOTP_OK on success, TOTP_* on error
 */
TOTP_EXPORT int sha1(uint8_t *buf, size_t len, size_t cap,
    uint8_t hash[20]);

/*
 * RFC 2104
 *
 * Parameters:
 *   key   - zero-padded key
 *   data  - up to 64 bytes of data
 *   len   - len of data
 *   hash  - output buffer
 *
 * Returns TOTP_OK on success, TOTP_* on error
 */
TOTP_EXPORT int hmac_sha1(const uint8_t key[64], const uint8_t *data,
    size_t len, uint8_t hash[20]);

/*
 * RFC 4226
 *
 * Parameters
 *   key      - zero-padded shared secret
 *   counter
 *
 * Returns HOTP code or -1 on error.
 */
TOTP_EXPORT int hotp(const uint8_t key[64], uint64_t counter);

/*
 * RFC 6238
 *
 * Parameters
 *   key  - zero-padded shared secret
 *   time - Unix time
 *
 * Returns HOTP code or -1 on error.
 */
TOTP_EXPORT int totp(const uint8_t key[64], uint64_t time);

/*
 * RFC 4648
 *
 * Parameters:
 *   s   - input base32 string, multiple of 8 length
 *   buf - output buffer, at least 5 bytes for every 8 in s
 *   cap - capacity of buf, in bytes
 *
 * Returns number of bytes written to buf, or 0 for invalid base32.
 */
TOTP_EXPORT size_t from_base32(const char *s, uint8_t *buf, size_t cap);
