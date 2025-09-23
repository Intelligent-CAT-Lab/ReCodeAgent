
#include "trusted_utils.h"
#include <assert.h>
#include <string.h>
#if IMPCHECK_WRITE_DIRECTIVES
#include "../writer.h"
#endif
#include <stdio.h>
#include <stdlib.h> // exit
#include <unistd.h> // getpid

char trusted_utils_msgstr[512] = "";

void trusted_utils_log(const char* msg) {
    printf("c [TRUSTED_CORE %i] %s\n", getpid(), msg);
}
void trusted_utils_log_err(const char* msg) {
    printf("c [TRUSTED_CORE %i] [ERROR] %s\n", getpid(), msg);
}

void trusted_utils_exit_eof() {
    trusted_utils_log("end-of-file - terminating");
    exit(0);
}
void exit_oom() {
    trusted_utils_log("allocation failed - terminating");
    exit(0);
}

bool begins_with(const char* str, const char* prefix) {
    u64 i = 0;
    while (true) {
        if (prefix[i] == '\0') return true;
        if (str[i] == '\0') return prefix[i] == '\0';
        if (str[i] != prefix[i]) return false;
        i++;
    }
}
void trusted_utils_try_match_arg(const char* arg, const char* opt, const char** out) {
    if (begins_with(arg, opt)) *out = arg+strlen(opt);
}
void trusted_utils_try_match_flag(const char* arg, const char* opt, bool* out) {
    if (begins_with(arg, opt)) *out = true;
}

void trusted_utils_copy_bytes(u8* to, const u8* from, u64 nb_bytes) {
    for (u64 i = 0; i < nb_bytes; i++) to[i] = from[i];
}

bool trusted_utils_equal_signatures(const u8* left, const u8* right) {
    for (u64 i = 0; i < SIG_SIZE_BYTES; i++)
        if (left[i] != right[i]) return false;
    return true;
}

void* trusted_utils_malloc(u64 size) {
    void* res = malloc(size);
    if (!res) exit_oom();
    return res;
}
void* trusted_utils_realloc(void* from, u64 new_size) {
    void* res = realloc(from, new_size);
    if (!res) exit_oom();
    return res;
}
void* trusted_utils_calloc(u64 nb_objs, u64 size_per_obj) {
    void* res = calloc(nb_objs, size_per_obj);
    if (!res) exit_oom();
    return res;
}

bool trusted_utils_read_bool(FILE* file) {
    int res = UNLOCKED_IO(fgetc)(file);
    if (res == EOF) trusted_utils_exit_eof();
#ifdef IMPCHECK_WRITE_DIRECTIVES
    write_bool(res ? 1 : 0);
#endif
    return res ? 1 : 0;
}
int trusted_utils_read_char(FILE* file) {
    int res = UNLOCKED_IO(fgetc)(file);
    if (res == EOF) trusted_utils_exit_eof();
#ifdef IMPCHECK_WRITE_DIRECTIVES
    write_char(res);
#endif
    return res;
}
void trusted_utils_read_objs(void* data, size_t size, size_t nb_objs, FILE* file) {
    u64 nb_read = UNLOCKED_IO(fread)(data, size, nb_objs, file);
    if (nb_read < nb_objs) trusted_utils_exit_eof();
}
int trusted_utils_read_int(FILE* file) {
    int i;
    trusted_utils_read_objs(&i, sizeof(int), 1, file);
#ifdef IMPCHECK_WRITE_DIRECTIVES
    write_int(i);
#endif
    return i;
}
void trusted_utils_read_ints(int* data, u64 nb_ints, FILE* file) {
    trusted_utils_read_objs(data, sizeof(int), nb_ints, file);
#ifdef IMPCHECK_WRITE_DIRECTIVES
    write_ints(data, nb_ints);
#endif
}
u64 trusted_utils_read_ul(FILE* file) {
    u64 u;
    trusted_utils_read_objs(&u, sizeof(u64), 1, file);
#ifdef IMPCHECK_WRITE_DIRECTIVES
    write_ul(u);
#endif
    return u;
}
void trusted_utils_read_uls(u64* data, u64 nb_uls, FILE* file) {
    trusted_utils_read_objs(data, sizeof(u64), nb_uls, file);
#ifdef IMPCHECK_WRITE_DIRECTIVES
    write_uls(data, nb_uls);
#endif
}
void trusted_utils_read_sig(u8* out_sig, FILE* file) {
    signature dummy;
    if (!out_sig) out_sig = dummy;
    trusted_utils_read_objs(out_sig, sizeof(int), 4, file);
#ifdef IMPCHECK_WRITE_DIRECTIVES
    write_sig(out_sig);
#endif
}

void trusted_utils_write_char(char c, FILE* file) {
    int res = UNLOCKED_IO(fputc)(c, file);
    if (res == EOF) trusted_utils_exit_eof();
}
void trusted_utils_write_bool(bool b, FILE* file) {
    int res = UNLOCKED_IO(fputc)(b ? 1 : 0, file);
    if (res == EOF) trusted_utils_exit_eof();
}
void write_objs(const void* data, size_t size, size_t nb_objs, FILE* file) {
    u64 nb_read = UNLOCKED_IO(fwrite)(data, size, nb_objs, file);
    if (nb_read < nb_objs) trusted_utils_exit_eof();
}
void trusted_utils_write_int(int i, FILE* file) {
    write_objs(&i, sizeof(int), 1, file);
}
void trusted_utils_write_ints(const int* data, u64 nb_ints, FILE* file) {
    write_objs(data, sizeof(int), nb_ints, file);
}
void trusted_utils_write_ul(u64 u, FILE* file) {
    write_objs(&u, sizeof(u64), 1, file);
}
void trusted_utils_write_uls(const u64* data, u64 nb_uls, FILE* file) {
    write_objs(data, sizeof(u64), nb_uls, file);
}
void trusted_utils_write_sig(const u8* sig, FILE* file) {
    write_objs(sig, sizeof(int), 4, file);
}

void trusted_utils_sig_to_str(const u8* sig, char* out) {
    for (int charpos = 0; charpos < SIG_SIZE_BYTES; charpos++) {
        char val1 = (sig[charpos] >> 4) & 0x0f;
        char val2 = sig[charpos] & 0x0f;
        assert(val1 >= 0 && val1 < 16);
        assert(val2 >= 0 && val2 < 16);
        out[2*charpos+0] = val1>=10 ? 'a'+val1-10 : '0'+val1;
        out[2*charpos+1] = val2>=10 ? 'a'+val2-10 : '0'+val2;
    }
    out[SIG_SIZE_BYTES*2] = '\0';
}

bool trusted_utils_str_to_sig(const char* str, u8* out) {
    for (int bytepos = 0; bytepos < SIG_SIZE_BYTES; bytepos++) {
        const char* hex_pair = str + bytepos*2;
        char hex1 = hex_pair[0]; char hex2 = hex_pair[1];
        int byte = 16 * (hex1 >= '0' && hex1 <= '9' ? hex1-'0' : 10+hex1-'a')
                      + (hex2 >= '0' && hex2 <= '9' ? hex2-'0' : 10+hex2-'a');
        if (byte < 0 || byte >= 256) return false;
        out[bytepos] = (u8) byte;
    }
    return true;
}
