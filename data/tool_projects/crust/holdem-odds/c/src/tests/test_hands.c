#include <stdio.h>
#include <string.h>

#include "CuTest.h"
#include "cards.h"
#include "hands.h"

#define OUTPUT_LENGTH 512 // Error messages should not exceed this length.

static char ranks[] = " 23456789TJQKA";
//static char suits[] = " chds";
//static size_t ranksLength = 13;
//static size_t suitsLength = 4;


void NewHandFromString(const char* string, Card* hand) {
    size_t length = strlen(string);
    size_t j = 0;

    for (size_t i = 0; i < length; i += 3) {
        Card c = NewCardFromChars(string[i], string[i+1]);
        hand[j] = c;
        j++;
    }
}


void TestHandClassifyStraightFlush(CuTest* tc) {
    Card hand[5];
    char output[OUTPUT_LENGTH];
    const char *straight_flushes[] = {
        "As Ks Qs Js Ts",
        "Ks Qs Js Ts 9s",
        "Qs Js Ts 9s 8s",
        "Js Ts 9s 8s 7s",
        "Ts 9s 8s 7s 6s",
        "9s 8s 7s 6s 5s",
        "8s 7s 6s 5s 4s",
        "7s 6s 5s 4s 3s",
        "6s 5s 4s 3s 2s",
        NULL
    };
    for (size_t i = 0; straight_flushes[i] != NULL; ++i) {
        snprintf(output, OUTPUT_LENGTH, "[%s] should be a straight flush", straight_flushes[i]);
        NewHandFromString(straight_flushes[i], hand);

        for (Suit s = Club; s <= Spade; ++s) {
            for (size_t j = 0; j < HAND_LENGTH; ++j) { hand[j].suit = s; }
            CuAssert(tc, output, HandClassify(hand) == StraightFlush);
        }
    }
}


void TestHandClassifyWheelFlush(CuTest* tc) {
    Card hand[5];
    NewHandFromString("Ac 5c 4c 3c 2c", hand);
    CuAssert(tc, "A-5-4-3-2 should be a wheel flush", HandClassify(hand) == WheelFlush);
}

void TestHandClassifyQuads(CuTest* tc) {
    Card hand[5];
    char output[OUTPUT_LENGTH];

    for (Rank r1 = Deuce; r1 <= Ace; ++r1) {
        for (Rank r2 = Deuce; r2 <= Ace; ++r2) {
            if (r1 == r2) continue;

            for (Suit s = Club; s <= Spade; ++s) {
                hand[0] = NewCard(r1, Club);
                hand[1] = NewCard(r1, Diamond);
                hand[2] = NewCard(r1, Heart);
                hand[3] = NewCard(r1, Spade);
                hand[4] = NewCard(r2, s);

                snprintf(output, OUTPUT_LENGTH, "[%c%c%c%c%c] should be four of a kind",
                         ranks[r1], ranks[r1], ranks[r1], ranks[r1], ranks[r2]);
                CuAssert(tc, output, HandClassify(hand) == FourOfAKind);
            }
        }
    }
}


void TestHandClassifyFullHouse(CuTest* tc) {
    Card hand[5];
    char output[OUTPUT_LENGTH];

    for (Rank r1 = Deuce; r1 <= Ace; ++r1) {
        for (Rank r2 = Deuce; r2 <= Ace; ++r2) {
            if (r1 == r2) continue;

            hand[0] = NewCard(r1, Club);
            hand[1] = NewCard(r1, Diamond);
            hand[2] = NewCard(r1, Spade);
            hand[3] = NewCard(r2, Club);
            hand[4] = NewCard(r2, Diamond);

            snprintf(output, OUTPUT_LENGTH, "[%c%c%c%c%c] should be a fullhouse",
                     ranks[r1], ranks[r1], ranks[r1], ranks[r2], ranks[r2]);
            CuAssert(tc, output, HandClassify(hand) == FullHouse);
        }
    }
}


