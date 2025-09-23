
#include "trusted_utils.h"
#include "siphash.h"

void confirm_result(u8* f_sig, u8 constant, u8* out) {
    siphash_reset();
    siphash_update(f_sig, SIG_SIZE_BYTES);
    siphash_update(&constant, 1);
    u8* sig = siphash_digest();
    trusted_utils_copy_bytes(out, sig, SIG_SIZE_BYTES);
}
