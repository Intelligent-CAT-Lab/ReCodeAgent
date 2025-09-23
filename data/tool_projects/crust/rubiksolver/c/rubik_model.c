#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

#include "rubik_model.h"

static color adjacent_left(color rotating_face, color around)
{
  color l_around = CYCLE_L(around);
  if(l_around == REAR(rotating_face) || l_around == rotating_face)
  {
    return CYCLE_L(l_around);
  }
  return l_around;
}

static color adjacent_right(color rotating_face, color around)
{
  color r_around = CYCLE_R(around);
  if(r_around == REAR(rotating_face) || r_around == rotating_face)
  {
    return CYCLE_R(r_around);
  }
  return r_around;
}

static color adjacent_cw(color rotating_face, color around)
{
  assert(around != REAR(rotating_face));
  if(IS_ODD(rotating_face))
  {
    return adjacent_right(rotating_face, around);
  }
  return adjacent_left(rotating_face, around);
}

static color adjacent_ccw(color rotating_face, color around)
{
  assert(around != REAR(rotating_face));
  if(IS_ODD(rotating_face))
  {
    return adjacent_left(rotating_face, around);
  }
  return adjacent_right(rotating_face, around);
}

cube_t populate_initial()
{
  cube_t cube = (color **)(malloc(sizeof(color *) * 6));
  for(color face = RED; face <= WHITE; face++)
  {
    cube[face] = (color *)(malloc(sizeof(color) * 8));
    for(int pos = 0; pos < 8; pos++)
    {
      cube[face][pos] = face;
    }
  }
  return cube;
}

cube_t populate_specific(const color data[][8])
{
  cube_t cube = (color **)(malloc(sizeof(color *) * 6));
  for(color face = RED; face <= WHITE; face++)
  {
    cube[face] = (color *)(malloc(sizeof(color) * 8));
    for(int pos = 0; pos < 8; pos++)
    {
      cube[face][pos] = data[face][pos];
    }
  }
  return cube;
}

void cube_free(cube_t cube)
{
  for(color face = RED; face <= WHITE; face++)
  {
    free(cube[face]);
  }
  free(cube);
}

unsigned int cube_hash(const cube_t cube)
{
  unsigned int hash = 0;
  for(color face = RED; face <= WHITE; face++)
  {
    for(int pos = 0; pos < 4; pos++)
    {
      if(cube[face][2 * pos] != face || cube[face][2 * pos + 1] != face)
      {
        hash++;
      }
      hash = hash << 1;
    }
  }
  return hash;
}

ecube_t ecube_init(cube_t cube)
{
  ecube_t ecube = (ecube_t)malloc(sizeof(struct ecube_struct));
  ecube->cube = cube;
  ecube->entropy = find_entropy(cube);
  ecube->hash = cube_hash(cube);
  return ecube;
}

cube_t cube_clone(cube_t cube)
{
  cube_t new_cube = (cube_t)malloc(sizeof(color *) * 6);
  for(color face = RED; face <= WHITE; face++)
  {
    new_cube[face] = (color *)(malloc(sizeof(color) * 8));
    for(int pos = 0; pos < 8; pos++)
    {
      new_cube[face][pos] = cube[face][pos];
    }
  }
  return new_cube;
}

cube_t new_cube_rotate_face(const cube_t cube, color face, rotation direction)
{
  cube_t new_cube = cube_clone(cube);
  rotate_face(new_cube, face, direction);
  return new_cube;
}

