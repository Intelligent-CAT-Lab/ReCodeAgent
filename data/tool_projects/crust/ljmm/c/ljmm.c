#include <sys/mman.h>
#include <unistd.h>
#include <stdint.h>
#include <fcntl.h>  /* for open(2) */
#include <stdio.h>  /* for snprintf */
#include <string.h>

#ifdef DEBUG
    #include <stdlib.h>
    #define ASSERT(c) if (!(c))\
        { fprintf(stderr, "%s:%d Assert: %s\n", __FILE__, __LINE__, #c); abort(); }
#else
    #define ASSERT(c) ((void)0)
#endif

#define unlikely(x) __builtin_expect((x),0)
#define likely(x)   __builtin_expect((x),1)

#define REAL_MMAP __real_mmap64
#define WRAP_MMAP __wrap_mmap64

extern void * REAL_MMAP(void *addr, size_t length, int prot, int flags,
                        int fd, off_t offset);

typedef struct {
    uintptr_t   page_size;
    uintptr_t   page_mask;

    uintptr_t   addr_upbound;
    uintptr_t   addr_lowbound;

    char        *dummy_blk;

    const char  *map_file; /* "/proc/$PID/maps" */

    char        *buffer;   /* the buffer to read /proc/$PID/maps */
    int          buf_len;

    /* it is up to OS to take care the 1G..2G space */
    char         OS_take_care_1G_2G;
    char         init_succ;
} ljmm_t;

static ljmm_t ljmm;

enum {
    BUFFER_SZ           = 8192,
    DUMMY_BLK_SZ        = 12,
    ADDR_2G             = 0x80000000,
    ADDR_1G             = 0x40000000,
};

/* Initialize before the main() is entered */
__attribute__((constructor)) static void
ljmm_init(void) {
    ljmm.OS_take_care_1G_2G =
#if defined(STRESS_TEST)
    0
#else
    1
#endif
    ;

    ljmm.addr_lowbound = (uintptr_t)sbrk(0);
    ljmm.addr_upbound  = ADDR_2G;

    ljmm.page_size = sysconf(_SC_PAGESIZE);
    ljmm.page_mask = ljmm.page_size - 1;

    /* step 1: mmap() a tiny block to prevent heap from growing, whichby we
     *   reserve the space [sbrk(0) - 2G].
     */
    char* p = REAL_MMAP(sbrk(0), DUMMY_BLK_SZ, PROT_READ|PROT_WRITE,
                        MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    if (unlikely(p == MAP_FAILED)) {
        return;
    }
    ljmm.dummy_blk = p;

    ljmm.map_file = "/proc/self/maps";

    /* step 2: create buffer for reading content from /proc/$PID/maps */
    p = REAL_MMAP(NULL, BUFFER_SZ, PROT_READ|PROT_WRITE,
                  MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    if (unlikely(p == MAP_FAILED)) {
        return;
    }
    ljmm.buffer = p;

    ljmm.init_succ = 1;
}

/* This function will be called when the process exit normally.*/
__attribute__((destructor)) static void
ljmm_finalize(void) {
    if (ljmm.dummy_blk && ljmm.dummy_blk != MAP_FAILED) {
        munmap(ljmm.dummy_blk, DUMMY_BLK_SZ);
        ljmm.dummy_blk = NULL;
        ljmm.map_file = NULL;
    }

    if (ljmm.buffer && ljmm.buffer != MAP_FAILED) {
        munmap(ljmm.buffer, BUFFER_SZ);
    }

    ljmm.init_succ = 0;
}

/* Read the first "BUFFER_SZ - 1" bytes from the given files into ljmm.buffer. */
static int
read_maps_to_buffer(void) {
    int fd = open(ljmm.map_file, 0);
    if (fd == -1) {
        return -1;
    }

    int len = read(fd, ljmm.buffer, BUFFER_SZ - 1);
    if (len >= 0 && ljmm.buffer[len-1] != '\n') {
        ljmm.buffer[len++] = '\n';
    }

    close(fd);

    ljmm.buf_len = len;
    return len;
}


static inline uintptr_t
page_align_addr(uintptr_t addr) {
    return (addr + ljmm.page_size - 1) & ~ljmm.page_mask;
}


/* Parse the addr-string, stop at a char which is neither '0'-'9' nor
 * 'a'-'f', nor ['A'-'F'].  The resulting address is saved to <addr_val>,
 * and the number of char the address spans is returned to the called.
 */
static int
parse_addr(const char* addr_str, uintptr_t *addr_val) {
    uintptr_t addr = 0;
    const char* p = addr_str;
    while (1) {
        char c = *p++;
        if (c >= '0' && c <= '9') {
            addr = addr * 16 + c - '0';
            continue;
        }

        c |= 0x20;
        if (c >= 'a' && c <= 'f') {
            addr = addr * 16 + 10 + c - 'a';
            continue;
        }

        *addr_val = addr;
        return p - 1 - addr_str;
    }

    return 0;
}


typedef struct {
    uintptr_t start;
    size_t size;
} mem_blk_t;


/* Going through the /proc/$PID/map in an attempt to find an unallocated hole
 * that tightly/best fit the allocation request. Return the best-fit address
 * if it prevailed, or NULL otherwise.
 */
static uintptr_t
find_best_fit(size_t length) {
    /* read the /proc/$PID/maps into the in-memory buffer */
    if (read_maps_to_buffer() < 0) {
        return 0;
    }

    length = page_align_addr(length);

    mem_blk_t best_fit, prev_blk;
    const char* buffer;
    int ofst, buf_len;
    uintptr_t upbound, lowbound;

    lowbound = ljmm.addr_lowbound;
    upbound = ljmm.addr_upbound;
    best_fit.size = -1;
    best_fit.start = upbound;
    prev_blk.size = prev_blk.start = 0;
    ofst = 0;
    buffer = ljmm.buffer;
    buf_len = ljmm.buf_len;

    /* iterate the /proc/$PID/maps, the fields of each line are:
     *   - address range:
     *   - permission
     *   - offset    : The offset of the file being mmapped. If it's not file
     *                 mapping, this column must take value 0.
     *   - dev       :
     *   - inode     : inode of file being mmapped or empty otherwise
     *   - pathname  : path of the file being mmaped or empty otherwise
     *
     * e.g
     * ...
     * 00618000-00619000 rw-p 00018000 08:02 13505427    /usr/lib/thunderbird/thunderbird
     * 7f2b4b200000-7f2b4b300000 rw-p 00000000 00:00 0
     * 7f2b4b400000-7f2b4ca00000 rw-p 00000000 00:00 0
     * ...
     *
     *  We only care about the address-ranges.
     */
    ASSERT(buffer[buf_len-1] == '\n');
    while (ofst < buf_len) {
        uintptr_t start_addr, end_addr;

        /* step 1: get the lower-bound of the address range */
        int advance = parse_addr(ljmm.buffer + ofst, &start_addr);
        if (advance && (buffer[ofst + advance] == '-')) {
            ofst += advance + 1;
        } else {
            /* the buffer is not large enough to accommodate /proc/$PID/map */
            break;
        }

        /* step 2: get the upper bound of the address range */
        advance = parse_addr(buffer + ofst , &end_addr);
        if (advance && buffer[ofst + advance] == ' ') {
            ofst += advance + 1;
            ASSERT(ofst < buf_len);
            /* skip the rest of the line. */
            while (buffer[ofst++] != '\n') {}
        } else {
            /* we can move on with incomplete up-bound address */
            end_addr = upbound >= start_addr ? upbound : start_addr;
        }

        end_addr = page_align_addr(end_addr);

        /* step 3: get the unallocated hole between the previous and current
         *  mmapped blocks, and see if the hole is the best-fit.
         */
        uintptr_t hole_start = prev_blk.start + prev_blk.size;
        uintptr_t hole_size = start_addr - hole_start;

        if (hole_size >= length &&
            (hole_start >= lowbound && (hole_start + length) <= upbound) &&
            hole_size < best_fit.size) {
            best_fit.start = hole_start;
            best_fit.size = hole_size;

            if (best_fit.size == length) {
                break;
            }
        }

        /* step 4: determine if we need to examine the next line */
        if (unlikely(start_addr >= ADDR_1G)) {
            /* We let OS to take care the [1G, 2G] space */
            if (!ljmm.OS_take_care_1G_2G || start_addr >= upbound) {
                break;
            }
        }

        if (unlikely(end_addr >= upbound)) {
            /* HINT: the lines are in the ascending order of the address-range.
             */
            break;
        }

        prev_blk.start = start_addr;
        prev_blk.size = end_addr - start_addr;
    }

    return (best_fit.size != (uintptr_t)-1) ? best_fit.start : 0;
}


/* All the calls to mmap64() will be replaced by the calls to this function,
 * which will be done by the linker fed with "--wrap=mmap64" flag.
 */
void *
WRAP_MMAP(void *addr, size_t length, int prot, int flags, int fd,
    off_t offset) {

    if (!(flags & MAP_32BIT) || addr || !ljmm.init_succ) {
        return REAL_MMAP(addr, length, prot, flags, fd, offset);
    }

    if (ljmm.OS_take_care_1G_2G) {
        void *blk = REAL_MMAP(NULL, length, prot, flags, fd, offset);
        if (blk != MAP_FAILED) {
            return blk;
        }
    }

    uintptr_t best_fit = find_best_fit(length);
    if (best_fit) {
        return REAL_MMAP((void*)best_fit, length, prot,
                         flags & ~MAP_32BIT, fd, offset);
    }

    /* Do not directly return MAP_FAILED. We should call mmap64() in order to
     * set errno properly.
     */
    return REAL_MMAP(NULL, length, prot, flags, fd, offset);
}


void
ljmm_let_OS_take_care_1G_2G(int turn_on) {
    ljmm.OS_take_care_1G_2G = turn_on;
}


void
ljmm_test_set_test_param(const char* map_file, void *sbrk0, int page_size) {
    ljmm.map_file = map_file;
    ljmm.addr_lowbound = (uintptr_t)sbrk0;

    /* page-size must be a power-of-two */
    ASSERT(page_size && (((page_size - 1) & page_size) == 0));
    ljmm.page_size = page_size;
    ljmm.page_mask = page_size - 1;
}
