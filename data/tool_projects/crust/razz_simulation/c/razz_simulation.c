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
 *****************************************************************************/

#include <stdio.h>
#include <stdlib.h>
#include "card.h"
#include "razz_simulation.h"

/** The number of cards each person is dealt in one round of Razz game. */
#define RAZZ_CARD_IN_HAND_COUNT 7

/**
 * Complete my hand with the predetermined cards and cards dealt from the deck.
 * The predetermined cards must not contain any duplicate.
 *
 * @param [in] my_hand the hand to be completed.
 * @param [in] decided_cards the predetermined cards for my hand.
 * @param [in] deck the deck from which additional cards are dealt.
 */
static void
complete_hand (card_hand *my_hand, const struct decided_cards *decided_cards,
	       card_deck *deck)
{
  int i;
  int end = decided_cards->my_card_count;

  for (i = 0; i < end; i++)
    {
      insert_into_hand (my_hand, decided_cards->my_cards[i]);
    }

  end = RAZZ_CARD_IN_HAND_COUNT - end;
  for (i = 0; i < end; i++)
    {
      insert_into_hand (my_hand, deal_from_deck (deck));
    }
}

/** Removes a duplicated rank from a hand. */
static enum itr_action
duplicated_rank_remover (unsigned long len, unsigned long pos, const card *c)
{
  static enum card_rank prev_rank;
  enum card_rank curr_rank = get_card_rank (c);

  if (pos == 0)
    {
      prev_rank = curr_rank;
      return CONTINUE;
    }

  if (prev_rank == curr_rank)
    {
      return REMOVE_AND_CONTINUE;
    }

  prev_rank = curr_rank;

  return CONTINUE;
}

/** Keeps only the first five cards in a hand. */
static enum itr_action
length_trimmer (unsigned long len, unsigned long pos, const card *c)
{
  if (pos >= 5)
    {
      return REMOVE_AND_CONTINUE;
    }

  return CONTINUE;
}

/** Prints all cards in the hand. */
static enum itr_action
card_printer (unsigned long len, unsigned long pos, const card *c)
{
  printf ("%4s", cardtostr (get_card_suit_rank (c)));
  return CONTINUE;
}

/**
 * Determines the Razz rank of a hand.
 *
 * @param [in] hand the hand whose rank is to be determined.
 *
 * @return the Razz rank of the hand between R5 and K or INVALID_RANK if the
 *         rank is worse than K.
 */
static enum card_rank
get_razz_rank (card_hand *hand)
{
  enum card_rank r;
  unsigned long cards_count;

#ifndef NDEBUG
  iterate_hand (hand, card_printer);
#endif
  iterate_hand (hand, duplicated_rank_remover);
#ifndef NDEBUG
  printf (" -> ");
  iterate_hand (hand, card_printer);
#endif

  cards_count = count_cards_in_hand (hand);
  if (cards_count < 5) // too many pairs in hand
    {
#ifndef NDEBUG
      printf ("\n");
#endif
      return INVALID_RANK;
    }

  iterate_hand (hand, length_trimmer);
  r = get_max_rank_of_hand (hand);

#ifndef NDEBUG
  if (cards_count > 5)
    {
      printf ("\t");
    }
  else
    {
      printf ("\t\t");
    }
  printf ("-> ");
  iterate_hand (hand, card_printer);
  printf (": %2s\n",
	  ranktostr (r));
#endif

  return r;
}

/**
 * Strips the deck from decided cards. The given decided cards must not contain
 * any duplicate.
 *
 * @param [in] deck the deck to be stripped out.
 * @param [in] decided_cards the non-duplicated cards to be stripped out.
 */
static void
strip_deck (card_deck *deck, const struct decided_cards *decided_cards)
{
  int i, end;

  end = decided_cards->my_card_count;
  for (i = 0; i < end; i++)
    {
      strip_card_from_deck (get_card_suit_rank (decided_cards->my_cards[i]),
			    deck);
    }

  end = decided_cards->opponent_card_count;
  for (i = 0; i < end; i++)
    {
      strip_card_from_deck (get_card_suit_rank (decided_cards->opponent_cards[i]),
			    deck);
    }
}

int
simulate_razz_game (const struct decided_cards *decided_cards,
		    unsigned long game_count,
		    void *arg,
		    rank_listener listener)
{
  unsigned long i;
  card_hand *my_hand;
  card_deck *deck;

  my_hand = create_hand (RAZZ_CARD_IN_HAND_COUNT, sort_card_by_rank);
  if (my_hand == NULL)
    {
      fprintf (stderr, "Cannot create a hand\n");
      return 1;
    }

  for (i = 0; i < game_count; i++)
    {
      deck = create_shuffled_deck ();
      if (deck == NULL)
	{
	  fprintf (stderr, "Cannot create a shuffled deck\n");
	  destroy_deck (&deck);
	  return 1;
	}
      strip_deck (deck, decided_cards);

      complete_hand (my_hand, decided_cards, deck);
      listener (arg, get_razz_rank (my_hand));

      reset_hand (my_hand);
      destroy_deck (&deck);
    }

  destroy_hand (&my_hand);
  destroy_deck (&deck);

  return 0;
}
