#ifndef LJMM_H
#define LJMM_H

#include <unistd.h>

int ljmm_init(void);

/* OS take care the [1G..2G] space */
void ljmm_let_OS_take_care_1G_2G(int turn_on);

void ljmm_test_set_test_param(const char* map_file, void *sbrk0, int page_size);

#endif

