#include <stddef.h>
#include <stdlib.h>
#include <assert.h>
#include <limits.h>
#include <string.h>
#include <stdio.h>
#include <math.h>

#ifndef ZOPFLI_SLICE_H_
#define ZOPFLI_SLICE_H_

/*
The "f" for the FindMinimum function below.
i: the current parameter of f(i)
context: for your implementation

Instance: `SplitCost` using `SplitCostContext` as context
*/
typedef double FindMinimumFun(size_t i, void *context);

/*
Stores lit/length and dist pairs for LZ77.
Parameter litlens: Contains the literal symbols or length values.
Parameter dists: Contains the distances. A value is 0 to indicate that there is
no dist and the corresponding litlens value is a literal instead of a length.
Parameter size: The size of both the litlens and dists arrays.
The memory can best be managed by using ZopfliInitLZ77Store to initialize it,
ZopfliCleanLZ77Store to destroy it, and ZopfliStoreLitLenDist to append values.

*/
typedef struct ZopfliLZ77Store
{
    unsigned short *litlens; /* Lit or len. */
    unsigned short *dists;   /* If 0: indicates literal in corresponding litlens,
         if > 0: length in corresponding litlens, this is the distance. */
    size_t size;

    const unsigned char *data; /* original data */
    size_t *pos;               /* position in data where this LZ77 command begins */

    unsigned short *ll_symbol;
    unsigned short *d_symbol;

    /* Cumulative histograms wrapping around per chunk. Each chunk has the amount
    of distinct symbols as length, so using 1 value per LZ77 symbol, we have a
    precise histogram at every N symbols, and the rest can be calculated by
    looping through the actual symbols of this chunk. */
    size_t *ll_counts;
    size_t *d_counts;
} ZopfliLZ77Store;

typedef struct SplitCostContext
{
    const ZopfliLZ77Store *lz77;
    size_t start;
    size_t end;
} SplitCostContext;

/* Number of distinct literal/length and distance symbols in DEFLATE */
#define ZOPFLI_NUM_LL 288

#define ZOPFLI_NUM_D 32

typedef struct SymbolStats
{
    /* The literal and length symbols. */
    size_t litlens[ZOPFLI_NUM_LL];
    /* The 32 unique dist symbols, not the 32768 possible dists. */
    size_t dists[ZOPFLI_NUM_D];

    /* Length of each lit/len symbol in bits. */
    double ll_symbols[ZOPFLI_NUM_LL];
    /* Length of each dist symbol in bits. */
    double d_symbols[ZOPFLI_NUM_D];
} SymbolStats;

typedef struct RanState
{
    unsigned int m_w, m_z;
} RanState;

/*
Function that calculates a cost based on a model for the given LZ77 symbol.
litlen: means literal symbol if dist is 0, length otherwise.

Instance: `GetCostFixed` using NULL as context
Instance: `GetCostStat` using `SymbolStats` as context
*/
typedef double CostModelFun(unsigned litlen, unsigned dist, void *context);

typedef struct ZopfliHash
{
    int *head;            /* Hash value to index of its most recent occurrence. */
    unsigned short *prev; /* Index to index of prev. occurrence of same hash. */
    int *hashval;         /* Index to hash value at this index. */
    int val;              /* Current hash value. */

    /* Fields with similar purpose as the above hash, but for the second hash with
    a value that is calculated differently.  */
    int *head2;            /* Hash value to index of its most recent occurrence. */
    unsigned short *prev2; /* Index to index of prev. occurrence of same hash. */
    int *hashval2;         /* Index to hash value at this index. */
    int val2;              /* Current hash value. */

    unsigned short *same; /* Amount of repetitions of same byte after this .*/
} ZopfliHash;

typedef struct Node Node;

/*
Nodes forming chains. Also used to represent leaves.
*/
struct Node
{
    size_t weight; /* Total weight (symbol count) of this chain. */
    Node *tail;    /* Previous node(s) of this chain, or 0 if none. */
    int count;     /* Leaf symbol index, or number of leaves before this chain. */
};

/*
Memory pool for nodes.
*/
typedef struct NodePool
{
    Node *next; /* Pointer to a free node in the pool. */
} NodePool;

/*
Cache used by ZopfliFindLongestMatch to remember previously found length/dist
values.
This is needed because the squeeze runs will ask these values multiple times for
the same position.
Uses large amounts of memory, since it has to remember the distance belonging
to every possible shorter-than-the-best length (the so called "sublen" array).
*/
typedef struct ZopfliLongestMatchCache
{
    unsigned short *length;
    unsigned short *dist;
    unsigned char *sublen;
} ZopfliLongestMatchCache;

