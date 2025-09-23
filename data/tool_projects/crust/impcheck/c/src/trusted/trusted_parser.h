
#pragma once

#include <stdbool.h>        // for bool
#include <stdio.h>          // for FILE
#include "trusted_utils.h"  // for u8

void tp_init(const char* filename, FILE* out);
bool tp_parse(u8** sig);
void tp_end();
