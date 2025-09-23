
#include <stdbool.h>         // for bool
#include <stdio.h>           // for fopen, FILE
#include <stdlib.h>          // for abort
#include "trusted_parser.h"  // for tp_init, tp_parse
#include "trusted_utils.h"   // for trusted_utils_begins_with

int main(int argc, char *argv[]) {

    const char *formula_input = "", *fifo_parsed_formula = "";
    for (int i = 0; i < argc; i++) {
        trusted_utils_try_match_arg(argv[i], "-formula-input=", &formula_input);
        trusted_utils_try_match_arg(argv[i], "-fifo-parsed-formula=", &fifo_parsed_formula);
    }

    // Parse
    FILE* source = fopen(fifo_parsed_formula, "w");
    tp_init(formula_input, source);
    u8* sig;
    bool ok = tp_parse(&sig);
    if (!ok) abort();
    return 0;
}
