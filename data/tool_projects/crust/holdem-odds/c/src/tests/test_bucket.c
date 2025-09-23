#include "CuTest.h"
#include "bucket.h"
#include "cards.h"


void TestBucket(CuTest* tc) {
    Bucket b = { .count = 0 };
    Card c1 = NewCard(Ace, Club);
    Card c2 = NewCard(Ace, Diamond);
    Card c3 = NewCard(Ace, Heart);
    Card c4 = NewCard(Ace, Spade);

    CuAssert(tc, "Bucket should have length 0", b.count == 0);

    BucketAdd(&b, &c1);
    CuAssert(tc, "Bucket should have length 1", b.count == 1);
    CuAssert(tc, "bucket[0] = Ac", b.cards[0].suit = Club);

    BucketAdd(&b, &c2);
    CuAssert(tc, "Bucket should have length 2", b.count == 2);
    CuAssert(tc, "bucket[0] = Ac", b.cards[0].suit = Club);
    CuAssert(tc, "bucket[1] = Ad", b.cards[1].suit = Diamond);

    BucketAdd(&b, &c3);
    CuAssert(tc, "Bucket should have length 3", b.count == 3);
    CuAssert(tc, "bucket[0] = Ac", b.cards[0].suit = Club);
    CuAssert(tc, "bucket[1] = Ad", b.cards[1].suit = Diamond);
    CuAssert(tc, "bucket[2] = Ah", b.cards[2].suit = Heart);

    BucketAdd(&b, &c4);
    CuAssert(tc, "Bucket should have length 4", b.count == 4);
    CuAssert(tc, "bucket[0] = Ac", b.cards[0].suit = Club);
    CuAssert(tc, "bucket[1] = Ad", b.cards[1].suit = Diamond);
    CuAssert(tc, "bucket[2] = Ah", b.cards[2].suit = Heart);
    CuAssert(tc, "bucket[3] = As", b.cards[3].suit = Spade);

    BucketAdd(&b, &c4);
    CuAssert(tc, "Bucket should still have length 4", b.count == 4);
    CuAssert(tc, "bucket[0] = Ac", b.cards[0].suit = Club);
    CuAssert(tc, "bucket[1] = Ad", b.cards[1].suit = Diamond);
    CuAssert(tc, "bucket[2] = Ah", b.cards[2].suit = Heart);
    CuAssert(tc, "bucket[3] = As", b.cards[3].suit = Spade);
}


CuSuite* BucketSuite() {
    CuSuite* suite = CuSuiteNew();

    SUITE_ADD_TEST(suite, TestBucket);

    return suite;
}