/* for size_t */
/*
Options used throughout the program.
*/
typedef struct ZopfliOptions
{
    /* Whether to print output */
    int verbose;

    /* Whether to print more detailed output */
    int verbose_more;

    /*
    Maximum amount of times to rerun forward and backward pass to optimize LZ77
    compression cost. Good values: 10, 15 for small files, 5 for files over
    several MB in size or it will be too slow.
    */
    int numiterations;

    /*
    If true, splits the data in multiple deflate blocks with optimal choice
    for the block boundaries. Block splitting gives better compression. Default:
    true (1).
    */
    int blocksplitting;

    /*
    No longer used, left for compatibility.
    */
    int blocksplittinglast;

    /*
    Maximum amount of blocks to split into (0 for unlimited, but this can give
    extreme results that hurt compression on some files). Default value: 15.
    */
    int blocksplittingmax;
} ZopfliOptions;

/*
Some state information for compressing a block.
This is currently a bit under-used (with mainly only the longest match cache),
but is kept for easy future expansion.
*/
typedef struct ZopfliBlockState
{
    const ZopfliOptions *options;

    /* Cache for length/distance pairs found so far. */
    ZopfliLongestMatchCache *lmc;

    /* The start (inclusive) and end (not inclusive) of the current block. */
    size_t blockstart;
    size_t blockend;
} ZopfliBlockState;

/*
Appends value to dynamically allocated memory, doubling its allocation size
whenever needed.

value: the value to append, type T
data: pointer to the dynamic array to append to, type T**
size: pointer to the size of the array to append to, type size_t*. This is the
size that you consider the array to be, not the internal allocation size.
Precondition: allocated size of data is at least a power of two greater than or
equal than *size.
*/
#define ZOPFLI_APPEND_DATA(/* T */ value, /* T** */ data, /* size_t* */ size)        \
    {                                                                                \
        if (!((*size) & ((*size) - 1)))                                              \
        {                                                                            \
            /*double alloc size if it's a power of two*/                             \
            (*data) = (*size) == 0 ? malloc(sizeof(**data))                          \
                                   : realloc((*data), (*size) * 2 * sizeof(**data)); \
            memset((*data) + (*size), 0, (*size) * sizeof(**data));                  \
        }                                                                            \
        (*data)[(*size)] = (value);                                                  \
        (*size)++;                                                                   \
    }

/*
Outputs minimum-redundancy length-limited code bitlengths for symbols with the
given counts. The bitlengths are limited by maxbits.

The output is tailored for DEFLATE: symbols that never occur, get a bit length
of 0, and if only a single symbol occurs at least once, its bitlength will be 1,
and not 0 as would theoretically be needed for a single symbol.

frequencies: The amount of occurrences of each symbol.
n: The amount of symbols.
maxbits: Maximum bit length, inclusive.
bitlengths: Output, the bitlengths for the symbol prefix codes.
return: 0 for OK, non-0 for error.
*/
int ZopfliLengthLimitedCodeLengths(
    const size_t *frequencies, int n, int maxbits, unsigned *bitlengths);

/*
Calculates the bitlengths for the Huffman tree, based on the counts of each
symbol.
*/
void ZopfliCalculateBitLengths(const size_t *count, size_t n, int maxbits,
                               unsigned *bitlengths);

/*
Converts a series of Huffman tree bitlengths, to the bit values of the symbols.
*/
void ZopfliLengthsToSymbols(const unsigned *lengths, size_t n, unsigned maxbits,
                            unsigned *symbols);

/* Gets the histogram of lit/len and dist symbols in the given range, using the
cumulative histograms, so faster than adding one by one for large range. Does
not add the one end symbol of value 256. */
void ZopfliLZ77GetHistogram(const ZopfliLZ77Store *lz77,
                            size_t lstart, size_t lend,
                            size_t *ll_counts, size_t *d_counts);

/* Gets the amount of raw bytes that this range of LZ77 symbols spans. */
size_t ZopfliLZ77GetByteRange(const ZopfliLZ77Store *lz77,
                              size_t lstart, size_t lend);

/*
Calculates block size in bits.
litlens: lz77 lit/lengths
dists: ll77 distances
lstart: start of block
lend: end of block (not inclusive)
*/
double ZopfliCalculateBlockSize(const ZopfliLZ77Store *lz77,
                                size_t lstart, size_t lend, int btype);

/*
For longest match cache. max 256. Uses huge amounts of memory but makes it
faster. Uses this many times three bytes per single byte of the input data.
This is so because longest match finding has to find the exact distance
that belongs to each length for the best lz77 strategy.
Good values: e.g. 5, 8.
*/
#define ZOPFLI_CACHE_LENGTH 8

/* Returns the length up to which could be stored in the cache. */
unsigned ZopfliMaxCachedSublen(const ZopfliLongestMatchCache *lmc,
                               size_t pos, size_t length);

