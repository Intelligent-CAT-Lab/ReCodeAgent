/* libc definitions for freestanding targets (WASM, GBA) */

typedef unsigned char		uint8_t;
typedef unsigned int		uint32_t;
typedef unsigned long long	uint64_t;
typedef unsigned long		size_t;

#define SIZE_MAX	(~(size_t)0)

void *memset(void *s, int c, size_t n);
void *memcpy(void *dst, const void *src, size_t n);
size_t strlen(const char *s);
