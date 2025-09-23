/*****************************************************************************
 * Copyright (C) 2010 Tadeus Prastowo (eus@member.fsf.org)                   *
 *                                                                           *
 * This program is free software: you can redistribute it and/or modify      *
 * it under the terms of the GNU General Public License as published by      *
 * the Free Software Foundation, either version 3 of the License, or         *
 * (at your option) any later version.                                       *
 *                                                                           *
 * This program is distributed in the hope that it will be useful,           *
 * but WITHOUT ANY WARRANTY; without even the implied warranty of            *
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the             *
 * GNU General Public License for more details.                              *
 *                                                                           *
 * You should have received a copy of the GNU General Public License         *
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.     *
 *************************************************************************//**
 * @file card.h
 * @brief Objects and functions dealing with the 52 cards.
 * @author Tadeus Prastowo <eus@member.fsf.org>
 ****************************************************************************/

#include <stdint.h>

#ifndef CARD_H
#define CARD_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * The 52 cards. It is guaranteed that the place of the existing enum values
 * will never change.
 */
enum card_suit_rank
  {
    SPADE_ACE, SPADE_2, SPADE_3, SPADE_4, SPADE_5, SPADE_6, SPADE_7, SPADE_8,
    SPADE_9, SPADE_10, SPADE_J, SPADE_Q, SPADE_K,

    HEART_ACE, HEART_2, HEART_3, HEART_4, HEART_5, HEART_6, HEART_7, HEART_8,
    HEART_9, HEART_10, HEART_J, HEART_Q, HEART_K,

    DIAMOND_ACE, DIAMOND_2, DIAMOND_3, DIAMOND_4, DIAMOND_5, DIAMOND_6,
    DIAMOND_7, DIAMOND_8, DIAMOND_9, DIAMOND_10, DIAMOND_J, DIAMOND_Q, DIAMOND_K,

    CLUB_ACE, CLUB_2, CLUB_3, CLUB_4, CLUB_5, CLUB_6, CLUB_7, CLUB_8, CLUB_9,
    CLUB_10, CLUB_J, CLUB_Q, CLUB_K,

    CARD_COUNT,

    INVALID_CARD,
  };

/**
 * The rank of a card. It is guaranteed that the place of the existing enum
 * values will never change.
 */
enum card_rank
  {
    ACE, R2, R3, R4, R5, R6, R7, R8, R9, R10, J, Q, K,

    RANK_COUNT,

    INVALID_RANK,
  };

/**
 * The suit of a card. It is guaranteed that the place of the existing enum
 * values will never change.
 */
enum card_suit
  {
    SPADE, HEART, DIAMOND, CLUB,

    SUIT_COUNT,

    INVALID_SUIT,
  };

/** A card having a particular suit and rank. */
typedef struct card_impl card;

/** A hand of cards. */
typedef struct card_hand_impl card_hand;

/** A deck of cards. */
typedef struct card_deck_impl card_deck;

/**
 * Returns the suit and rank of the card.
 *
 * @param [in] c the card to be determined.
 *
 * @return the suit and the rank of this card.
 */
enum card_suit_rank
get_card_suit_rank (const card *c);

/**
 * Returns the rank of this card.
 *
 * @param [in] c the card to be determined.
 *
 * @return the rank of this card.
 */
enum card_rank
get_card_rank (const card *c);

/**
 * Returns the suit of this card.
 *
 * @param [in] c the card to be determined.
 *
 * @return the suit of this card.
 */
enum card_suit
get_card_suit (const card *c);

/**
 * Creates a card having the desired suit and rank.
 * The returned card has to be freed with destroy_card().
 *
 * @param [in] csr the desired suit and rank.
 *
 * @return an immutable card or NULL if the card cannot be created due to
 *         invalid csr or insufficient memory.
 */
const card *
create_card (enum card_suit_rank csr);

/**
 * Creates a card whose suit and rank are specified in the string.
 * The returned card has to be freed with destroy_card().
 *
 * @param [in] str the string representation of a suit and rank.
 *
 * @return an immutable card if the string representation is valid
 *         or NULL if it is invalid or the card cannot be created.
 */
const card *
strtocard (const char *str);

