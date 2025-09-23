#ifndef _HANDS_H

#define _HANDS_H

#include <stddef.h>
#include <stdbool.h>

#include "cards.h"

#define HAND_LENGTH 5


typedef enum {
    InvalidHand, HighCard, Pair, TwoPair, ThreeOfAKind,
    Wheel, Straight, Flush, FullHouse, FourOfAKind, WheelFlush,
    StraightFlush,
} HandType;


int      HandCompare(Card*, Card*);
HandType HandClassify(Card*);
void     HandSort(Card*);
bool     HandIsValid(const Card*);

#endif
