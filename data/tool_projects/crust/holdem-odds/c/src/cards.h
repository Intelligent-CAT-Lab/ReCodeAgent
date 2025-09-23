#ifndef _CARDS_H

#define _CARDS_H

#include <stdlib.h>
#include <stdbool.h>

#define RANKS_PER_DECK 13
#define SUITS_PER_DECK 4


#define RANK_CHARS "23456789TJQKA"
#define SUIT_CHARS "cdhs"


typedef enum { InvalidSuit, Club, Diamond, Heart, Spade } Suit;
typedef enum {
    InvalidRank, Deuce, Trey, Four, Five, Six, Seven,
    Eight, Nine, Ten, Jack, Queen, King, Ace
} Rank;

typedef struct {
    Rank rank;
    Suit suit;
} Card;



size_t    indexOf(char, const char*, size_t);

/* Constructors */
Card      NewCard(Rank, Suit);
Card      NewCardFromChars(char, char);
Card      NewCardFromString(char*);

bool      CardIsValid(const Card *);
Rank      CharToRank(char);
Suit      CharToSuit(char);
int       CardCompare(const Card*, const Card*);
bool      CardEqual(const Card*, const Card*);
void      CardToString(char *, const Card*);
void      CardSwap(Card*, Card*);


#endif