void TestHandClassifyFlush(CuTest* tc) {
    Card hand[5];
    char output[OUTPUT_LENGTH];
    const char *flushes[] = {
        "As 2s 3s 4s 6s",
        "As 2s 3s 4s 7s",
        "As 2s 3s 4s 8s",
        "As 2s 3s 4s 9s",
        "As 2s 3s 4s Ts",
        "As 2s 3s 4s Js",
        "As 2s 3s 4s Qs",
        "As 2s 3s 4s Ks",
        NULL,
    };

    for (size_t i = 0; flushes[i] != NULL; ++i) {
        snprintf(output, OUTPUT_LENGTH, "[%s] should be a flush", flushes[i]);
        NewHandFromString(flushes[i], hand);
        for (Suit s = Club; s <= Spade; ++s) {
            for (size_t j = 0; j < HAND_LENGTH; ++j) { hand[j].suit = s; }
            CuAssert(tc, output, HandClassify(hand) == Flush);
        }
    }
}


void TestHandClassifyStraight(CuTest* tc) {
    Card hand[5];
    char output[OUTPUT_LENGTH];
    const char *straights[] = {
        "6s 5c 4s 3h 2d",
        "7d 6s 5c 4s 3h",
        "8h 7d 6s 5c 4s",
        "9s 8h 7d 6s 5c",
        "Tc 9s 8h 7d 6s",
        "Jd Tc 9s 8h 7d",
        "Qh Jd Tc 9s 8h",
        "Ks Qh Jd Tc 9s",
        "As Ks Qh Jd Tc",
        NULL,
    };

    for (size_t i = 0; straights[i] != NULL; ++i) {
        snprintf(output, OUTPUT_LENGTH, "[%s] should be a straight", straights[i]);
        NewHandFromString(straights[i], hand);
        CuAssert(tc, output, HandClassify(hand) == Straight);
    }
}

void TestHandClassifyWheel(CuTest* tc) {
    Card hand[5];
    NewHandFromString("Ac 5c 4s 3h 2d", hand);
    CuAssert(tc, "A-5-4-3-2 should be a wheel", HandClassify(hand) == Wheel);
}


void TestHandClassifyTrips(CuTest* tc) {
    Card hand[5];
    char output[OUTPUT_LENGTH];

    for (Rank r1 = Deuce; r1 <= Ace; ++r1) {
        for (Rank r2 = Deuce; r2 <= Ace; ++r2) {
            if (r1 == r2) continue;

            for (Rank r3 = Deuce; r3 <= Ace; ++r3) {
                if (r3 == r1 || r3 == r2) continue;

                hand[0] = NewCard(r1, Club);
                hand[1] = NewCard(r1, Diamond);
                hand[2] = NewCard(r1, Spade);
                hand[3] = NewCard(r2, Club);
                hand[4] = NewCard(r3, Diamond);

                snprintf(output, OUTPUT_LENGTH, "[%c%c%c%c%c] should be three of a kind",
                         ranks[r1], ranks[r1], ranks[r1], ranks[r2], ranks[r3]);
                CuAssert(tc, output, HandClassify(hand) == ThreeOfAKind);
            }
        }
    }
}




void TestHandClassifyTwoPair(CuTest* tc) {
    Card hand[5];
    char output[OUTPUT_LENGTH];
    const char *twopairs[] = {
        "As Ac Ks Kc Qs",
        "As Ac Qs Qc Ks",
        "As Ac Js Jc Qs",
        "As Ac Ts Tc Qs",
        "As Ac 9s 9c Qs",
        "As Ac 8s 8c Qs",
        "As Ac 7s 7c Qs",
        "As Ac 6s 6c Qs",
        "As Ac 5s 5c Qs",
        "As Ac 4s 4c Qs",
        "As Ac 3s 3c Qs",
        "As Ac 2s 2c Qs",
        NULL,
    };

    for (size_t i = 0; twopairs[i] != NULL; ++i) {
        snprintf(output, OUTPUT_LENGTH, "[%s] should be a two pair", twopairs[i]);
        NewHandFromString(twopairs[i], hand);
        CuAssert(tc, output, HandClassify(hand) == TwoPair);
    }
}


