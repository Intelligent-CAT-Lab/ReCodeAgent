#if !defined CHECK_H
#define CHECK_H

#if !defined NDEBUG
#include <assert.h>

#define CHECK(x) \
	do { \
		assert(x); \
	} while(0)

#define DEBUGIN(stm) stm

#else /* !defined NDEBUG */

#define CHECK(x) ((void)0)

#define DEBUGIN(stm)

#endif /* !NDEBUG */

#endif /* !CHECK_H */

