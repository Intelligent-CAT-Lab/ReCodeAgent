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

#include <stdint.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include "card.h"

/** A card having a particular suit and rank. */
struct card_impl
{
  uint8_t card; /**<
		 * The first 3 bits are for suit while the last 5 bits are for
		 * rank.
		 */
/* The value of the following defines will never change. */
#define INVALID_CARD_BITS (0U)

#define SUIT_BITS (0x7U << 5)
#define SPADE_BITS (1U << 5)
#define HEART_BITS (2U << 5)
#define DIAMOND_BITS (3U << 5)
#define CLUB_BITS (4U << 5)

#define RANK_BITS (0x1FU)
#define ACE_BITS (1U)
#define R2_BITS (2U)
#define R3_BITS (3U)
#define R4_BITS (4U)
#define R5_BITS (5U)
#define R6_BITS (6U)
#define R7_BITS (7U)
#define R8_BITS (8U)
#define R9_BITS (9U)
#define R10_BITS (10U)
#define J_BITS (11U)
#define Q_BITS (12U)
#define K_BITS (13U)
};

/**
 * Makes a card have the desired suit and rank.
 *
 * @param [in] csr the desired rank and suit.
 * @param [out] c a pointer to the memory location to write the card on.
 */
static void
write_card (enum card_suit_rank csr, card *c)
{
  c->card = INVALID_CARD_BITS;

  if (csr >= SPADE_ACE && csr <= SPADE_K)
    {
      c->card |= SPADE_BITS;
    }
  else if (csr >= HEART_ACE && csr <= HEART_K)
    {
      c->card |= HEART_BITS;
    }
  else if (csr >= DIAMOND_ACE && csr <= DIAMOND_K)
    {
      c->card |= DIAMOND_BITS;
    }
  else if (csr >= CLUB_ACE && csr <= CLUB_K)
    {
      c->card |= CLUB_BITS;
    }

  switch (csr)
    {
    case SPADE_ACE:
    case HEART_ACE:
    case DIAMOND_ACE:
    case CLUB_ACE:
      c->card |= ACE_BITS;
      break;
    case SPADE_2:
    case HEART_2:
    case DIAMOND_2:
    case CLUB_2:
      c->card |= R2_BITS;
      break;
    case SPADE_3:
    case HEART_3:
    case DIAMOND_3:
    case CLUB_3:
      c->card |= R3_BITS;
      break;
    case SPADE_4:
    case HEART_4:
    case DIAMOND_4:
    case CLUB_4:
      c->card |= R4_BITS;
      break;
    case SPADE_5:
    case HEART_5:
    case DIAMOND_5:
    case CLUB_5:
      c->card |= R5_BITS;
      break;
    case SPADE_6:
    case HEART_6:
    case DIAMOND_6:
    case CLUB_6:
      c->card |= R6_BITS;
      break;
    case SPADE_7:
    case HEART_7:
    case DIAMOND_7:
    case CLUB_7:
      c->card |= R7_BITS;
      break;
    case SPADE_8:
    case HEART_8:
    case DIAMOND_8:
    case CLUB_8:
      c->card |= R8_BITS;
      break;
    case SPADE_9:
    case HEART_9:
    case DIAMOND_9:
    case CLUB_9:
      c->card |= R9_BITS;
      break;
    case SPADE_10:
    case HEART_10:
    case DIAMOND_10:
    case CLUB_10:
      c->card |= R10_BITS;
      break;
    case SPADE_J:
    case HEART_J:
    case DIAMOND_J:
    case CLUB_J:
      c->card |= J_BITS;
      break;
    case SPADE_Q:
    case HEART_Q:
    case DIAMOND_Q:
    case CLUB_Q:
      c->card |= Q_BITS;
      break;
    case SPADE_K:
    case HEART_K:
    case DIAMOND_K:
    case CLUB_K:
      c->card |= K_BITS;
      break;
    }
}

struct card_collection;

/** A collection of cards. */
struct card_collection
{
  struct card_collection *prev; /**< The previous card in the collection. */
  struct card_collection *next; /**< The next card in the collection. */
  const card *c; /**< A card in a collection. */
};