void TestHandClassifyPair(CuTest* tc) {
    Card hand[5];
    char output[OUTPUT_LENGTH];
    const char *pairs[] = {
        "As Ac Ks 2c Qs",
        "As Ac Qs 2c Ks",
        "As Ac Js 2c Qs",
        "As Ac Ts 2c Qs",
        "As Ac 9s 2c Qs",
        "As Ac 8s 2c Qs",
        "As Ac 7s 2c Qs",
        "As Ac 6s 2c Qs",
        "As Ac 5s 2c Qs",
        "As Ac 4s 2c Qs",
        "As Ac 3s 2c Qs",
        "As Ac 2s 3c Qs",
        NULL,
    };

    for (size_t i = 0; pairs[i] != NULL; ++i) {
        snprintf(output, OUTPUT_LENGTH, "[%s] should be a pair", pairs[i]);
        NewHandFromString(pairs[i], hand);
        CuAssert(tc, output, HandClassify(hand) == Pair);
    }
}


void TestHandClassifyHighCard(CuTest* tc) {
    Card hand[5];
    char output[OUTPUT_LENGTH];
    const char *highcards[] = {
        "As 3c Ks 2c Qs",
        "As 3c Qs 2c Ks",
        "As 3c Js 2c Qs",
        "As 3c Ts 2c Qs",
        "As 3c 9s 2c Qs",
        "As 3c 8s 2c Qs",
        "As 3c 7s 2c Qs",
        "As 3c 6s 2c Qs",
        "As 3c 5s 2c Qs",
        "As 3c 4s 2c Qs",
        "As 4c 3s 2c Qs",
        "As 3c 2s 4c Qs",
        NULL,
    };

    for (size_t i = 0; highcards[i] != NULL; ++i) {
        snprintf(output, OUTPUT_LENGTH, "[%s] should be a high card", highcards[i]);
        NewHandFromString(highcards[i], hand);
        CuAssert(tc, output, HandClassify(hand) == HighCard);
    }
}


void TestHandSort(CuTest* tc) {
    Card hand1[5];
    Card hand2[5];
    char output[OUTPUT_LENGTH];

    const char *hands[] = {
        // Royal flush
        "Ts Js Qs Ks As", "As Ks Qs Js Ts",
        "Js Ts Qs Ks As", "As Ks Qs Js Ts",
        "Js Qs Ts Ks As", "As Ks Qs Js Ts",
        "Js Qs Ks Ts As", "As Ks Qs Js Ts",
        "Js Qs Ks As Ts", "As Ks Qs Js Ts",

        // Quads
        "Kc 8c 8d 8h 8s", "8c 8d 8h 8s Kc",
        "8c Kc 8d 8h 8s", "8c 8d 8h 8s Kc",
        "8d 8c Kc 8h 8s", "8c 8d 8h 8s Kc",
        "8c 8d 8h Kc 8s", "8c 8d 8h 8s Kc",
        "8c 8d 8h 8s Kc", "8c 8d 8h 8s Kc",

        // Full house
        "2c 2s Ac Ad Ah", "Ac Ad Ah 2s 2c",
        "2c Ac 2s Ad Ah", "Ac Ad Ah 2s 2c",
        "2c Ac Ad 2s Ah", "Ac Ad Ah 2s 2c",
        "2c Ac Ad Ah 2s", "Ac Ad Ah 2s 2c",
        "Ac 2c 2s Ad Ah", "Ac Ad Ah 2s 2c",
        "Ac 2c Ad 2s Ah", "Ac Ad Ah 2s 2c",
        "Ac 2c Ad Ah 2s", "Ac Ad Ah 2s 2c",
        "Ac Ad 2c 2s Ah", "Ac Ad Ah 2s 2c",
        "Ac Ad 2c Ah 2s", "Ac Ad Ah 2s 2c",
        "Ac Ad Ah 2c 2s", "Ac Ad Ah 2s 2c",

        "Kc Ks 3c 3d 3h", "3c 3d 3h Ks Kc",
        "Kc 3c Ks 3d 3h", "3c 3d 3h Ks Kc",
        "Kc 3c 3d Ks 3h", "3c 3d 3h Ks Kc",
        "Kc 3c 3d 3h Ks", "3c 3d 3h Ks Kc",
        "3c Kc Ks 3d 3h", "3c 3d 3h Ks Kc",
        "3c Kc 3d Ks 3h", "3c 3d 3h Ks Kc",
        "3c Kc 3d 3h Ks", "3c 3d 3h Ks Kc",
        "3c 3d Kc Ks 3h", "3c 3d 3h Ks Kc",
        "3c 3d Kc 3h Ks", "3c 3d 3h Ks Kc",
        "3c 3d 3h Kc Ks", "3c 3d 3h Ks Kc",

        // Trips
        "2c 3s Ac Ad Ah", "Ac Ad Ah 3s 2c",
        "2c Ac 3s Ad Ah", "Ac Ad Ah 3s 2c",
        "2c Ac Ad 3s Ah", "Ac Ad Ah 3s 2c",
        "2c Ac Ad Ah 3s", "Ac Ad Ah 3s 2c",
        "Ac 2c 3s Ad Ah", "Ac Ad Ah 3s 2c",
        "Ac 2c Ad 3s Ah", "Ac Ad Ah 3s 2c",
        "Ac 2c Ad Ah 3s", "Ac Ad Ah 3s 2c",
        "Ac Ad 2c 3s Ah", "Ac Ad Ah 3s 2c",
        "Ac Ad 2c Ah 3s", "Ac Ad Ah 3s 2c",
        "Ac Ad Ah 2c 3s", "Ac Ad Ah 3s 2c",

        "3c 2s Ac Ad Ah", "Ac Ad Ah 3s 2c",
        "3c Ac 2s Ad Ah", "Ac Ad Ah 3s 2c",
        "3c Ac Ad 2s Ah", "Ac Ad Ah 3s 2c",
        "3c Ac Ad Ah 2s", "Ac Ad Ah 3s 2c",
        "Ac 3c 2s Ad Ah", "Ac Ad Ah 3s 2c",
        "Ac 3c Ad 2s Ah", "Ac Ad Ah 3s 2c",
        "Ac 3c Ad Ah 2s", "Ac Ad Ah 3s 2c",
        "Ac Ad 3c 2s Ah", "Ac Ad Ah 3s 2c",
        "Ac Ad 3c Ah 2s", "Ac Ad Ah 3s 2c",
        "Ac Ad Ah 3c 2s", "Ac Ad Ah 3s 2c",

        NULL,
    };

    for (size_t i = 0; hands[i] != NULL; i += 2) {
        snprintf(output, OUTPUT_LENGTH, "[%s] != [%s]", hands[i], hands[i+1]);
        NewHandFromString(hands[i], hand1);
        NewHandFromString(hands[i+1], hand2);
        HandSort(hand1);
        CuAssert(tc, output,
                 CardCompare(&hand1[0], &hand2[0]) == 0 &&
                 CardCompare(&hand1[1], &hand2[1]) == 0 &&
                 CardCompare(&hand1[2], &hand2[2]) == 0 &&
                 CardCompare(&hand1[3], &hand2[3]) == 0 &&
                 CardCompare(&hand1[4], &hand2[4]) == 0);

    }
}


