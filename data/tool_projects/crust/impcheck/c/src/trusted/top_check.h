
#include <stdbool.h>        // for bool
#include "trusted_utils.h"  // for u8, u64

// Top level checking procedure. Checks clauses, validates signatures,
// and returns certificates for (un)satisfiability.

void top_check_init(int nb_vars, bool check_model, bool lenient);
void top_check_commit_formula_sig(const u8* f_sig);
void top_check_load(int lit);
bool top_check_end_load();
bool top_check_produce(unsigned long id, const int* literals, int nb_literals,
    const unsigned long* hints, int nb_hints, u8* out_sig_or_null);
bool top_check_import(unsigned long id, const int* literals, int nb_literals,
    const u8* signature_data);
bool top_check_delete(const unsigned long* ids, int nb_ids);
bool top_check_validate_unsat(u8* out_signature_or_null);
bool top_check_validate_sat(int* model, u64 size, u8* out_signature_or_null);
bool top_check_valid();