/**
 * Inserts a card having the specified suit and rank into a collection.
 *
 * @param [in] head a pointer to the first element of the collection (this
 *                  can be NULL if the collection is still empty).
 * @param [in] c the card to be inserted into the collection.
 * @param [in] sorter a callback function that determines the placement of the
 *                    card in the collection.
 *
 * @return 0 if the addition is successful or non-zero if a new element cannot
 *         be created.
 */
static int
insert_into_collection (struct card_collection **head, const card *c,
			card_sorter sorter)
{
  struct card_collection *col;
  struct card_collection *itr;

  col = malloc (sizeof (*col));
  if (col == NULL)
    {
      destroy_card (&c);
      return 1;
    }

  col->c = c;

  if (*head == NULL)
    {
      col->prev = col;
      col->next = col;
      *head = col;

      return 0;
    }

  itr = *head;
  if (sorter (NULL, c, itr->c))
    {
      col->prev = itr->prev;
      col->next = itr;

      itr->prev = col;
      col->prev->next = col;

      *head = col;

      return 0;
    }
  while (itr->next != *head)
    {
      if (sorter (itr->c, c, itr->next->c))
	{
	  col->prev = itr;
	  col->next = itr->next;

	  itr->next = col;
	  col->next->prev = col;

	  return 0;
	}
      itr = itr->next;
    }
  if (sorter (itr->c, c, NULL))
    {
      col->prev = itr;
      col->next = *head;

      itr->next = col;
      col->next->prev = col;

      return 0;
    }

  return 0;
}

/**
 * Iterates a collection through the next pointer.
 *
 * @param [in] head the pointer to the first element in the collection. NULL
 *                  means an empty collection.
 * @param [in,out] itr the pointer by which the caller can operate on the
 *                     element under the iterator. The iteration state is
 *                     saved in this pointer. The pointer should first point to
 *                     NULL. You can point itr to the address of an element in
 *                     the collection to start the
 *                     iteration with the element <strong>after</strong> the
 *                     pointed element onward.
 *
 * @return zero if all elements has been iterated or non-zero if there are still
 *         some elements to be iterated.
 */
static int
iterate_collection (struct card_collection *head, struct card_collection **itr)
{
  if (head == NULL)
    {
      return 0;
    }

  if (*itr == NULL)
    {
      *itr = head;
    }
  else
    {
      *itr = (*itr)->next;
      if (*itr == head)
	{
	  *itr = NULL;
	}
    }

  return *itr != NULL;
}

/**
 * Appends a new entry into the end of the collection.
 *
 * @param [in] head a pointer to the head of the collection. It can point to
 *                  NULL if the collection is empty.
 * @param [in] new the new entry to be appended.
 */
static void
append_into_collection (struct card_collection **head, struct card_collection *new)
{
  if (*head == NULL)
    {
      new->prev = new;
      new->next = new;      
      *head = new;

      return;
    }

  new->prev = (*head)->prev;
  new->next = *head;
  new->prev->next = new;
  new->next->prev = new;
}

/**
 * Detach an entry from the collection.
 * This function is safe to be used during iteration because the detached entry
 * is not freed and its previous and next pointers are still valid although the
 * entry is no longer recognized as a part of the collection (i.e., subsequent
 * iteration on the collection will not iterate the detached entry).
 * It is your responsibility to free the detached entry as well as its payload.
 *
 * @param [in] entry the entry to be detached.
 */
static void
detach_from_collection (const struct card_collection *entry)
{
  entry->next->prev = entry->prev;
  entry->prev->next = entry->next;
}

/**
 * The callback function used to free the payload of an entry in the collection.
 *
 * @param [in] payload the pointer to the payload to be freed.
 */
typedef void (*payload_freer) (void **payload);

static void
destroy_card_payload (void **payload)
{
  destroy_card ((const card **) payload);
}

/**
 * Reclaims the memory space that was allocated for the entry as well as
 * setting the pointer to NULL as a safe guard. Passing a pointer to NULL is,
 * safe but not a NULL pointer. A function callback to free the payload should
 * be provided unless the payload should not be freed.
 *
 * @param [in] entry the pointer pointing to the entry to be freed.
 * @param [in] freer the callback function used to free the payload of an entry
 *                   or NULL if the payload should not be freed.
 */