/**
 * Returns the string representation of a card.
 *
 * @param [in] c the card.
 *
 * @return an immutable string if the card is valid or NULL if it is invalid.
 */
const char *
cardtostr (enum card_suit_rank c);

/**
 * Returns the string representation of a rank.
 *
 * @param [in] r the rank.
 *
 * @return an immutable string if the rank is valid or NULL if it is invalid.
 */
const char *
ranktostr (enum card_rank r);

/**
 * Reclaims the memory space that was allocated for the card as well as
 * setting the pointer to NULL as a safe guard. The card to be freed
 * must be a card obtained from strtocard() or create_card(). Passing
 * a pointer pointing to NULL is safe but not a NULL pointer.
 *
 * @param [in] c_ptr the pointer pointing to a card to be freed.
 */
void
destroy_card (const card **c_ptr);

/**
 * Returns the rank specified in the string.
 *
 * @param [in] str the string representation of a rank.
 *
 * @return the rank or ::INVALID_RANK if the string is invalid.
 */
enum card_rank
strtorank (const char *str);

/**
 * Determines where a card should be inserted in a collection of cards.
 * Several predefined card sorters are:
 * sort_card_after() and
 * sort_card_by_rank().
 *
 * @param [in] before the card before this candidate insertion place (this is
 *                    NULL for the place before the first element).
 * @param [in] new the card to be inserted.
 * @param [in] afer the card after this candidate insertion place (this is
 *                  NULL for the place after the last element).
 *
 * @return zero if the new card should not be inserted in this candidate place
 *         or non-zero if the card should be inserted in this candidate place.
 */
typedef int (*card_sorter) (const card *before, const card *new,
			    const card *after);

/**
 * Inserts a new card after the last inserted card (to be used with
 * create_hand()).
 */
int
sort_card_after (const card *before, const card *new, const card *after);

/**
 * Inserts a new card after a card with a lower rank but before a card with
 * a higher or equal rank (to be used with create_hand()).
 */
int
sort_card_by_rank (const card *before, const card *new, const card *after);

/**
 * Creates an empty hand to hold cards. The returned card hand has to be freed
 * with destroy_hand().
 * 
 * @param [in] max the maximum number of cards that the hand can contain.
 * @param [in] sorter the callback function to determine the place where a card
 *                    should be inserted in a hand. If this is NULL,
 *                    sort_card_after() is used as the default.
 *
 * @return an empty card hand or NULL if an empty card hand cannot be created.
 */
card_hand *
create_hand (unsigned char max, card_sorter sorter);

/**
 * Resets hand empties a hand without freeing the cards removed from the hand.
 * This should not burden you unless you have ever inserted a card that
 * you have ever created directly using create_card() or strtocard(). In that
 * case, you have to free the card yourself using destroy_card().
 *
 * @param [in] h the hand to reset.
 */
void
reset_hand (card_hand *h);

/**
 * Inserts a card into a card hand. The card will not be inserted into the hand
 * if the hand is full already. You should check whether or not you can still
 * insert a card into a hand first. The inserted card must not be freed before
 * its reference is removed from the hand because further operation on the hand
 * may segfault.
 *
 * @param [out] h the hand into which the card will be inserted.
 * @param [in] c the card to be inserted into the hand.
 */
void
insert_into_hand (card_hand *h, const card *c);

/**
 * Counts the number of cards currently in hand.
 *
 * @param [in] h the hand whose number of cards is to be determined.
 *
 * @return the number of cards in hand.
 */
unsigned long
count_cards_in_hand (const card_hand *h);

/**
 * Returns the number maximum number of cards that the hand can contain.
 *
 * @param [in] h the hand whose maximum number of cards is to be determined.
 *
 * @return the maximum number of cards of the hand.
 */
unsigned long
get_max_of_hand (const card_hand *h);

/**
 * Determines the maximum rank in the hand. If the hand is empty, the result is
 * ::INVALID_RANK. You should always check first whether or not the hand is
 * empty.
 *
 * @param [in] h the card hand to be checked.
 *
 * @return the maximum rank in the hand.
 */
enum card_rank
get_max_rank_of_hand (const card_hand *h);

