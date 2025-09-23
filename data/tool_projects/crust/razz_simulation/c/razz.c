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

#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include "card.h"
#include "razz_simulation.h"

int
process_args (unsigned long *game_count,
	      struct decided_cards *decided_cards,
	      int argc,
	      char **argv)
{
  int i, end;
  enum card_suit_rank csr;
  card_deck *deck;

  if (argc < 4 || argc > 11)
    {
      fprintf (stderr, "Invalid argument count\n");
      return 1;
    }

  *game_count = atoi (*argv++);

  deck = create_shuffled_deck ();
  if (deck == NULL)
    {
      fprintf (stderr, "Cannot create a shuffled deck\n");
      return 1;
    }
  end = 3;
  decided_cards->my_card_count = end;
  for (i = 0; i < end; i++)
    {
      enum card_rank my_rank;
      enum card_suit cs = SPADE;
      const card *my_card;

      if ((my_rank = strtorank (*argv++)) == INVALID_RANK)
	{
	  fprintf (stderr, "Invalid my rank specification #%d\n", i + 1);
	  destroy_deck (&deck);  
	  return 1;
	}

      csr = cs * RANK_COUNT + my_rank;
      while (!is_card_in_deck (csr, deck))
	{
	  if (++cs == SUIT_COUNT)
	    {
	      break;
	    }

	  csr = cs * RANK_COUNT + my_rank;
	}

      if (cs == SUIT_COUNT)
	{
	  fprintf (stderr, "Duplicated my rank specification #%d\n", i + 1);
	  destroy_deck (&deck);  
	  return 1;
	}

      strip_card_from_deck (csr, deck);

      if ((my_card = create_card (csr)) == NULL)
	{
	  fprintf (stderr, "Cannot create my card #%d\n", i + 1);
	  destroy_deck (&deck);  
	  return 1;
	}
      decided_cards->my_cards[i] = my_card;
    }

  i = 0;
  while (*argv != NULL)
    {
      enum card_rank opponent_rank;
      enum card_suit cs = SPADE;
      const card *opponent_card;

      if ((opponent_rank = strtorank (*argv++)) == INVALID_RANK)
	{
	  fprintf (stderr, "Invalid opponent rank specification #%d\n", i + 1);
	  destroy_deck (&deck);  
	  return 1;
	}

      csr = cs * RANK_COUNT + opponent_rank;
      while (!is_card_in_deck (csr, deck))
	{
	  if (++cs == SUIT_COUNT)
	    {
	      break;
	    }

	  csr = cs * RANK_COUNT + opponent_rank;
	}

      if (cs == SUIT_COUNT)
	{
	  fprintf (stderr, "Duplicated opponent rank specification #%d\n", i + 1);
	  destroy_deck (&deck);  
	  return 1;
	}

      strip_card_from_deck (csr, deck);

      if ((opponent_card = create_card (csr)) == NULL)
	{
	  fprintf (stderr, "Cannot create opponent card #%d\n", i + 1);
	  destroy_deck (&deck);  
	  return 1;
	}
      decided_cards->opponent_cards[i] = opponent_card;

      i++;
    }
  decided_cards->opponent_card_count = i;

  destroy_deck (&deck);
  return 0;
}

void
listener (void *arg, enum card_rank r)
{
  unsigned long *rank_count = arg;

  if (r != INVALID_RANK && (r < R5 || r > K))
    {
      fprintf (stderr, "%s is out of range\n", ranktostr (r));
      return;
    }

  if (r != INVALID_RANK)
    {
      rank_count[r - R5]++;
    }
}

int
main (int argc, char **argv, char **envp)
{
  int i;
  int end;
  struct decided_cards decided_cards;
  unsigned long game_count;
  unsigned long rank_count[K - R5 + 1] = {0};

  srand48 (time (NULL));

  if (argc < 5 || argc > 12)
    {
      fprintf (stderr,
	       "Usage: razz GAME_COUNT\n"
	       "\tRANK1 RANK2 RANK3\n"
	       "\t[OPP1_RANK [OPP2_RANK [... [OPP7_RANK]]]]\n"
	       "\n"
	       "You specify a rank with the following symbols:\n"
	       "\tA, 2, ..., 10, J, Q, K for ace to king\n");
      exit (EXIT_FAILURE);
    }

  if (process_args (&game_count, &decided_cards,
		    argc - 1, &argv[1]))
    {
      exit (EXIT_FAILURE);
    }

  if (simulate_razz_game (&decided_cards, game_count, rank_count, listener))
    {
      exit (EXIT_FAILURE);
    }

  for (i = 0; i < decided_cards.my_card_count; i++)
    {
      destroy_card (&decided_cards.my_cards[i]);
    }
  for (i = 0; i < decided_cards.opponent_card_count; i++)
    {
      destroy_card (&decided_cards.opponent_cards[i]);
    }

  end = K - R5 + 1;
  for (i = 0; i < end; i++)
    {
      printf ("%2s = %.4f\n",
	      ranktostr (R5 + i),
	      (double) rank_count[i] / game_count);
    }

  exit (EXIT_SUCCESS);
}