static void
destroy_collection_entry (struct card_collection **entry, payload_freer freer)
{
  if (*entry == NULL)
    {
      return;
    }

  if (freer != NULL)
    {
      freer ((void *) &(*entry)->c);
    }
  free ((void *) *entry);

  *entry = NULL;
}

/**
 * Reclaims the memory space that was allocated for the entry under an iteration.
 * Entry and head must point to valid entries in a collection. Upon completion,
 * invocation of iterate_collection() on the entry will continue with the next
 * entry. If head gets removed, head will be adjusted to point to the next entry
 * in the chain. A function callback to free the payload should be provided
 * unless the payload should not be freed.
 *
 * @param [in] head the pointer pointing to the head of the collection.
 * @param [in] entry the pointer pointing to the entry to be freed.
 * @param [in] freer the callback function used to free the payload of the entry
 *                   or NULL if the payload should not be freed.
 */
static void
remove_from_collection_under_itr (struct card_collection **head,
				  struct card_collection **entry,
				  payload_freer freer)
{
  struct card_collection *entry_to_remove = NULL;
  int is_entry_head = (*head == *entry);

  detach_from_collection (*entry);
  entry_to_remove = *entry;
  *entry = (*entry)->prev;
  if (*entry == entry_to_remove)
    {
      *entry = NULL;
    }
  destroy_collection_entry (&entry_to_remove, freer);

  if (is_entry_head && *head != *entry) // the head gets deleted;
    {
      *head = ((*entry == NULL) ? NULL : (*entry)->next);
    }
}

/**
 * Reclaims the memory space that was allocated for the collection as well as
 * setting the pointer to NULL as a safe guard. Passing a pointer to NULL is,
 * safe but not a NULL pointer. A function callback to free the payload should
 * be provided unless the payload should not be freed.
 *
 * @param [in] col_ptr the pointer pointing to the collection to be freed.
 * @param [in] freer the callback function used to free the payload of an entry
 *                   or NULL if the payload should not be freed.
 */
static void
destroy_collection (struct card_collection **col_ptr, payload_freer freer)
{
  struct card_collection *head = *col_ptr;
  struct card_collection *head_next;

  if (head == NULL)
    {
      return;
    }

  head->prev->next = NULL; /* break the ring */

  do
    {
      head_next = head->next;

      if (freer != NULL)
	{
	  freer ((void *) &head->c);
	}
      free (head);

      head = head_next;
    }
  while (head != NULL);

  *col_ptr = NULL;
}

/** A hand of cards. */
struct card_hand_impl
{
  uint8_t max; /**< The maximum number of cards at hand. */
  uint8_t len; /**< The current number of cards at hand. */
  card_sorter sorter; /**<
		       * The callback function used to determine the place of
		       * insertion of a new card.
		       */
  struct card_collection *cards; /**< The cards at hand. */
};

/** A deck of cards. */
struct card_deck_impl
{
  char card_count; /**< The number of cards in the deck. */
  card cards[CARD_COUNT]; /**< The cards in the deck. */
};

enum card_suit_rank
get_card_suit_rank (const card *c)
{
  enum card_suit_rank csr;
  enum card_suit cs = get_card_suit (c);
  enum card_rank cr = get_card_rank (c);

  if (cs == INVALID_SUIT || cr == INVALID_RANK)
    {
      return INVALID_CARD;
    }

  switch (cs)
    {
    case SPADE:
      csr = SPADE_ACE;
      break;
    case HEART:
      csr = HEART_ACE;
      break;
    case DIAMOND:
      csr = DIAMOND_ACE;
      break;
    case CLUB:
      csr = CLUB_ACE;
      break;
    }

  return csr + cr;
}

enum card_rank
get_card_rank (const card *c)
{
  uint8_t r = (c->card & RANK_BITS);

  if (r < ACE_BITS || r > K_BITS)
    {
      return INVALID_RANK;
    }

  return r - 1;
}

enum card_suit
get_card_suit (const card *c)
{
  uint8_t s = c->card & SUIT_BITS;

  if (s < SPADE_BITS || s > CLUB_BITS)
    {
      return INVALID_SUIT;
    }

  return (s >> 5) - 1;
}

