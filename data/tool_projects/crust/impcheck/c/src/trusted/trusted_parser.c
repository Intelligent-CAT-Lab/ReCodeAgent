
#include <stdbool.h>        // for false, bool, true
#include <stdio.h>          // for FILE, fgetc_unlocked, fopen, EOF
#include <stdlib.h>         // for abort, free
#include "secret.h"         // for SECRET_KEY
#include "siphash.h"        // for siphash_digest, siphash_init, siphash_update
#include "trusted_utils.h"  // for trusted_utils_write_int, trusted_utils_wr...

// Instantiate int_vec
#define TYPE int
#define TYPED(THING) int_ ## THING
#include "vec.h"
#undef TYPED
#undef TYPE

FILE* f;
FILE* f_out;

struct int_vec* data;

bool comment = false;
bool header = false;
bool input_finished = false;
bool input_invalid = false;
bool began_num = false;

int num = 0;
int sign = 1;
int nb_read_cls = 0;

int nb_vars = -1;
int nb_cls = -1;


void output_literal_buffer() {
    siphash_update((unsigned char*) data->data, data->size * sizeof(int));
    trusted_utils_write_ints(data->data, data->size, f_out);
    int_vec_clear(data);
}

void append_integer() {
    if (header) {
        if (nb_vars == -1) {
            nb_vars = num;
            trusted_utils_write_int(nb_vars, f_out);
        } else if (nb_cls == -1) {
            nb_cls = num;
            trusted_utils_write_int(nb_cls, f_out);
            header = false;
        } else abort();
        num = 0;
        began_num = false;
        return;
    }

    const int lit = sign * num;
    if (lit == 0) nb_read_cls++;
    num = 0;
    sign = 1;
    began_num = false;

    if (data->size == data->capacity) output_literal_buffer();
    int_vec_push(data, lit);
}

bool process(char c) {

    if (comment && c != '\n' && c != '\r') return false;

    signed char uc = *((signed char*) &c);
    switch (uc) {
    case EOF:
        input_finished = true;
        return true;
    case '\n':
    case '\r':
        comment = false;
        if (began_num) append_integer();
        break;
    case 'p':
        header = true;
        break;
    case 'c':
        if (!header) comment = true;
        break;
    case ' ':
        if (began_num) append_integer();
        break;
    case '-':
        sign = -1;
        began_num = true;
        break;
    case '0': case '1': case '2': case '3': case '4':
    case '5': case '6': case '7': case '8': case '9':
        // Add digit to current number
        num = num*10 + (c-'0');
        began_num = true;
        break;
    default:
        break;
    }
    return false;
}


void tp_init(const char* filename, FILE* out) {
    siphash_init(SECRET_KEY);
    f = fopen(filename, "r");
    f_out = out;
    data = int_vec_init(TRUSTED_CHK_MAX_BUF_SIZE);
}

void tp_end() {
    free(data);
}

bool tp_parse(u8** sig) {
    while (true) {
        int c_int = UNLOCKED_IO(fgetc)(f);
        if (process((char) c_int)) break;
    }
    if (began_num) append_integer();
    if (data->size > 0) output_literal_buffer();
    siphash_pad(2); // two-byte padding for formula signature input
    *sig = siphash_digest();
    trusted_utils_write_sig(*sig, f_out);
    return input_finished && !input_invalid;
}
