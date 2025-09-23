#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "cards.h"


Card INVALID_CARD = { .rank = InvalidRank, .suit = InvalidSuit };


/*
 * Return the 1-based index of c in chars.  Return
 * 0 if the character isn't found.
 */
size_t indexOf(char c, const char* chars, size_t N) {
    for (size_t i = 0; i < N; ++i) {
        if (chars[i] == c) {
            return i+1;
        }
    }
    return 0;
}


/*
 * Convert a char to the rank it represents or InvalidRank.
 */
Rank CharToRank(char r) {
    return (Rank)indexOf(r, RANK_CHARS, RANKS_PER_DECK);
}


/*
 * Convert a char to the suit it represents or InvalidSuit.
 */
Suit CharToSuit(char s) {
    return (Suit)indexOf(s, SUIT_CHARS, SUITS_PER_DECK);
}


/*
 * Create a new card and return it.  rank and suit are set
 * to InvalidRank and InvalidSuit respectively if the
 * parameters are invalid.
 */
Card NewCard(Rank rank, Suit suit) {
    Card card = { .rank = rank, .suit = suit };

    if (CardIsValid(&card))
        return card;
    else
        return INVALID_CARD;
}


/*
 * Create a new card from two chars representing the rank
 * and the suit.
 */
Card NewCardFromChars(char r, char s) {
    Rank rank = CharToRank(r);
    Suit suit = CharToSuit(s);

    return NewCard(rank, suit);
}


/*
 * Create a new card from a 2-char string representing
 * the rank and the suit.
 */
Card NewCardFromString(char *s) {
    if (strlen(s) != 2)
        return INVALID_CARD;
    else
        return NewCardFromChars(s[0], s[1]);
}


/*
 * Compare two cards and return:
 *   < 0 if a is smaller than b
 *     0 if a is equal to b
 *   > 0 if a is greater than b
 */
int CardCompare(const Card* a, const Card* b) {
    return a->rank - b->rank;
}


/*
 * Return whether two cards have the same rank and suit.
 */
bool CardEqual(const Card* a, const Card* b) {
    return a->rank == b->rank && a->suit == b->suit;
}


/*
 * Convert a card to its 2-char string representation.
 */
void CardToString(char* out, const Card* c) {
    if (!CardIsValid(c)) {
        out[0] = '-';
        out[1] = '-';
    }
    else {
        out[0] = RANK_CHARS[(int)c->rank - 1];
        out[1] = SUIT_CHARS[(int)c->suit - 1];
    }
    out[2] = '\0';
}


/*
 * Returns whether a card has valid values for its
 * rank and suit.
 */
bool CardIsValid(const Card* c) {
    return (c->rank >= Deuce && c->rank <= Ace &&
            c->suit >= Club  && c->suit <= Spade);
}


/*
 * Swap two cards.
 */
void CardSwap(Card* a, Card* b) {
    Card t = *a;
    *a = *b;
    *b = t;
}