const card *
create_card (enum card_suit_rank csr)
{
  card *c;

  c = malloc (sizeof (*c));

  if (c == NULL)
    {
      return NULL;
    }

  write_card (csr, c);

  if (c->card == INVALID_CARD_BITS)
    {
      free (c);
      return NULL;
    }

  return c;
}

const card *
strtocard (const char *str)
{
  enum card_suit_rank csr;
  size_t char_count = strlen (str);

  if (char_count != 2)
    {
      return NULL;
    }

  switch (toupper (str[0]))
    {
    case 'S':
      csr = SPADE_ACE;
      break;
    case 'H':
      csr = HEART_ACE;
      break;
    case 'D':
      csr = DIAMOND_ACE;
      break;
    case 'C':
      csr = CLUB_ACE;
      break;
    default:
      return NULL;
    }

  if (str[1] >= '2' && str[1] <= '9')
    {
      return create_card (csr + str[1] - '1');
    }

  switch (toupper (str[1]))
    {
    case 'A':
      return create_card (csr);
    case '1':
      if (char_count == 3)
	{
	  return create_card (csr + 9);
	}
      else
	{
	  return NULL;
	}
    case 'J':
      return create_card (csr + 10);
    case 'Q':
      return create_card (csr + 11);
    case 'K':
      return create_card (csr + 12);
    }

  return NULL;
}

const char *
cardtostr (enum card_suit_rank c)
{
  static const char *s[SUIT_COUNT][RANK_COUNT] = {
    {"SA", "S2", "S3", "S4", "S5", "S6", "S7", "S8", "S9", "S10", "SJ", "SQ", "SK"},
    {"HA", "H2", "H3", "H4", "H5", "H6", "H7", "H8", "H9", "H10", "HJ", "HQ", "HK"},
    {"DA", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "D10", "DJ", "DQ", "DK"},
    {"CA", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "C10", "CJ", "CQ", "CK"},
  };

  if (c < SPADE_ACE || c > CLUB_K)
    {
      return NULL;
    }

  return s[c / RANK_COUNT][c % RANK_COUNT];
}

const char *
ranktostr (enum card_rank r)
{
  static const char *s[RANK_COUNT] = {
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"
  };

  if (r < ACE || r > K)
    {
      return NULL;
    }

  return s[r];
}

void
destroy_card (const card **c_ptr)
{
  if (*c_ptr == NULL)
    {
      return;
    }

  free ((void *) *c_ptr);
  *c_ptr = NULL;
}

enum card_rank
strtorank (const char *str)
{
  enum card_rank cr = ACE;

  if (str[0] >= '2' && str[0] <= '9')
    {
      cr += str[0] - '1';
    }
  else
    {
      switch (toupper (str[0]))
	{
	case 'A':
	  break;
	case '1':
	  if (str[1] == '0')
	    {
	      cr += 9;
	    }
	  else
	    {
	      return INVALID_RANK;
	    }
	  break;
	case 'J':
	  cr += 10;
	  break;
	case 'Q':
	  cr += 11;
	  break;
	case 'K':
	  cr += 12;
	  break;
	default:
	  return INVALID_RANK;
	}
    }

  return cr;
}

int
sort_card_after (const card *before, const card *new, const card *after)
{
  if (after == NULL)
    {
      return 1;
    }

  return 0;
}

int
sort_card_by_rank (const card *before, const card *new, const card *after)
{
  enum card_rank r = get_card_rank (new);

  if (after == NULL
      || ((before == NULL || r > get_card_rank (before))
	  && r <= get_card_rank (after)))
    {
      return 1;
    }

  return 0;
}

card_hand *
create_hand (unsigned char max, card_sorter sorter)
{
  struct card_hand_impl *h = malloc (sizeof (*h));

  if (h == NULL)
    {
      return NULL;
    }

  if (sorter == NULL)
    {
      sorter = sort_card_after;
    }

  h->max = max;
  h->len = 0;
  h->sorter = sorter;
  h->cards = NULL;

  return h;
}

void
reset_hand (card_hand *h)
{
  h->len = 0;

  destroy_collection (&h->cards, NULL);
}

void
insert_into_hand (card_hand *h, const card *c)
{
  if (h->max == h->len)
    {
      return;
    }

  insert_into_collection (&h->cards, c, h->sorter);
  h->len++;
}