void TestHandCompare(CuTest* tc) {
    Card hand1[5];
    Card hand2[5];
    char output[OUTPUT_LENGTH];
    char output2[OUTPUT_LENGTH];

    const char *hands[] = {
        // Royal flush
        "Ts Js Qs Ks As", "9s Ts Js Qs Ks", // vs straight flush
        "Ts Js Qs Ks As", "As Ad Ks Ah Ac", // vs quads
        "Ts Js Qs Ks As", "As Ad Ks Kc Ah", // vs full house
        "Ts Js Qs Ks As", "As 3s 5s 7s 8s", // vs flush
        "Ts Js Qs Ks As", "3s 4d 5h 6c 7s", // vs straight
        "Ts Js Qs Ks As", "As 2d Ac 3h Ah", // vs trips
        "Ts Js Qs Ks As", "As Ks Ad Kd 3h", // vs two pair
        "Ts Js Qs Ks As", "As 2d 3h 4c Ac", // vs pair
        "Ts Js Qs Ks As", "As 9d 8h 3s 2s", // vs high card

        // All straight flushes are stronger than the wheel flush
        "Ac Kc Qc Jc Tc", "5c 4c 3c 2c Ac",
        "Kc Qc Jc Tc 9c", "5c 4c 3c 2c Ac",
        "Qc Jc Tc 9c 8c", "5c 4c 3c 2c Ac",
        "Jc Tc 9c 8c 7c", "5c 4c 3c 2c Ac",
        "Tc 9c 8c 7c 6c", "5c 4c 3c 2c Ac",
        "9c 8c 7c 6c 5c", "5c 4c 3c 2c Ac",
        "8c 7c 6c 5c 4c", "5c 4c 3c 2c Ac",
        "7c 6c 5c 4c 3c", "5c 4c 3c 2c Ac",
        "6c 5c 4c 3c 2c", "5c 4c 3c 2c Ac",

        // Quads
        "Ac Ad Ah As Ks", "Ac Ad Ah As Qs", // vs quads, lesser kicker
        "Ac Ad Ah As Ks", "Kc Kd Kh Ks As", // vs quads, lesser quads
        "Ac Ad Ah As Ks", "As Ad Ks Kc Ah", // vs full house
        "Ac Ad Ah As Ks", "As 3s 5s 7s 8s", // vs flush
        "Ac Ad Ah As Ks", "3s 4d 5h 6c 7s", // vs straight
        "Ac Ad Ah As Ks", "As 2d Ac 3h Ah", // vs trips
        "Ac Ad Ah As Ks", "As Ks Ad Kd 3h", // vs two pair
        "Ac Ad Ah As Ks", "As 2d 3h 4c Ac", // vs pair
        "Ac Ad Ah As Ks", "As 9d 8h 3s 2s", // vs high card

        // Full house
        "Ac Ad Ah Kd Ks", "Ac Ad Ah Qd Qs", // vs full house, lesser twos
        "Ac Ad Ah Kd Ks", "Qc Qd Qh Ac Ad", // vs full house, lesser threes
        "Ac Ad Ah Kd Ks", "As 3s 5s 7s 8s", // vs flush
        "Ac Ad Ah Kd Ks", "3s 4d 5h 6c 7s", // vs straight
        "Ac Ad Ah Kd Ks", "As 2d Ac 3h Ah", // vs trips
        "Ac Ad Ah Kd Ks", "As Ks Ad Kd 3h", // vs two pair
        "Ac Ad Ah Kd Ks", "As 2d 3h 4c Ac", // vs pair
        "Ac Ad Ah Kd Ks", "As 9d 8h 3s 2s", // vs high card

        // Flush
        "Ac Qc Tc 8c 6c", "Ac Qc Tc 8c 5c", // vs lesser flush 1
        "Ac Qc Tc 8c 6c", "Ac Qc Tc 7c 6c", // vs lesser flush 2
        "Ac Qc Tc 8c 6c", "Ac Qc 9c 8c 6c", // vs lesser flush 3
        "Ac Qc Tc 8c 6c", "Ac Jc Tc 8c 6c", // vs lesser flush 4
        "Ac Qc Tc 8c 6c", "Kc Qc Tc 8c 6c", // vs lesser flush 5
        "Ac Qc Tc 8c 6c", "3s 4d 5h 6c 7s", // vs straight
        "Ac Qc Tc 8c 6c", "As 2d Ac 3h Ah", // vs trips
        "Ac Qc Tc 8c 6c", "As Ks Ad Kd 3h", // vs two pair
        "Ac Qc Tc 8c 6c", "As 2d 3h 4c Ac", // vs pair
        "Ac Qc Tc 8c 6c", "As 9d 8h 3s 2s", // vs high card

        // Straight
        "Ac Kd Qh Js Tc", "Kd Qh Js Tc 9d", // vs lesser straight
        "Ac Kd Qh Js Tc", "As 2d Ac 3h Ah", // vs trips
        "Ac Kd Qh Js Tc", "As Ks Ad Kd 3h", // vs two pair
        "Ac Kd Qh Js Tc", "As 2d 3h 4c Ac", // vs pair
        "Ac Kd Qh Js Tc", "As 9d 8h 3s 2s", // vs high card

        // All straights are stronger than a wheel
        "Ac Kd Qh Js Tc", "5d 4h 3s 2c Ac",
        "Kd Qh Js Tc 9d", "5d 4h 3s 2c Ac",
        "Qh Js Tc 9d 8h", "5d 4h 3s 2c Ac",
        "Js Tc 9d 8h 7s", "5d 4h 3s 2c Ac",
        "Tc 9d 8h 7s 6s", "5d 4h 3s 2c Ac",
        "9d 8h 7s 6s 5c", "5d 4h 3s 2c Ac",
        "8h 7s 6s 5c 4d", "5d 4h 3s 2c Ac",
        "7s 6s 5c 4d 3h", "5d 4h 3s 2c Ac",
        "6s 5c 4d 3h 2s", "5d 4h 3s 2c Ac",

        // Trips
        "As Ac Ah Jh 8d", "As Ac Ah Jh 7d", // vs trips, lesser kicker 1
        "As Ac Ah Jh 8d", "As Ac Ah Th 8d", // vs trips, lesser kicker 2
        "As Ac Ah Jh 8d", "Ks Kc Kh Ah Qd", // vs trips, lesser three cards
        "As Ac Ah Jh 8d", "As Ks Ad Kd 3h", // vs two pair
        "As Ac Ah Jh 8d", "As 2d 3h 4c Ac", // vs pair
        "As Ac Ah Jh 8d", "As 9d 8h 3s 2s", // vs high card

        // Two pair
        "Ac Ad Jh Js 8s", "Ac Ad Jh Js 7s", // vs two pair, lesser kicker
        "Ac Ad Jh Js 8s", "Ac Ad Th Ts Qs", // vs two pair, lesser low pair
        "Ac Ad Jh Js 8s", "Kc Kd Jh Js Qs", // vs two pair, lesser high pair
        "Ac Ad Jh Js 8s", "As 2d 3h 4c Ac", // vs pair
        "Ac Ad Jh Js 8s", "As 9d 8h 3s 2s", // vs high card

        // Pair
        "Ac Ad Qh 8s 4c", "Ac Ad Qh 8s 3c", // vs pair, lesser kicker 1
        "Ac Ad Qh 8s 4c", "Ac Ad Qh 7s 4c", // vs pair, lesser kicker 2
        "Ac Ad Qh 8s 4c", "Ac Ad Jh 8s 4c", // vs pair, lesser kicker 3
        "Ac Ad Qh 8s 4c", "Kc Kd Qh 8s 4c", // vs pair, lesser pair
        "Ac Ad Qh 8s 4c", "As 9d 8h 3s 2s", // vs high card

        // High card
        "Ac Qd Th 8s 6c", "Ac Qd Th 8s 5c", // vs high card, lesser kicker 1
        "Ac Qd Th 8s 6c", "Ac Qd Th 7s 6c", // vs high card, lesser kicker 2
        "Ac Qd Th 8s 6c", "Ac Qd 9h 8s 6c", // vs high card, lesser kicker 3
        "Ac Qd Th 8s 6c", "Ac Jd Th 8s 6c", // vs high card, lesser kicker 4
        "Ac Qd Th 8s 6c", "Kc Qd Th 8s 6c", // vs high card, lesser kicker 5
        NULL,
    };

    for (size_t i = 0; hands[i] != NULL; i += 2) {
        NewHandFromString(hands[i], hand1);
        NewHandFromString(hands[i+1], hand2);
        HandSort(hand1);
        HandSort(hand2);

        snprintf(output, OUTPUT_LENGTH, "[%s] should be greater than [%s]", hands[i], hands[i+1]);
        snprintf(output2, OUTPUT_LENGTH, "[%s] should be equal to [%s]", hands[i], hands[i+1]);
        CuAssert(tc, output, HandCompare(hand1, hand2) > 0);
        CuAssert(tc, output, HandCompare(hand2, hand1) < 0);
        CuAssert(tc, output2, HandCompare(hand1, hand1) == 0);
        CuAssert(tc, output2, HandCompare(hand2, hand2) == 0);
    }
}



CuSuite* HandUtilSuite() {
    CuSuite* suite = CuSuiteNew();

    SUITE_ADD_TEST(suite, TestHandClassifyStraightFlush);
    SUITE_ADD_TEST(suite, TestHandClassifyQuads);
    SUITE_ADD_TEST(suite, TestHandClassifyFullHouse);
    SUITE_ADD_TEST(suite, TestHandClassifyFlush);
    SUITE_ADD_TEST(suite, TestHandClassifyStraight);
    SUITE_ADD_TEST(suite, TestHandClassifyTrips);
    SUITE_ADD_TEST(suite, TestHandClassifyTwoPair);
    SUITE_ADD_TEST(suite, TestHandClassifyPair);
    SUITE_ADD_TEST(suite, TestHandClassifyHighCard);
    SUITE_ADD_TEST(suite, TestHandSort);
    SUITE_ADD_TEST(suite, TestHandCompare);

    return suite;
}
