#ifndef RUBIK_MODEL_H

#define RUBIK_MODEL_H

#include "helpers.h"

typedef enum { RED, GREEN, BLUE, ORANGE, YELLOW, WHITE } color;       // Also represents each face of the cube
static const color top[] = {GREEN, BLUE, RED, WHITE, ORANGE, YELLOW}; // Map current face to their top face
static const char color_code[] = {'R', 'G', 'B', 'O', 'Y', 'W'};      // Color code is used for printing
typedef enum { CW, CCW } rotation;                                    // Clockwise or counter-clockwise
typedef color ** cube_t;
typedef struct ecube_struct
{
  cube_t cube;
  int entropy;
  int hash;
}*ecube_t;

#define REAR(x)         ((x + 3) % 6)
#define CYCLE_L(x)      ((x + 5) % 6)
#define CYCLE_R(x)      ((x + 1) % 6)

#define SWAP_COLOR(x, y)  SWAP_ENUM(color, x, y)

cube_t populate_initial();
cube_t populate_specific(const color cube_data[][8]);
unsigned int cube_hash(const cube_t cube);
ecube_t ecube_init(cube_t cube);
cube_t cube_clone(const cube_t cube);
void rotate_face(cube_t cube, color face, rotation direction);
cube_t new_cube_rotate_face(const cube_t cube, color face, rotation direction);
bool cube_compare_equal(const cube_t cube1, const cube_t cube2);
void print_cube(const cube_t cube);
int find_entropy(const cube_t cube);
void test_adj_functions(); //adjacent_cw, adjacent_ccw, adjacent_left, adjacent_right are private functions
void cube_free(cube_t cube);
#endif
