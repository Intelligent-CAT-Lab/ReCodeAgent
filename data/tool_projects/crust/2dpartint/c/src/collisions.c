#include <math.h>
#include <stddef.h>
#include "data.h"
#include "functions.h"
#include "collisions.h"

/**
 * Helper function. Return the column number given an x coordinate. If the x coordinate lies outside the x dimension covered by the grid,
 * then return a negative number: -2 for the left boundary and -1 for the right boundary.
 */
int find_col(const double x, const int x_squares, const double square_length){
    double x_left_limit = -(x_squares*square_length/2); // The far leftest (if that word exists) of the grid
    double diff = x_left_limit -x;
    if(diff>0) return -2; // outside the left boundary
    int col_ind = floor(fabs(diff)/square_length);
    if(col_ind>=x_squares) return -1; // outisde the right boundary
    return col_ind;
}

/**
 * Helper function. Return the row number given an y coordinate. If the y coordinate lies below 0,
 * then return a negative number: -2 for the left boundary and -1 for the right boundary.
 */
int find_row(const double y, const int y_squares, const double square_length){
    int row_ind = floor(((double)y)/square_length);
    if(row_ind >= y_squares) return -1;
    if(row_ind < 0) return -2;
    return row_ind;
}

/**
 * Helper function. Given a point (x, y), find the index for the square that should contain it. If either coordinate lies outide the area covered
 * by the grid, then return -1.
 */
int find_square(const double x, const double y, const int x_squares, const int y_squares, const double square_length){
    int row_ind = find_row(y,y_squares,square_length);
    int col_ind = find_col(x,x_squares,square_length);
    if(col_ind < 0 || row_ind < 0){
        return -1;
    }
    return row_ind * x_squares + col_ind;
}

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
size_t compute_contacts(Particle const *const *const grid, const int x_squares, const int y_squares, const double square_length, Contact* contacts){
    size_t k = 0; // current number of contacts
    // For each square
    for(int row=0; row<y_squares; row++){
        for(int col=0; col<x_squares; col++){
            size_t square_idx = row*x_squares+col;
            if(!grid[square_idx]) continue; // If the square is empty
            Particle* first = (Particle*)grid[square_idx]; // Cast so the compiler does not yell because of referencing a const pointer
            Particle* p =(Particle*)grid[square_idx]; // Iterator. Cast so the compiler does not yell because of referencing a const pointer
            while(p){
                Particle* other = first;
                while(other){ // First compare with particles within the same square
                    if(other != p){
                        const double overlap = compute_overlap(p, other);
                        if(overlap > 0){
                            contacts[k].p1_idx = p->idx;
                            contacts[k].p2_idx = other->idx;
                            contacts[k].overlap = overlap;
                            k++;
                        }
                    }
                    other = other->next;
                }
                // Then, compare with p's surrounding squares
                // Find all the squares which could contain particles colliding with p
                int left_col = find_col(p->x_coordinate - p->radius*2, x_squares, square_length);
                if(left_col == -2) left_col = 0;
                int right_col = find_col(p->x_coordinate + p->radius*2, x_squares, square_length);
                if(right_col == -1) right_col = x_squares-1;
                int bottom_row = find_row(p->y_coordinate - p->radius*2, y_squares, square_length);
                if(bottom_row==-2) bottom_row = 0;
                int top_row = find_row(p->y_coordinate + p->radius*2, y_squares, square_length);
                if(top_row==-1) top_row=y_squares-1;
                // Iterate over the squares
                for(int neighbor_row=bottom_row; neighbor_row<=top_row; neighbor_row++){
                    for(int neighbor_col=left_col; neighbor_col<=right_col;neighbor_col++){
                        size_t neighbor_square_idx = neighbor_row*x_squares+neighbor_col;
                        other = (Particle *)grid[neighbor_square_idx]; // Cast so the compiler does not yell because of referencing a const pointer
                        if(other==NULL) continue; // If there are no particles inside this square
                        if(neighbor_square_idx==square_idx) continue; // If this is p's square, then this has already been traversed
                        while(other){
                            const double overlap = compute_overlap(p, other);
                            if(overlap > 0){
                                contacts[k].p1_idx = p->idx;
                                contacts[k].p2_idx = other->idx;
                                contacts[k].overlap = overlap;
                                k++;
                            }
                            other = other->next;
                        }
                    }
                }
                p = p->next;
            }
        }
    }
    return k;
}

/**
 * Fills the grid with particles. The side length of each square grid is twice the diameter of each particle,
 * since all particles have the same radius. Running time is O(N), and memory space is O(N) but really 2*N because of 'lasts' pointers array.
 */
void fill_grid(const size_t num_particles, const int x_squares, const int y_squares, const double square_length,
        Particle const *const particles, Particle* *const grid, Particle* *const grid_lasts){
    // For each particle, find its grid's square. Keep in mind that grid is an array of pointers to the first element in the squares, and that
    // grid_lasts is an array of pointers to the last element in the squares.
    for(size_t i=0; i<num_particles; i++){
        Particle* p = (Particle*) &particles[i]; // Cast so the compiler does not yell because of referencing a const pointer
        p->next=NULL; // Make sure we do not do anything funny
        int square_ind = find_square(p->x_coordinate, p->y_coordinate, x_squares, y_squares, square_length);
        if(square_ind< 0) continue; // The particle is outside the boundaries of our grid
        if(!grid_lasts[square_ind]){ // if is the first element in the square
            grid[square_ind] = p;
            grid_lasts[square_ind] = p;
        }else{
            grid_lasts[square_ind]->next=p;
            grid_lasts[square_ind] = p;
        }
    }

}
