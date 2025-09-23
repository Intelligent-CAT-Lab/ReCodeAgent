#ifndef _DECK_H

#define _DECK_H

#define CARDS_PER_DECK 52

#include "cards.h"



void DeckShuffle(Card*, size_t, size_t);
void NewDeck(Card*, const Card*, size_t);

#endif
