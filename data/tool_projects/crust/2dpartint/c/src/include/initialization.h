#pragma once

#include "config.h"

/**
 * Initialize all simulation data structures,
 * according to the simulation size.
 * Returns the number of initialized particles.
 *
 * Note: Except for the particles,
 * all structures are effectively initialized with zeros.
 */
size_t initialize(const Config *config);
