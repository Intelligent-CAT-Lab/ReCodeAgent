#include "utils.h"

char *strcpy_to_heap(const char *str) {
	char *ptr = (char*)malloc(strlen(str) + 1);
	if (ptr == NULL)
		return NULL;

	strcpy(ptr, str);
	return ptr;
}
