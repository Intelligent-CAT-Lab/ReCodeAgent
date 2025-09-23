#ifndef _BUCKET_H

#define _BUCKET_H

#include <stddef.h>

#include "cards.h"


typedef struct {
    Card    cards[4];
    size_t  count;
} Bucket;


void   BucketAdd(Bucket*, const Card*);


#endif
