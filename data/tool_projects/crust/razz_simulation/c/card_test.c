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

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include "card.h"

static enum card_suit_rank seed3_dealing_order[] = {
  HEART_9, SPADE_ACE, HEART_10, CLUB_2, DIAMOND_6, HEART_Q, DIAMOND_2, DIAMOND_9,
  HEART_7, CLUB_10, CLUB_4, DIAMOND_5, CLUB_K, SPADE_2, HEART_K, HEART_4,
  DIAMOND_10, CLUB_Q, SPADE_5, SPADE_K, HEART_6, DIAMOND_J, SPADE_9, CLUB_ACE,
  CLUB_5, DIAMOND_8, CLUB_9, HEART_2, SPADE_J, CLUB_7, DIAMOND_4, DIAMOND_K,
  DIAMOND_Q, SPADE_8, SPADE_6, SPADE_7, SPADE_Q, HEART_8, HEART_J, DIAMOND_3,
  DIAMOND_7, SPADE_3, HEART_3, CLUB_J, CLUB_6, HEART_ACE, CLUB_3, DIAMOND_ACE,
  HEART_5, SPADE_4, SPADE_10, CLUB_8,
};

static enum itr_action
test_sort_card_by_rank_1 (unsigned long len, unsigned long pos, const card *c)
{
  static const enum card_suit_rank test_data[] = {
    SPADE_ACE, DIAMOND_2, CLUB_2, DIAMOND_6, HEART_9, HEART_10, HEART_Q,
  };

  assert (get_card_suit_rank (c) == test_data[pos]);

  return CONTINUE;
}

static enum itr_action
test_sort_card_by_rank_2 (unsigned long len, unsigned long pos, const card *c)
{
  static const enum card_suit_rank test_data[] = {
    SPADE_ACE, DIAMOND_2, CLUB_2, HEART_9, HEART_10, HEART_Q,
  };

  assert (get_card_suit_rank (c) == test_data[pos]);

  return CONTINUE;
}

static enum itr_action
test_sort_card_by_rank_3 (unsigned long len, unsigned long pos, const card *c)
{
  static const enum card_suit_rank test_data[] = {
    SPADE_ACE, DIAMOND_2, CLUB_2, HEART_9, HEART_10
  };

  assert (get_card_suit_rank (c) == test_data[pos]);

  return CONTINUE;
}

static enum itr_action
test_sort_card_by_rank_4 (unsigned long len, unsigned long pos, const card *c)
{
  static const enum card_suit_rank test_data[] = {
    HEART_9, HEART_10
  };

  assert (get_card_suit_rank (c) == test_data[pos]);

  return CONTINUE;
}

static enum itr_action
test_empty_hand (unsigned long len, unsigned long pos, const card *c)
{
  assert (0);

  return CONTINUE;
}

