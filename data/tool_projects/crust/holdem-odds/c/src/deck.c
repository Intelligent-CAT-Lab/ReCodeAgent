#include <stdlib.h>

#include "cards.h"
#include "deck.h"


/*
 * Shuffle the n first cards in a deck of `deck_size` cards.
 */
void DeckShuffle(Card* deck, size_t n, size_t deck_size) {
    for (size_t i = 0; i < n; ++i) {
        size_t j = rand() % deck_size;
        CardSwap(&deck[i], &deck[j]);
    }
}


/*
 * Fills up deck with all cards in order except for those found in
 * rejects.
 */
void NewDeck(Card* deck, const Card* rejects, size_t nb_rejects) {
    size_t i = 0;

    for (Rank r = Deuce; r <= Ace; ++r) {
        for (Suit s = Club; s <= Spade; ++s) {
            Card c = NewCard(r, s);
            bool insert = true;

            for (size_t j = 0; j < nb_rejects; ++j) {
                if (CardEqual(&c, &rejects[j])) {
                    insert = false;
                    break;
                }
            }

            if (insert) {
                deck[i] = c;
                i++;
            }
        }
    }
}
