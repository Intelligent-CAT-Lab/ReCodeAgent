
#include <stdbool.h>        // for bool, false, true
#include <stdio.h>          // for snprintf
#include "lrat_check.h"     // for lrat_check_add_axiomatic_clause, lrat_che...
#include "secret.h"         // for SECRET_KEY
#include "siphash.h"        // for siphash_update, siphash_digest, siphash_r...
#include "trusted_utils.h"  // for u8, trusted_utils_copy_bytes, trusted_uti...
#include "confirm.h"

bool parsed_formula = false;
signature formula_signature;

bool valid = true;


void compute_clause_signature(u64 id, const int* lits, int nb_lits, u8* out) {
    siphash_reset();
    siphash_update((u8*) &id, sizeof(u64));
    siphash_update((u8*) lits, nb_lits*sizeof(int));
    siphash_update(formula_signature, SIG_SIZE_BYTES);
    const u8* hash_out = siphash_digest();
    trusted_utils_copy_bytes(out, hash_out, SIG_SIZE_BYTES);
}


void top_check_init(int nb_vars, bool check_model, bool lenient) {
    siphash_init(SECRET_KEY);
    lrat_check_init(nb_vars, check_model, lenient);
}

void top_check_commit_formula_sig(const u8* f_sig) {
    // Store formula signature to validate later after loading
    trusted_utils_copy_bytes(formula_signature, f_sig, SIG_SIZE_BYTES);
}

void top_check_load(int lit) {
    valid &= lrat_check_load(lit);
}

bool top_check_end_load() {
    u8* sig_from_chk;
    valid = valid && lrat_check_end_load(&sig_from_chk);
    if (!valid) return false;
    // Check against provided signature
    valid = trusted_utils_equal_signatures(sig_from_chk, formula_signature);
    if (!valid) snprintf(trusted_utils_msgstr, 512, "Formula signature check failed");
    return valid;
}

bool top_check_produce(unsigned long id, const int* literals, int nb_literals,
    const unsigned long* hints, int nb_hints, u8* out_sig_or_null) {
    
    // forward clause to checker
    valid &= lrat_check_add_clause(id, literals, nb_literals, hints, nb_hints);
    if (!valid) return false;
    // compute signature if desired
    if (out_sig_or_null) {
        compute_clause_signature(id, literals, nb_literals, out_sig_or_null);
    }
    return true;
}

bool top_check_import(unsigned long id, const int* literals, int nb_literals,
    const u8* signature_data) {
    
    // verify signature
    signature computed_sig;
    compute_clause_signature(id, literals, nb_literals, computed_sig);
    if (!trusted_utils_equal_signatures(signature_data, computed_sig)) {
        valid = false;
        snprintf(trusted_utils_msgstr, 512, "Signature check of clause %lu failed", id);
        return false;
    }

    // signature verified - forward clause to checker as an axiom
    valid &= lrat_check_add_axiomatic_clause(id, literals, nb_literals);
    return valid;
}

bool top_check_delete(const unsigned long* ids, int nb_ids) {
    return lrat_check_delete_clause(ids, nb_ids);
}

bool top_check_validate_unsat(u8* out_signature_or_null) {
    valid &= lrat_check_validate_unsat();
    if (!valid) return false;
    if (out_signature_or_null)
        confirm_result(formula_signature, 20, out_signature_or_null);
    return true;
}

bool top_check_validate_sat(int* model, u64 size, u8* out_signature_or_null) {
    valid &= lrat_check_validate_sat(model, size);
    if (!valid) return false;
    if (out_signature_or_null)
        confirm_result(formula_signature, 10, out_signature_or_null);
    return true;
}

bool top_check_valid() {return valid;}
