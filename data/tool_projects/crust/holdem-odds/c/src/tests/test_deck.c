#include <stdio.h>
#include <stdbool.h>

#include "CuTest.h"
#include "cards.h"
#include "deck.h"


#define OUTPUT_LENGTH 512

void TestDeckShuffle(CuTest* tc) {
    Card deck1[CARDS_PER_DECK];
    Card deck2[CARDS_PER_DECK];

    size_t index = 0;
    for (Rank r = Deuce; r <= Ace; ++r) {
        for (Suit s = Club; s <= Spade; ++s) {
            deck1[index] = NewCard(r, s);
            deck2[index] = NewCard(r, s);
            index++;
        }
    }

    DeckShuffle(deck2, CARDS_PER_DECK, CARDS_PER_DECK);

    // Make sure all the cards are valid
    for (size_t i = 0; i < CARDS_PER_DECK; ++i) {
        char s[3];
        CardToString(s, &deck2[i]);
        CuAssert(tc, s, CardIsValid(&deck2[i]));
    }

    // There's a 1/52! chance that this test will fail; we'll take that chance.
    bool decks_differ = false;
    for (size_t i = 0; i < CARDS_PER_DECK; ++i) {
        if (CardCompare(&deck1[i], &deck2[i]) != 0) {
            decks_differ = true;
            break;
        }
    }
    CuAssert(tc, "deck2 was not shuffled", decks_differ);


}


void TestNewDeck(CuTest* tc) {
    Card deck[CARDS_PER_DECK];
    NewDeck(deck, NULL, 0);

    size_t i = 0;
    char cardStr1[3];
    char cardStr2[3];
    char output[OUTPUT_LENGTH];

    for (Rank r = Deuce; r <= Ace; ++r) {
        for (Suit s = Club; s <= Spade; ++s) {
            Card c = NewCard(r, s);
            CardToString(cardStr1, &c);
            CardToString(cardStr2, &deck[i]);
            snprintf(output, OUTPUT_LENGTH, "%s != %s", cardStr1, cardStr2);
            CuAssert(tc, output, CardCompare(&c, &deck[i]) == 0);
            i++;
        }
    }

}


CuSuite* DeckSuite() {
    CuSuite* suite = CuSuiteNew();

    SUITE_ADD_TEST(suite, TestDeckShuffle);
    SUITE_ADD_TEST(suite, TestNewDeck);

    return suite;
}
