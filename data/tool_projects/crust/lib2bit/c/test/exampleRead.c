#include "2bit.h"
#include <assert.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    TwoBit *tb;
    tb = twobitOpen(argv[1], 1);

    assert(tb);
    uint32_t i;
    for(i=0; i<tb->hdr->nChroms; i++) {
        printf("%"PRIu32"\t%s\t%"PRIu32" offset 0x%"PRIx64"\n", i, tb->cl->chrom[i], tb->idx->size[i], tb->idx->offset[i]);
    }

    char *seq;
    seq = twobitSequence(tb, "chr1", 0, 0);
    printf("%s\n", seq);
    if(seq) free(seq);

    seq = twobitSequence(tb, "chr1", 24, 74);
    printf("%s\n", seq);
    if(seq) free(seq);

    double *stats;
    stats = twobitBases(tb, "chr1", 0, 0, 1);
    assert(stats);
    for(i=0; i<4; i++) {
        printf("%"PRIu32"\t%f\n", i, stats[i]);
    }
    free(stats);

    stats = twobitBases(tb, "chr1", 24, 74, 1);
    assert(stats);
    for(i=0; i<4; i++) {
        printf("%"PRIu32"\t%f\n", i, stats[i]);
    }
    free(stats);

    twobitClose(tb);

    return 0;
}
