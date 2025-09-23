/* This testing case is to dispel bugs about parsing /proc/$PID/maps */

#include <sys/mman.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include "../ljmm.h"

enum {
    ADDR_1G = 0x40000000,
    ADDR_2G = 0x80000000,
};

typedef struct {
    const char* map_file;
    size_t alloc_sz;

    /* should not allocate block under this addr; mimic sbrk(0) */
    uintptr_t low_bound;

    /* the return code of mmmap should fall into this range */
    uintptr_t ret_code_low;
    uintptr_t ret_code_high;

    int OS_take_care_1G_2G;
} test_data_t;

test_data_t test_data[] = {
    /* test 1 */
    {"input_001_001.txt", 32 * 1024 - 1,    0,        0x418000, 0x418000, 0},

    /* test 2: considering low-bound */
    {"input_001_001.txt", 32 * 1024 - 100,  0x619000, 0x619000, 0x619000, 0},

    /* test 3: Let OS take over [1G-2G] space */
    {"input_001_001.txt", 32 * 1024 - 10,   0,        ADDR_1G,  ADDR_2G, 1},

    /* test 4: mimic the sitatuation when buffer is not large enough to
     * accommodate the /proc/pid/maps.
     */
    {"input_001_002.txt", 32 * 1024 - 10,   0x619000,  0x619000, 0x619000, 0},

    /* test 5: mimic the sitatuation when buffer is not large enough to
     *   accommodate the /proc/pid/maps, the starting addr is last line is
     *   incomplete.
     */
    {"input_001_003.txt", 32 * 1024 - 10,   0x619000,  ADDR_1G, ADDR_2G, 0},

    {"input_001_004.txt", 32 * 1024,   0x619000,  0x3ffff000, 0x3ffff000, 0},
};


static inline int
in_range(uintptr_t val, uintptr_t low, uintptr_t high) {
    return val >= low && val <= high;
}

int
main (int argc, char** argv) {
    int idx, num;
    for (idx = 0, num = sizeof(test_data)/sizeof(test_data[0]);
         idx < num; idx++) {

        test_data_t *p = &test_data[idx];

        char input_path[4096];
        snprintf(input_path, sizeof(input_path), "test_input/%s", p->map_file);

        ljmm_let_OS_take_care_1G_2G(p->OS_take_care_1G_2G);
        ljmm_test_set_test_param(input_path, (void*)p->low_bound, 4096);

        void *r = mmap(NULL, p->alloc_sz, PROT_READ|PROT_WRITE,
                       MAP_32BIT|MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);

        if (!in_range((uintptr_t)r, p->ret_code_low, p->ret_code_high)) {
            fprintf(stderr, "fail to run testing %d: with input %s\n",
                    idx + 1, p->map_file);
            exit(1);
        }

        if (r != MAP_FAILED) {
            munmap(r, p->alloc_sz);
        }
    }

    return 0;
}
