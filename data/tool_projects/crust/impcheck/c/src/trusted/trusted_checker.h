
#pragma once

#include <stdbool.h>

void tc_init(const char* fifo_in, const char* fifo_out);
void tc_end();
int tc_run(bool check_model, bool lenient);