/** What the iterator should do. */
enum itr_action
  {
    CONTINUE, /**< Continue iterating. */
    BREAK, /**< Stop iterating. */
    REMOVE_AND_CONTINUE, /**<
			  * Remove the card under the iterator from the hand
			  * and continue iterating with the next card. The
			  * pos and len values supplied to the callback function
			  * in the following iteration will stay constant as in
			  * this current iteration reflecting the removal.
			  */
    REMOVE_AND_BREAK, /**<
		       * Remove the card under the iterator from the hand
		       * and stop iterating.
		       */
  };

/**
 * The callback function used to iterate the cards in a hand.
 * Since iterating a data structure with more than one iterators at once
 * where not all iterators is reader clearly is an equivalent of doing
 * multi-threading, it is not allowed by designing the iterator to be very
 * strict. It is a total mess when you iterate a data structure with
 * Iterator1 and use Iterator2 within Iterator1 to remove the iterated
 * element under Iterator1's nose.
 *
 * @param [in] len the total number of cards in this hand.
 * @param [in] pos the 0-based position of the current card in the hand.
 * @param [in] c the current card under the iterator's view.
 *
 * @return the action that the iterator should do next.
 */
typedef enum itr_action (*card_iterator) (unsigned long len,
					  unsigned long pos,
					  const card *c);

/**
 * Iterates a hand invoking the callback function for each iterated card in the
 * hand.
 *
 * @param [in] h the hand to be iterated.
 * @param [in] itr_fn the callback function that will be invoked in each
 *                    iteration.
 */
void
iterate_hand (card_hand *h, card_iterator itr_fn);

/**
 * Removes all cards having the same suit and rank from a card hand.
 *
 * @param [in] h the hand from which a card is to be removed.
 * @param [in] c the card to be removed.
 */
void
remove_from_hand (card_hand *h, enum card_suit_rank c);

/**
 * Reclaims the memory space that was allocated for the card hand as well as
 * setting the pointer to NULL as a safe guard. Passing a pointer to NULL is
 * safe but not a NULL pointer. This destructor does not free the cards that
 * have ever been inserted. So, if you have ever inserted a card not dealt
 * from the deck, you have to remember to free the card. All cards dealt from
 * a deck will be freed when the deck is destroyed.
 *
 * @param [in] h_ptr the pointer pointing to the card hand to be freed.
 */
void
destroy_hand (card_hand **h_ptr);

/**
 * Checks whether or not a particular card has not been dealt from the deck.
 *
 * @param [in] c the card to be looked for in the deck.
 * @param [in] d the card deck to be checked.
 *
 * @return 0 if the card has been dealt or non-zero if the card has been dealt.
 */
int
is_card_in_deck (enum card_suit_rank c, const card_deck *d);

/**
 * Deals a card from the deck. You do not need to free the dealt card because
 * all dealt cards will be freed upon destructing the deck with destroy_deck().
 * This also means that you cannot maintain a reference to a dealt card after
 * destroy_deck() is called on the deck from which the card is dealt.
 *
 * @param [in] d the deck from which the next card is to be dealt.
 *
 * @return the dealt card or NULL if the deck is empty.
 */
const card *
deal_from_deck (card_deck *d);

/**
 * Removes the specified card from the deck. This is different from
 * deal_from_deck() in a way that this removes an arbitrary card from the deck
 * preventing the card to be dealt while deal_from_deck() always removes the
 * top most card from the deck.
 *
 * @param [in] c the card to be removed.
 * @param [in] d the deck from which the card is to be removed.
 */
void
strip_card_from_deck (enum card_suit_rank c, card_deck *d);

/**
 * Creates a deck of shuffled cards. The returned card deck has to be freed
 * with destroy_deck().
 *
 * @return NULL if the deck cannot be created or a deck of shuffled cards.
 */
card_deck *
create_shuffled_deck (void);

/**
 * Reclaims the memory space that was allocated for the card deck as well as
 * setting the pointer to NULL as a safe guard. Passing a pointer to NULL is
 * safe but not a NULL pointer.
 * <strong>WARNING:</strong> all pointers to a card that has ever been dealt
 * from the deck will be invalid.
 *
 * @param [in] d_ptr the pointer pointing to the card deck to be freed.
 */
void
destroy_deck (card_deck **d_ptr);

#ifdef __cplusplus
}
#endif

#endif /* CARD_H */
