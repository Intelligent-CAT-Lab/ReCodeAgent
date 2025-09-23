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
 * @file razz_simulation.h
 * @brief The Razz simulation engine.
 * @author Tadeus Prastowo <eus@member.fsf.org>
 ****************************************************************************/

#include <stdint.h>
#include "card.h"

#ifndef RAZZ_SIMULATION_H
#define RAZZ_SIMULATION_H

#ifdef __cplusplus
extern "C" {
#endif

/** The cards that are not played in the simulated game. */
struct decided_cards
{
  uint8_t my_card_count; /**< The number of my cards. */
  const card *my_cards[3]; /**< My initial three cards. */
  uint8_t opponent_card_count; /**< The total number of opponents' cards. */
  const card *opponent_cards[7]; /**< The initial card of the opponent. */
};

/**
 * Listens to the final rank of my hand at the end of each game.
 *
 * @param [in] arg your marshalled argument into the listener.
 * @param [in] r the rank of my hand.
 */
typedef void (*rank_listener) (void *arg, enum card_rank r);

/**
 * Runs a Razz game for a number of times.
 *
 * @param [in] decided_cards the cards that will not be included in the simulated
 *                           dealing.
 * @param [in] game_count the number of Razz games to be simulated.
 * @param [in] arg your marshalled argument into the listener.
 * @param [in] listener the callback function that will be invoked with the rank
 *                      of my hand at the end of each game.
 *
 * @return 0 if the simulation encounters no error or non-zero if it encounters
 *         one.
 */
int
simulate_razz_game (const struct decided_cards *decided_cards,
		    unsigned long game_count,
		    void *arg,
		    rank_listener listener);

#ifdef __cplusplus
}
#endif

#endif /* RAZZ_SIMULATION_H */
