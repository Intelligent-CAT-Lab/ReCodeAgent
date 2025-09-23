
#pragma once

#include "trusted/trusted_utils.h"

#if IMPCHECK_WRITE_DIRECTIVES
void writer_init(char* output_path);
void write_bool(bool b);
void write_char(int c_int);
void write_int(int i);
void write_ints(int* data, u64 nb_ints);
void write_ul(u64 ul);
void write_uls(u64* data, u64 nb_uls);
void write_sig(u8* sig);
#endif
