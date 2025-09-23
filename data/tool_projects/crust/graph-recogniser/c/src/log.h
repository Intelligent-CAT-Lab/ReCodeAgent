#if !defined LOG_H
#define LOG_H

#if !defined NLOG

/**
 * Log types:
 * Those numbers are the differnet types of logs.
 * A log type must be specified for each log() call.
 * The priority for each log can be adjusted so that
 * a certan type can be filtered or even excluded.
 */

#define LOG_LOG           0
#define UNIT_TEST_LOG     1
#define OPEN_HASH_LOG     2
#define CUCKOO_HASH_LOG   3
#define HASH_TYPE_COUNT   4

/**
 * Priorities:
 * Select a priority between 0 and 10.
 * Priority 0: debug log for the module.
 * Priority 4-7: generall description of the work.
 * Priority 10: critical event. 
 * If the threshold is more than 10 nothing is logged.
 */

void insert_log(unsigned log_type, unsigned priority, const char *format, ...);

unsigned change_log_priority(unsigned log_type, unsigned new_priority);

#else /* !defined NLOG */

#define insert_log(...) ((void)0)

#endif /* !NLOG */

#endif /* !LOG_H */

