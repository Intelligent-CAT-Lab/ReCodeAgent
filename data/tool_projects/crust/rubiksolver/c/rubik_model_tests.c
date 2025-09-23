#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

#include "rubik_model.h"

int main()
{
  printf("Running rubik_model tests\n");
  
  printf("Testing REAR...\n");
  assert_verbose(REAR(RED) == ORANGE);
  assert_verbose(REAR(GREEN) == YELLOW);
  assert_verbose(REAR(BLUE) == WHITE);
  assert_verbose(REAR(REAR(RED)) == RED);
  assert_verbose(REAR(REAR(GREEN)) == GREEN);
  assert_verbose(REAR(REAR(BLUE)) == BLUE);
  
  // Call function to test private functions(adjacent_cw, adjacent_ccw,...)
  test_adj_functions();
  
  printf("Testing cube_compare_equal...\n");
  const color test_cube_data[][8] = {
      {RED, RED, RED, RED, RED, RED, RED, RED},
      {GREEN, YELLOW, BLUE, ORANGE, ORANGE, YELLOW, GREEN, GREEN},
      {BLUE, BLUE, BLUE, ORANGE, ORANGE, GREEN, ORANGE, BLUE},
      {WHITE, WHITE, WHITE, GREEN, GREEN, WHITE, YELLOW, BLUE},
      {ORANGE, GREEN, BLUE, YELLOW, YELLOW, YELLOW, YELLOW, BLUE},
      {YELLOW, ORANGE, WHITE, WHITE, WHITE, WHITE, GREEN, ORANGE}
    };
  cube_t test_cube = populate_specific(test_cube_data);
  cube_t original = populate_specific(test_cube_data);

  const color output1_data[][8] = {
      {RED, RED, RED, RED, BLUE, ORANGE, ORANGE, RED},
      {GREEN, YELLOW, BLUE, ORANGE, ORANGE, YELLOW, GREEN, GREEN},
      {BLUE, BLUE, YELLOW, BLUE, WHITE, GREEN, ORANGE, BLUE},
      {WHITE, WHITE, WHITE, GREEN, GREEN, WHITE, YELLOW, ORANGE},
      {YELLOW, BLUE, ORANGE, GREEN, BLUE, YELLOW, YELLOW, YELLOW},
      {RED, RED, RED, WHITE, WHITE, WHITE, GREEN, ORANGE}
    };
  cube_t output1 = populate_specific(output1_data);

  assert_verbose(cube_compare_equal(test_cube, original));
  assert_verbose(!cube_compare_equal(test_cube, output1));
  
  printf("Testing SWAP_COLOR...\n");
  color r = RED, g = GREEN;
  SWAP_COLOR(r,g);
  assert_verbose(r == GREEN && g == RED);
  SWAP_COLOR(r,g);
  assert_verbose(r == RED && g == GREEN);
  
  printf("Testing rotate_face...\n");
  rotate_face(test_cube, YELLOW, CW);
  assert_verbose(cube_compare_equal(test_cube, output1));
  rotate_face(test_cube, YELLOW, CCW);
  assert_verbose(cube_compare_equal(test_cube, original));
  
  printf("Testing find_entropy...\n");
  assert_verbose(find_entropy(original) == 25);
  
  cube_free(test_cube);
  cube_free(original);
  cube_free(output1);
  printf("rubik_model tests finished successfully\n");
  return 0;
}