/* Stores sublen array in the cache. */
void ZopfliSublenToCache(const unsigned short *sublen,
                         size_t pos, size_t length,
                         ZopfliLongestMatchCache *lmc);

#define ZOPFLI_MIN_MATCH 3

/* Minimum and maximum length that can be encoded in deflate. */
#define ZOPFLI_MAX_MATCH 258

/* Extracts sublen array from the cache. */
void ZopfliCacheToSublen(const ZopfliLongestMatchCache *lmc,
                         size_t pos, size_t length,
                         unsigned short *sublen);

/*
Finds the longest match (length and corresponding distance) for LZ77
compression.
Even when not using "sublen", it can be more efficient to provide an array,
because only then the caching is used.
array: the data
pos: position in the data to find the match for
size: size of the data
limit: limit length to maximum this value (default should be 258). This allows
finding a shorter dist for that length (= less extra bits). Must be
in the range [ZOPFLI_MIN_MATCH, ZOPFLI_MAX_MATCH].
sublen: output array of 259 elements, or null. Has, for each length, the
smallest distance required to reach this length. Only 256 of its 259 values
are used, the first 3 are ignored (the shortest length is 3. It is purely
for convenience that the array is made 3 longer).
*/
void ZopfliFindLongestMatch(
    ZopfliBlockState *s, const ZopfliHash *h, const unsigned char *array,
    size_t pos, size_t size, size_t limit,
    unsigned short *sublen, unsigned short *distance, unsigned short *length);

/*
limit the max hash chain hits for this hash value. This has an effect only
on files where the hash value is the same very often. On these files, this
gives worse compression (the value should ideally be 32768, which is the
ZOPFLI_WINDOW_SIZE, while zlib uses 4096 even for best level), but makes it
faster on some specific files.
Good value: e.g. 8192.
*/
#define ZOPFLI_MAX_CHAIN_HITS 8192

/*
The window size for deflate. Must be a power of two. This should be 32768, the
maximum possible by the deflate spec. Anything less hurts compression more than
speed.
*/
#define ZOPFLI_WINDOW_SIZE 32768

/*
The window mask used to wrap indices into the window. This is why the
window size must be a power of two.
*/
#define ZOPFLI_WINDOW_MASK (ZOPFLI_WINDOW_SIZE - 1)

/*
Verifies if length and dist are indeed valid, only used for assertion.
*/
void ZopfliVerifyLenDist(const unsigned char *data, size_t datasize, size_t pos,
                         unsigned short dist, unsigned short length);

/*
Prepopulates hash:
Fills in the initial values in the hash, before ZopfliUpdateHash can be used
correctly.
*/
void ZopfliWarmupHash(const unsigned char *array, size_t pos, size_t end,
                      ZopfliHash *h);

void ZopfliStoreLitLenDist(unsigned short length, unsigned short dist,
                           size_t pos, ZopfliLZ77Store *store);

/*
Updates the hash values based on the current position in the array. All calls
to this must be made for consecutive bytes.
*/
void ZopfliUpdateHash(const unsigned char *array, size_t pos, size_t end,
                      ZopfliHash *h);

/* Resets all fields of ZopfliHash. */
void ZopfliResetHash(size_t window_size, ZopfliHash *h);

/*
Used to initialize costs for example
*/
#define ZOPFLI_LARGE_FLOAT 1e30

/* Frees ZopfliHash memory. */
void ZopfliCleanHash(ZopfliHash *h);

/*
Does the same as ZopfliLZ77Optimal, but optimized for the fixed tree of the
deflate standard.
The fixed tree never gives the best compression. But this gives the best
possible LZ77 encoding possible with the fixed tree.
This does not create or output any fixed tree, only LZ77 data optimized for
using with a fixed tree.
If instart is larger than 0, it uses values before instart as starting
dictionary.
*/
void ZopfliLZ77OptimalFixed(ZopfliBlockState *s,
                            const unsigned char *in,
                            size_t instart, size_t inend,
                            ZopfliLZ77Store *store);

/* Allocates ZopfliHash memory. */
void ZopfliAllocHash(size_t window_size, ZopfliHash *h);

void ZopfliCleanBlockState(ZopfliBlockState *s);

/* Frees up the memory of the ZopfliLongestMatchCache. */
void ZopfliCleanCache(ZopfliLongestMatchCache *lmc);

void ZopfliInitBlockState(const ZopfliOptions *options,
                          size_t blockstart, size_t blockend, int add_lmc,
                          ZopfliBlockState *s);

/* Initializes the ZopfliLongestMatchCache. */
void ZopfliInitCache(size_t blocksize, ZopfliLongestMatchCache *lmc);

