#include <stdio.h>
#include <stdarg.h>

#if defined NLOG
#error you must exclude this file from the makefile use no-logs configuration
#endif /* NLOG */

#include "log.h"

#include "check.h" /* CHECK */

/**
 *  log_info_table:
 *  A table which provides information about how, when and where
 *  each type of log should be printed.
 */

static struct log_info {
	unsigned priority;
	char prefix[0x7f];
	char suffix[0x7f];
	/* not implemented */
	/* FILE *file; */
} log_info_table[HASH_TYPE_COUNT] = {
	/* log type */         /* priority */ /* prefix*/               /* suffix */
	/* LOG_LOG */           { 0,             "log log:\t",             "\n", },
	/* UNIT_TEST_LOG */     { 0,             "unit testing:\t",        "\n", },
	/* OPEN_HASH_LOG */     { 0,             "open hash table:\t",     "\n", },
	/* CUCKOO_HASH_LOG */   { 0,             "cuckoo hash table:\t",   "\n", },
};

static inline void print_log(FILE *file, const char *format, va_list args) {
	vfprintf(file, format, args);
}

unsigned change_log_priority(unsigned log_type, unsigned new_priority) {
	unsigned old_priority = log_info_table[log_type].priority;
	log_info_table[log_type].priority = new_priority;
	return old_priority;
}

void insert_log(unsigned log_type, unsigned priority, const char *format, ...) {
	CHECK(log_type < HASH_TYPE_COUNT && priority <= 10);

	struct log_info *cur_log_info = &log_info_table[log_type];

	if (cur_log_info->priority <= priority) {
		char message[0xff];
		sprintf(message, "%s%s%s",
			cur_log_info->prefix,
			format,
			cur_log_info->suffix
		);

		va_list args;
		va_start(args, format);
		print_log(stdout, message, args);
		va_end(args);
	}
}

