
#include "trusted/trusted_utils.h"
#include <stdio.h>
#include <assert.h>

#if IMPCHECK_WRITE_DIRECTIVES
FILE* f_writer = 0;

void writer_init(char* output_path) {
    f_writer = fopen(output_path, "w");
    if (!f_writer) trusted_utils_exit_eof();
}

void write_bool(bool b) {
    if (!f_writer) return;
#if IMPCHECK_WRITE_DIRECTIVES == 1
    trusted_utils_write_bool(b, f_writer);
#else
    fprintf(f_writer, "%i ", b ? 1 : 0);
#endif
}
void write_char(int c_int) {
    if (!f_writer) return;
#if IMPCHECK_WRITE_DIRECTIVES == 1
    trusted_utils_write_char(c_int, f_writer);
#else
    fprintf(f_writer, "\n%c ", c_int);
#endif
}
void write_int(int i) {
    if (!f_writer) return;
#if IMPCHECK_WRITE_DIRECTIVES == 1
    trusted_utils_write_int(i, f_writer);
#else
    fprintf(f_writer, "%i ", i);
#endif
}
void write_ints(int* data, u64 nb_ints) {
    if (!f_writer) return;
#if IMPCHECK_WRITE_DIRECTIVES == 1
    trusted_utils_write_ints(data, nb_ints, f_writer);
#else
    for (u64 i = 0; i < nb_ints; i++)
        fprintf(f_writer, "%i ", data[i]);
#endif
}
void write_ul(u64 ul) {
    if (!f_writer) return;
#if IMPCHECK_WRITE_DIRECTIVES == 1
    trusted_utils_write_ul(ul, f_writer);
#else
    fprintf(f_writer, "%lu ", ul);
#endif
}
void write_uls(u64* data, u64 nb_uls) {
    if (!f_writer) return;
#if IMPCHECK_WRITE_DIRECTIVES == 1
    trusted_utils_write_uls(data, nb_uls, f_writer);
#else
    for (u64 i = 0; i < nb_uls; i++)
        fprintf(f_writer, "%lu ", data[i]);
#endif
}
void write_sig(u8* sig) {
    if (!f_writer) return;
#if IMPCHECK_WRITE_DIRECTIVES == 1
    trusted_utils_write_sig(sig, f_writer);
#else
    char out[SIG_SIZE_BYTES*2 + 1];
    trusted_utils_sig_to_str(sig, out);
    fprintf(f_writer, "%s", out);
#endif
}
#endif