unsigned long
count_cards_in_hand (const card_hand *h)
{
  return h->len;
}

unsigned long
get_max_of_hand (const card_hand *h)
{
  return h->max;
}

enum card_rank
get_max_rank_of_hand (const card_hand *h)
{
  struct card_collection *itr = NULL;
  enum card_rank cr = INVALID_RANK;

  if (h->len == 0)
    {
      return cr;
    }

  while (iterate_collection (h->cards, &itr))
    {
      enum card_rank this_cr = get_card_rank (itr->c);

      if (cr == INVALID_RANK)
	{
	  cr = this_cr;
	}
      else if (this_cr > cr)
	{
	  cr = this_cr;
	}
    }

  return cr;
}

/**
 * Removes an entry in a hand under an iteration.
 *
 * @param [in] h the hand from which the entry is to be removed.
 * @param [in] itr the iterator that is iterating the hand.
 * @param [in] pos the iteration position to be adjusted
 *                 (set to NULL if there is need for one)
 */
static void
remove_from_hand_under_itr (card_hand *h, struct card_collection **itr,
			    unsigned long *pos)
{
  remove_from_collection_under_itr (&h->cards, itr, NULL);

  h->len--;
  if (pos != NULL)
    {
      *pos -= 1;
    }
}

void
iterate_hand (card_hand *h, card_iterator itr_fn)
{
  struct card_collection *itr = NULL;
  unsigned long pos = 0;
  int is_stopped = 0;

  while (!is_stopped && iterate_collection (h->cards, &itr))
    {
      switch (itr_fn (h->len, pos, itr->c))
	{
	case CONTINUE:
	  break;
	case BREAK:
	  is_stopped = 1;
	  break;
	case REMOVE_AND_CONTINUE:
	  remove_from_hand_under_itr (h, &itr, &pos);
	  break;
	case REMOVE_AND_BREAK:
	  remove_from_hand_under_itr (h, &itr, &pos);
	  is_stopped = 1;
	  break;
	}
      pos++;
    }
}

void
remove_from_hand (card_hand *h, enum card_suit_rank c)
{
  struct card_collection *itr = NULL;

  while (iterate_collection (h->cards, &itr))
    {
      if (get_card_suit_rank (itr->c) == c)
	{
	  remove_from_hand_under_itr (h, &itr, NULL);
	}
    }
}

void
destroy_hand (card_hand **h_ptr)
{
  if (*h_ptr == NULL)
    {
      return;
    }

  destroy_collection (&(*h_ptr)->cards, NULL);
  free ((void *) *h_ptr);

  *h_ptr = NULL;
}

int
is_card_in_deck (enum card_suit_rank c, const card_deck *d)
{
  return get_card_suit_rank (&d->cards[c]) == INVALID_CARD;
}

const card *
deal_from_deck (card_deck *d)
{
  unsigned long i;
  unsigned long valid_card_idx;
  unsigned long selected_card_idx;
  card *c;

  if (d->card_count <= 0)
    {
      return NULL;
    }

  selected_card_idx = lrand48 () % d->card_count;

  valid_card_idx = 0;
  for (i = 0; i < CARD_COUNT; i++)
    {
      c = &d->cards[i];
      if (get_card_suit_rank (c) == INVALID_CARD)
	{
	  if (valid_card_idx == selected_card_idx)
	    {
	      write_card (i, c);
	      d->card_count--;
	      break;
	    }

	  valid_card_idx++;
	}
    }

  return c;
}

void
strip_card_from_deck (enum card_suit_rank c, card_deck *d)
{
  card *card = &d->cards[c];

  if (get_card_suit_rank (card) == INVALID_CARD)
    {
      write_card (c, card);
      d->card_count--;
    }
}

card_deck *
create_shuffled_deck (void)
{
  struct card_deck_impl *deck;

  deck = malloc (sizeof (*deck));
  if (deck == NULL)
    {
      return NULL;
    }

  memset (deck, 0, sizeof (*deck));

  deck->card_count = CARD_COUNT;

  return deck;
}

void
destroy_deck (card_deck **d_ptr)
{
  if (*d_ptr == NULL)
    {
      return;
    }

  free (*d_ptr);
  *d_ptr = NULL;
}