void rotate_face(cube_t cube, color face, rotation direction)
{
  assert(cube != NULL);
  /*
            + - - - +
            | 2 3 4 |
            | 1 G 5 |
            | 0 7 6 |
    + - - - + - - - + - - - + - - - +
    | 6 7 0 | 0 1 2 | 4 5 6 | 2 3 4 |      'R' will be rotated either CW/CCW
    | 5 B 1 | 7 R 3 | 3 W 7 | 1 O 5 |
    | 4 3 2 | 6 5 4 | 2 1 0 | 0 7 6 |
    + - - - + - - - + - - - + - - - +
            | 4 5 6 |
            | 3 Y 7 |
            | 2 1 0 |
            + - - - +
  */
  color g = top[face];
  color w = adjacent_cw(face, g);
  color y = adjacent_cw(face, w);
  color b = adjacent_cw(face, y);
  color temp;
  if(direction == CW)
  {
    // Swap G's (0,7,6) with W's (4,3,2)
    SWAP_COLOR(cube[g][0], cube[w][4]);
    SWAP_COLOR(cube[g][7], cube[w][3]);
    SWAP_COLOR(cube[g][6], cube[w][2]);
    /*
    do
    {
      color temp = cube[g][6];
      cube[g][6] = cube[w][2];
      cube[w][2] = temp;
    } while(0);
    */
    // Swap B's (2,1,0) with G's (0,7,6)
    SWAP_COLOR(cube[b][2], cube[g][0]);
    SWAP_COLOR(cube[b][1], cube[g][7]);
    SWAP_COLOR(cube[b][0], cube[g][6]);
    
    // Swap Y's (6,5,4) with B's (2,1,0)
    SWAP_COLOR(cube[y][6], cube[b][2]);
    SWAP_COLOR(cube[y][5], cube[b][1]);
    SWAP_COLOR(cube[y][4], cube[b][0]);
    
    // Rotating R's face CW
    temp = cube[face][0];
    cube[face][0] = cube[face][6];
    cube[face][6] = cube[face][4];
    cube[face][4] = cube[face][2];
    cube[face][2] = temp;
    temp = cube[face][1];
    cube[face][1] = cube[face][7];
    cube[face][7] = cube[face][5];
    cube[face][5] = cube[face][3];
    cube[face][3] = temp;
  }
  else// if(dir == CCW)
  {
    // Swap G's (0,7,6) with B's (2,1,0)
    SWAP_COLOR(cube[g][0], cube[b][2]);
    SWAP_COLOR(cube[g][7], cube[b][1]);
    SWAP_COLOR(cube[g][6], cube[b][0]);
    
    // Swap W's (4,3,2) with G's (0,7,6)
    SWAP_COLOR(cube[w][4], cube[g][0]);
    SWAP_COLOR(cube[w][3], cube[g][7]);
    SWAP_COLOR(cube[w][2], cube[g][6]);
    
    // Swap Y's (6,5,4) with W's (4,3,2)
    SWAP_COLOR(cube[y][6], cube[w][4]);
    SWAP_COLOR(cube[y][5], cube[w][3]);
    SWAP_COLOR(cube[y][4], cube[w][2]);
    
    // Rotating R's face CCW
    temp = cube[face][0];
    cube[face][0] = cube[face][2];
    cube[face][2] = cube[face][4];
    cube[face][4] = cube[face][6];
    cube[face][6] = temp;
    
    temp = cube[face][1];
    cube[face][1] = cube[face][3];
    cube[face][3] = cube[face][5];
    cube[face][5] = cube[face][7];
    cube[face][7] = temp;
  }
}

void print_cube(const cube_t cube)
{
  for(color face = RED; face <= WHITE; face++)
  {
    printf("%c %c %c\n%c %c %c\n%c %c %c\n",
        color_code[cube[face][0]],
        color_code[cube[face][1]],
        color_code[cube[face][2]],
        color_code[cube[face][7]],
        color_code[face],
        color_code[cube[face][3]],
        color_code[cube[face][6]],
        color_code[cube[face][5]],
        color_code[cube[face][4]]
    );
    printf("-----\n");
  }
}

bool cube_compare_equal(const cube_t cube1, const cube_t cube2)
{
  for(color face = RED; face <= WHITE; face++)
  {
    for(int pos = 0; pos < 8; pos++)
    {
      if(cube1[face][pos] != cube2[face][pos])
      {
        return false;
      }
    }
  }
  return true;
}

int find_entropy(const cube_t cube)
{
  int count = 0;
  for(color face = RED; face <= WHITE; face++)
  {
    for(int pos = 0; pos < 8; pos++)
    {
      if(cube[face][pos] != face)
        count++;
    } 
  }
  return count;
}

void test_adj_functions()
{
  printf("Testing adjacent_cw...\n");
  assert_verbose(adjacent_cw(RED, YELLOW) == BLUE);
  assert_verbose(adjacent_cw(RED, BLUE) == GREEN);
  assert_verbose(adjacent_cw(RED, WHITE) == YELLOW);
  assert_verbose(adjacent_cw(RED, GREEN) == WHITE);
  assert_verbose(adjacent_cw(GREEN, RED) == BLUE);
  assert_verbose(adjacent_cw(GREEN, WHITE) == RED);
  assert_verbose(adjacent_cw(GREEN, ORANGE) == WHITE);
  assert_verbose(adjacent_cw(GREEN, BLUE) == ORANGE);
  assert_verbose(adjacent_cw(WHITE, ORANGE) == YELLOW);
  assert_verbose(adjacent_cw(WHITE, GREEN) == ORANGE);
  assert_verbose(adjacent_cw(YELLOW, WHITE) == ORANGE);
  assert_verbose(adjacent_cw(YELLOW, RED) == WHITE);
  
  printf("Testing adjacent_ccw...\n");
  assert_verbose(adjacent_ccw(RED, BLUE) == YELLOW);
  assert_verbose(adjacent_ccw(RED, GREEN == BLUE));
  assert_verbose(adjacent_ccw(RED, YELLOW) == WHITE);
  assert_verbose(adjacent_ccw(RED, WHITE) == GREEN);
  assert_verbose(adjacent_ccw(GREEN, BLUE) == RED);
  assert_verbose(adjacent_ccw(GREEN, RED) == WHITE);
  assert_verbose(adjacent_ccw(GREEN, WHITE) == ORANGE);
  assert_verbose(adjacent_ccw(GREEN, ORANGE) == BLUE);
  assert_verbose(adjacent_ccw(WHITE, YELLOW) == ORANGE);
  assert_verbose(adjacent_ccw(WHITE, ORANGE) == GREEN);
  assert_verbose(adjacent_ccw(YELLOW, ORANGE) == WHITE);
  assert_verbose(adjacent_ccw(YELLOW, WHITE) == RED);
}
