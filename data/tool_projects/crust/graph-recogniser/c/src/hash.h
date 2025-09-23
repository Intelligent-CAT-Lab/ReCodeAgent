#if !defined HASH_H

#define HASH_H

#include <string.h> /* strcmp */

/**
 * No description avaiable
 */

typedef unsigned hash_t;

typedef const char *o_key_t;
static const o_key_t empty_key = NULL;

typedef const char *o_data_t;
static const o_data_t empty_data = NULL;

static const hash_t power = 131;
static const hash_t alternative_power = 171;
static const hash_t rehasher = 718841;

static inline hash_t hash_by_power(o_key_t p, hash_t power) {
	hash_t res = 0;
	while (*p)
		res = res * power + *(p++);
	return res;
}

static inline hash_t hash(o_key_t p) {
	return hash_by_power(p, power);
}

static inline hash_t alternative_hash(o_key_t p) {
	return hash_by_power(p, alternative_power);
}

static inline int compare_keys(o_key_t k1, o_key_t k2) {
	return strcmp(k1, k2);
}

static inline hash_t rehash(hash_t h) {
	return h + rehasher;
}

#endif /* HASH_H */

