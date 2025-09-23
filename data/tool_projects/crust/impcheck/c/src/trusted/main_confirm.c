
#include <stdbool.h>         // for bool
#include <stdio.h>           // for fopen, FILE
#include <stdlib.h>          // for abort
#include <string.h>

#include "trusted_parser.h"  // for tp_init, tp_parse
#include "trusted_utils.h"   // for trusted_utils_begins_with
#include "confirm.h"

int error() {
    printf("s NOT VERIFIED\n");
    return 1;
}

int main(int argc, char *argv[]) {

    const char *formula_input = "", *result_sig = "", *resultint_str = "";
    for (int i = 0; i < argc; i++) {
        trusted_utils_try_match_arg(argv[i], "-formula-input=", &formula_input);
        trusted_utils_try_match_arg(argv[i], "-result-sig=", &result_sig);
        trusted_utils_try_match_arg(argv[i], "-result=", &resultint_str);
    }

    // valid input?
    int result = atoi(resultint_str);
    if (result != 10 && result != 20) {
        trusted_utils_log_err("Result code missing or invalid");
        return error();
    }
    if (strnlen(result_sig, 2*SIG_SIZE_BYTES+1) != 2*SIG_SIZE_BYTES) {
        trusted_utils_log_err("Result signature missing or malformed");
        return error();
    }

    // Parse formula to get its signature
    FILE* source = fopen("/dev/null", "w"); // write formula to /dev/null
    tp_init(formula_input, source);
    u8* sig_formula;
    bool ok = tp_parse((u8**) &sig_formula);
    if (!ok) {
        trusted_utils_log_err("Problem during parsing");
        return error();
    }

    // convert the reported signature from hex string to raw data
    signature sig_res_reported;
    ok = trusted_utils_str_to_sig(result_sig, sig_res_reported);
    if (!ok) {
        trusted_utils_log_err("Invalid signature string");
        return error();
    }

    // re-compute result signature
    signature sig_res_computed;
    confirm_result(sig_formula, (u8) result, sig_res_computed);

    // check reported signature against computed signature
    if (!trusted_utils_equal_signatures(sig_res_computed, sig_res_reported)) {
        trusted_utils_log_err("Signature does not match!");
        return error();
    }

    if (result == 10)
        printf("s VERIFIED SATISFIABLE\n");
    if (result == 20)
        printf("s VERIFIED UNSATISFIABLE\n");
    return 0;
}
