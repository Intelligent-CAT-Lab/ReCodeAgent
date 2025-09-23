#include <stdbool.h>
#include <string.h>

#include "bucket.h"
#include "cards.h"
#include "hands.h"


static bool HandIsStraightFlush(const Card*);
static bool HandIsWheelFlush(const Card*);
static bool HandIsFourOfAKind(const Card*);
static bool HandIsFullHouse(const Card*);
static bool HandIsFlush(const Card*);
static bool HandIsStraight(const Card*);
static bool HandIsWheel(const Card*);
static bool HandIsThreeOfAKind(const Card*);
static bool HandIsTwoPair(const Card*);
static bool HandIsPair(const Card*);
static bool HandIsHighCard(const Card*);


/*
 * Compare two hands and return:
 *   < 0 if hand1 < hand2
 *     0 if hand1 = hand2
 *   > 0 if hand1 > hand2
 */
int HandCompare(Card* hand1, Card* hand2) {
    HandType hand1_type = HandClassify(hand1);
    HandType hand2_type = HandClassify(hand2);
    int diff = hand1_type - hand2_type;

    if (diff != 0)
        return diff;

    // Tie breaker when two hands of the same type.
    // Find the strongest hand by going card-by-card
    // until one card is higher.  If no card is higher
    // we have a tie.
    for (size_t i = 0; i < HAND_LENGTH; ++i) {
        int cmp = CardCompare(&hand1[i], &hand2[i]);
        if (cmp != 0)
            return cmp;
    }
    return 0;
}


/*
 * Sort the cards by the number of cards of the same
 * rank that appear in the hand and then by rank from
 * strongest to weakest.
 */
void HandSort(Card* hand) {
    Bucket buckets[RANKS_PER_DECK] = {{{{InvalidRank, InvalidSuit}}, 0}};

    // Copy the cards into the buckets.
    for (size_t i = 0; i < HAND_LENGTH; ++i) {
        size_t bucket_index = hand[i].rank - 1;
        BucketAdd(&buckets[bucket_index], &hand[i]);
    }

    size_t index = 0; // Index of the next card to insert in hand

    // Search for buckets with 4 cards first, then 3, 2 and 1.
    for (size_t count = 4; count > 0; --count) {

        // Go from the highest rank to the lowest rank when inserting into hand
        for (size_t j = RANKS_PER_DECK - 1; j != (size_t)-1; --j) {

            // If the number of cards in the bucket matches the number
            // we're looking for, insert them into the hand.
            if (buckets[j].count == count) {
                memcpy(&hand[index], &buckets[j].cards, sizeof(Card) * count);
                index += count;
            }
        }
    }
}



/*
 * Find the type of the hand that's been given to us.  Go
 * from strongest to weakest type; this assures us that a
 * full house is not classified as merely a pair.
 *
 * The cards *must* be sorted in order for this function
 * to work properly.
 */
HandType HandClassify(Card* cards) {
    if (HandIsFlush(cards)) {
        if (HandIsStraight(cards)) return StraightFlush;
        if (HandIsWheel(cards))    return WheelFlush;
        return Flush;
    }
    else {
        if (HandIsFourOfAKind(cards))   return FourOfAKind;
        if (HandIsFullHouse(cards))     return FullHouse;
        if (HandIsStraight(cards))      return Straight;
        if (HandIsWheel(cards))         return Wheel;
        if (HandIsThreeOfAKind(cards))  return ThreeOfAKind;
        if (HandIsTwoPair(cards))       return TwoPair;
        if (HandIsPair(cards))          return Pair;
        return HighCard;
    }
}


/*
 * A valid hand has 5 cards in decreasing order.
 */
bool HandIsValid(const Card* cards) {
    for (size_t i = 0; i < HAND_LENGTH; ++i)
        if (!CardIsValid(&cards[i]))
            return false;

    return true;
}


/*
 * A straight flush is a straight _and_ a flush.
 */
static bool HandIsStraightFlush(const Card* cards) {
    return HandIsStraight(cards) && HandIsFlush(cards);
}


/*
 * A wheel flush is a wheel _and_ a flush.
 */
static bool HandIsWheelFlush(const Card* cards) {
    return HandIsWheel(cards) && HandIsFlush(cards);
}


/*
 * Four of a kind is when 4 of the 5 cards have the same rank.
 */
static bool HandIsFourOfAKind(const Card* cards) {
    // If the cards are properly ordered, only in the
    // case of quads should the first and fourth card
    // be of the same rank.
    return cards[0].rank == cards[3].rank;
}


/*
 * A full house is when a hand is composed of a pair and three of a
 * kind.
 */
static bool HandIsFullHouse(const Card* cards) {
    // If the cards are properly ordered, only in the
    // case of a full house should the first and third
    // card be the same and the last two cards be the same.
    return cards[0].rank == cards[2].rank
        && cards[3].rank == cards[4].rank;
}


/*
 * A flush is a hand where all cards have the same suit.
 */
static bool HandIsFlush(const Card* cards) {
    for (size_t i = 1; i < HAND_LENGTH; ++i)
        if (cards[i].suit != cards[0].suit)
            return false;

    return true;
}


/*
 * A straight is a hand where all cards are successive.  The Ace
 * is considered both high and low, meaning it can follow a King
 * or precede a Deuce.
 */
static bool HandIsStraight(const Card* cards) {
    // If the cards are properly ordered, if the first
    // card's rank is equal to the rank of the next card
    // plus 1 (meaning they have different ranks) and if
    // the fifth card is 4 ranks lower than the first card,
    // then we have a straight.
    return cards[0].rank == cards[1].rank + 1
        && cards[0].rank == cards[4].rank + 4;
}


/*
 * A wheel is a straight from A to 5
 */
static bool HandIsWheel(const Card* cards) {
       return cards[0].rank == Ace && cards[1].rank == Five;
}


/*
 * A hand is three of a kind if three cards have the same rank
 * and the other two cards don't match.
 */
static bool HandIsThreeOfAKind(const Card* cards) {
    // If the cards are properly ordered and a full house
    // wasn't found, if the first and third cards are of
    // the same rank, we have trips.
    return cards[0].rank == cards[2].rank;
}


/*
 * A hand has two pairs if two sets of two cards share
 * the same rank and the other card doesn't match.
 */
static bool HandIsTwoPair(const Card* cards) {
    // If the4 cards are properly ordered and quads
    // haven't been found, if the third and fourth
    // cards are of the same rank, we have two pairs.
    return cards[2].rank == cards[3].rank;
}


/*
 * A hand is a pair if two cards share the same rank
 * and the other three cards all have different ranks.
 */
static bool HandIsPair(const Card* cards) {
    return cards[0].rank == cards[1].rank;
}


/*
 * A hand is high card if there is no pair, no straight or
 * no flush.  (Any hand is a high card hand)
 */
static bool HandIsHighCard(const Card* cards) {
    (void)cards;
    return true;
}