void ZopfliCleanLZ77Store(ZopfliLZ77Store *store);

void ZopfliInitLZ77Store(const unsigned char *data, ZopfliLZ77Store *store);

/*
Calculates block size in bits, automatically using the best btype.
*/
double ZopfliCalculateBlockSizeAutoType(const ZopfliLZ77Store *lz77,
                                        size_t lstart, size_t lend);

/*
Does LZ77 using an algorithm similar to gzip, with lazy matching, rather than
with the slow but better "squeeze" implementation.
The result is placed in the ZopfliLZ77Store.
If instart is larger than 0, it uses values before instart as starting
dictionary.
*/
void ZopfliLZ77Greedy(ZopfliBlockState *s, const unsigned char *in,
                      size_t instart, size_t inend,
                      ZopfliLZ77Store *store, ZopfliHash *h);

/*
Calculates the entropy of each symbol, based on the counts of each symbol. The
result is similar to the result of ZopfliCalculateBitLengths, but with the
actual theoritical bit lengths according to the entropy. Since the resulting
values are fractional, they cannot be used to encode the tree specified by
DEFLATE.
*/
void ZopfliCalculateEntropy(const size_t *count, size_t n, double *bitlengths);

void ZopfliCopyLZ77Store(const ZopfliLZ77Store *source, ZopfliLZ77Store *dest);

/*
Calculates lit/len and dist pairs for given data.
If instart is larger than 0, it uses values before instart as starting
dictionary.
*/
void ZopfliLZ77Optimal(ZopfliBlockState *s,
                       const unsigned char *in, size_t instart, size_t inend,
                       int numiterations,
                       ZopfliLZ77Store *store);

/*
Does blocksplitting on LZ77 data.
The output splitpoints are indices in the LZ77 data.
maxblocks: set a limit to the amount of blocks. Set to 0 to mean no limit.
*/
void ZopfliBlockSplitLZ77(const ZopfliOptions *options,
                          const ZopfliLZ77Store *lz77, size_t maxblocks,
                          size_t **splitpoints, size_t *npoints);

/*
Does blocksplitting on uncompressed data.
The output splitpoints are indices in the uncompressed bytes.

options: general program options.
in: uncompressed input data
instart: where to start splitting
inend: where to end splitting (not inclusive)
maxblocks: maximum amount of blocks to split into, or 0 for no limit
splitpoints: dynamic array to put the resulting split point coordinates into.
The coordinates are indices in the input array.
npoints: pointer to amount of splitpoints, for the dynamic array. The amount of
blocks is the amount of splitpoitns + 1.
*/
void ZopfliBlockSplit(const ZopfliOptions *options,
                      const unsigned char *in, size_t instart, size_t inend,
                      size_t maxblocks, size_t **splitpoints, size_t *npoints);

void ZopfliAppendLZ77Store(const ZopfliLZ77Store *store,
                           ZopfliLZ77Store *target);

/*
Like ZopfliDeflate, but allows to specify start and end byte with instart and
inend. Only that part is compressed, but earlier bytes are still used for the
back window.
*/
void ZopfliDeflatePart(const ZopfliOptions *options, int btype, int final,
                       const unsigned char *in, size_t instart, size_t inend,
                       unsigned char *bp, unsigned char **out,
                       size_t *outsize);

/*
A block structure of huge, non-smart, blocks to divide the input into, to allow
operating on huge files without exceeding memory, such as the 1GB wiki9 corpus.
The whole compression algorithm, including the smarter block splitting, will
be executed independently on each huge block.
Dividing into huge blocks hurts compression, but not much relative to the size.
Set it to 0 to disable master blocks.
*/
#define ZOPFLI_MASTER_BLOCK_SIZE 1000000

/*
Compresses according to the deflate specification and append the compressed
result to the output.
This function will usually output multiple deflate blocks. If final is 1, then
the final bit will be set on the last block.

options: global program options
btype: the deflate block type. Use 2 for best compression.
-0: non compressed blocks (00)
-1: blocks with fixed tree (01)
-2: blocks with dynamic tree (10)
final: whether this is the last section of the input, sets the final bit to the
last deflate block.
in: the input bytes
insize: number of input bytes
bp: bit pointer for the output array. This must initially be 0, and for
consecutive calls must be reused (it can have values from 0-7). This is
because deflate appends blocks as bit-based data, rather than on byte
boundaries.
out: pointer to the dynamic output array to which the result is appended. Must
be freed after use.
outsize: pointer to the dynamic output array size.
*/
void ZopfliDeflate(const ZopfliOptions *options, int btype, int final,
                   const unsigned char *in, size_t insize,
                   unsigned char *bp, unsigned char **out, size_t *outsize);

#endif /* ZOPFLI_SLICE_H_ */