int
main (int argc, char **argv, char **envp)
{
  int i;
  int end;
  const card *c;
  card_deck *d;
  card_hand *h;

  /* Enum position */
  assert (SPADE_ACE < SPADE_K);
  assert (HEART_K < CLUB_K);
  assert (ACE < R3);

  /* Card */
  c = create_card (SPADE_ACE);
  assert (c != NULL);
  assert (get_card_suit_rank (c) == SPADE_ACE);
  assert (get_card_rank (c) == ACE);
  assert (get_card_suit (c) == SPADE);
  destroy_card (&c);
  assert (c == NULL);

  assert (c == NULL);
  destroy_card (&c);
  assert (c == NULL);

  c = create_card (CARD_COUNT);
  assert (c == NULL);

  c = create_card (CLUB_8);
  assert (c != NULL);
  assert (get_card_suit_rank (c) == CLUB_8);
  assert (get_card_rank (c) == R8);
  assert (get_card_suit (c) == CLUB);
  destroy_card (&c);
  assert (c == NULL);

  c = strtocard ("S8");
  assert (c != NULL);
  assert (get_card_suit_rank (c) == SPADE_8);
  assert (get_card_rank (c) == R8);
  assert (get_card_suit (c) == SPADE);
  destroy_card (&c);
  assert (c == NULL);

  c = strtocard ("dk");
  assert (c != NULL);
  assert (get_card_suit_rank (c) == DIAMOND_K);
  assert (get_card_rank (c) == K);
  assert (get_card_suit (c) == DIAMOND);
  destroy_card (&c);
  assert (c == NULL);

  c = strtocard ("Ca");
  assert (c != NULL);
  assert (get_card_suit_rank (c) == CLUB_ACE);
  assert (get_card_rank (c) == ACE);
  assert (get_card_suit (c) == CLUB);
  destroy_card (&c);
  assert (c == NULL);

  c = strtocard ("hJ");
  assert (c != NULL);
  assert (get_card_suit_rank (c) == HEART_J);
  assert (get_card_rank (c) == J);
  assert (get_card_suit (c) == HEART);
  destroy_card (&c);
  assert (c == NULL);

  c = strtocard ("SQ");
  assert (c != NULL);
  assert (get_card_suit_rank (c) == SPADE_Q);
  assert (get_card_rank (c) == Q);
  assert (get_card_suit (c) == SPADE);
  destroy_card (&c);
  assert (c == NULL);

  c = strtocard ("SS");
  assert (c == NULL);
  c = strtocard ("S0");
  assert (c == NULL);
  c = strtocard ("S1");
  assert (c == NULL);
  c = strtocard ("a2");
  assert (c == NULL);

  /* Rank */
  assert (strtorank ("ace") == ACE);
  assert (strtorank ("8") == R8);
  assert (strtorank ("K") == K);
  assert (strtorank ("10") == R10);
  assert (strtorank ("1") == INVALID_RANK);

  /* Card to string */
  assert (strcmp (cardtostr (SPADE_8), "S8") == 0);
  assert (strcmp (cardtostr (CLUB_10), "C10") == 0);
  assert (strcmp (cardtostr (SPADE_ACE), "SA") == 0);
  assert (strcmp (cardtostr (CLUB_K), "CK") == 0);
  assert (cardtostr (CARD_COUNT) == NULL);

  /* Rank to string */
  assert (strcmp (ranktostr (R8), "8") == 0);
  assert (strcmp (ranktostr (R10), "10") == 0);
  assert (strcmp (ranktostr (ACE), "A") == 0);
  assert (strcmp (ranktostr (K), "K") == 0);
  assert (ranktostr (INVALID_RANK) == NULL);

  /* Deck */
  srand48 (3);
  d = create_shuffled_deck ();
  assert (d != NULL);
  assert (is_card_in_deck (HEART_K, d));
  strip_card_from_deck (HEART_K, d);
  assert (!is_card_in_deck (HEART_K, d));
  strip_card_from_deck (HEART_9, d);
  assert (!is_card_in_deck (HEART_9, d));
  for (i = 1; i <= 50; i++)
    {
      c = deal_from_deck (d);
      assert (c != NULL);
      assert (!is_card_in_deck (get_card_suit_rank (c), d));
    }
  c = deal_from_deck (d);
  assert (c == NULL);
  destroy_deck (&d);
  assert (d == NULL);

  /* Hand */
  srand48 (3);
  d = create_shuffled_deck ();
  assert (d != NULL);
  h = create_hand (7, sort_card_by_rank);
  assert (h != NULL);
  assert (count_cards_in_hand (h) == 0);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == INVALID_RANK);

  insert_into_hand (h, deal_from_deck (d)); /* 1 */
  assert (count_cards_in_hand (h) == 1);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == R9);
  insert_into_hand (h, deal_from_deck (d)); /* 2 */
  assert (count_cards_in_hand (h) == 2);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == R9);
  insert_into_hand (h, deal_from_deck (d)); /* 3 */
  assert (count_cards_in_hand (h) == 3);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == R10);
  insert_into_hand (h, deal_from_deck (d)); /* 4 */
  assert (count_cards_in_hand (h) == 4);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == R10);
  insert_into_hand (h, deal_from_deck (d)); /* 5 */
  assert (count_cards_in_hand (h) == 5);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == R10);
  insert_into_hand (h, deal_from_deck (d)); /* 6 */
  assert (count_cards_in_hand (h) == 6);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == Q);
  insert_into_hand (h, deal_from_deck (d)); /* 7 */
  assert (count_cards_in_hand (h) == 7);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == Q);
  insert_into_hand (h, deal_from_deck (d)); /* 8 */
  assert (count_cards_in_hand (h) == 7);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == Q);
  iterate_hand (h, test_sort_card_by_rank_1);

  remove_from_hand (h, DIAMOND_6);
  assert (count_cards_in_hand (h) == 6);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == Q);
  iterate_hand (h, test_sort_card_by_rank_2);

  remove_from_hand (h, HEART_Q);
  assert (count_cards_in_hand (h) == 5);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == R10);
  iterate_hand (h, test_sort_card_by_rank_3);

  reset_hand (h);
  assert (count_cards_in_hand (h) == 0);
  assert (get_max_of_hand (h) == 7);
  assert (get_max_rank_of_hand (h) == INVALID_RANK);
  remove_from_hand (h, HEART_Q);
  iterate_hand (h, test_empty_hand);
  
  destroy_hand (&h);
  assert (h == NULL);
  destroy_deck (&d);
  assert (d == NULL);

  /* Hand with 1 card */
  srand48 (3);
  d = create_shuffled_deck ();
  assert (d != NULL);
  h = create_hand (1, sort_card_by_rank);
  assert (h != NULL);

  insert_into_hand (h, deal_from_deck (d));
  assert (count_cards_in_hand (h) == 1);
  assert (get_max_of_hand (h) == 1);
  assert (get_max_rank_of_hand (h) == R9);
  remove_from_hand (h, HEART_Q);
  remove_from_hand (h, HEART_Q);

  destroy_hand (&h);
  assert (h == NULL);
  destroy_deck (&d);
  assert (d == NULL);

  /* Head removal */
  srand48 (3);
  d = create_shuffled_deck ();
  assert (d != NULL);
  h = create_hand (3, sort_card_by_rank);
  assert (h != NULL);

  insert_into_hand (h, deal_from_deck (d));
  assert (get_max_rank_of_hand (h) == R9);
  insert_into_hand (h, deal_from_deck (d));
  assert (get_max_rank_of_hand (h) == R9);
  insert_into_hand (h, deal_from_deck (d));
  assert (get_max_rank_of_hand (h) == R10);
  remove_from_hand (h, SPADE_ACE);
  assert (get_max_rank_of_hand (h) == R10);
  iterate_hand (h, test_sort_card_by_rank_4);
  assert (count_cards_in_hand (h) == 2);

  destroy_hand (&h);
  assert (h == NULL);
  destroy_deck (&d);
  assert (d == NULL);

  /* Uniform distribution */
  unsigned long card_count[CARD_COUNT] = {0};
  unsigned long expected_card_count[] = {
    1001, 1012, 1032, 911, 1000, 987, 1026, 978, 971, 964, 942, 968, 1033,
    962, 1011, 939, 975, 1052, 1028, 1002, 992, 1054, 1004, 1045, 993, 984,
    1023, 960, 993, 1058, 998, 971, 1018, 1025, 1042, 1045, 983, 1018, 999,
    995, 1024, 999, 968, 1024, 986, 1015, 1036, 1015, 966, 973, 982, 1018,
  };
  srand48 (3);
  end = CARD_COUNT * 1000;
  for (i = 0; i < end; i++)
    {
      d = create_shuffled_deck ();
      assert (d != NULL);
      c = deal_from_deck (d);
      assert (c != NULL);
      card_count[get_card_suit_rank (c)]++;
      destroy_deck (&d);
      assert (d == NULL); 
    }
  for (i = 0; i < CARD_COUNT; i++)
    {
      assert (card_count[i] == expected_card_count[i]);
    }

  exit (EXIT_SUCCESS);
}
