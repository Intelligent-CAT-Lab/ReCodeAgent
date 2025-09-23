#pragma once
#include "data.h"

/**
 * From the Grid, find all the pairs of particles that are colliding with each other, create a struct Contact for
 * each one and save it into 'contacts'. Returns the number of collisions.
 *
 * The search for contacts is done as following:
 *     1. Traverse each square in the grid. If it is empty (there is a null pointer in grid[square_idx]), then continue to the next square.
 *          2. Iterate each particle inside the square. The particle iterated over will be 'p'.
 *              3. If p is the only particle inside the square, continue to 5.
 *                  4. Traverse all the square to find collisions with its  square-neighbors.
 *              5. Traverse the particles that are inside squares that could be containing particles colliding with p. The squares can be found with the following method:
 *                  a. Imagine a circle with twice the radius of p, called neighboring cicle. Every other coliding particle's center 
 *                     must be inside the neighboring circle, since all have the same radius. 
 *                  b. We have to find the set of squares such that part of its geometric area is overlaping with p's neighboring circle.
 *                     We do not do this, but something similar. Instead of using a circle, we use the square in which the neighboring circle is inscribed.
 *                  c. We find the rows and columns that contain the limits (upper, lower, left and right) of said square. These will define
 *                     us the set of squares inside Grid that could contain p's colliding particles.
 *                  d. Traverse all the particles inside each found square to find collisions.
 */
size_t compute_contacts(Particle const *const *const grid, const int x_squares, const int y_squares, const double square_length, Contact* contacts);

/**
 * Fills the grid with particles. This updates the value of a pointer in grid when a particle is inside a square, and updates the pointer of the last
 * particle in the last position of the singly linked list of the square. Running time is O(N), and memory space is O(N) but really 2*N because of 'grid_lasts' pointers array.
 */
void fill_grid(const size_t num_particles, const int x_squares, const int y_squares, const double square_length, Particle const *const particles, Particle* *const grid, Particle** grid_lasts);
