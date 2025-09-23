#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

#include "rubik_model.h"
#include "heap/heap.h"
#include "hash/hash.h"

bool is_better(const void *ec1, const void *ec2)
{
  ecube_t ecube1 = (ecube_t)ec1;
  ecube_t ecube2 = (ecube_t)ec2;
  if(ecube1->entropy < ecube2->entropy)
    return true;
  else
    return false;
}

unsigned int hash_function(const void *element)
{
  ecube_t ecube = (ecube_t)element;
  return ecube->hash;
}

bool ecube_compare_equal(const void *ec1, const void *ec2)
{
  ecube_t ecube1 = (ecube_t)ec1;
  ecube_t ecube2 = (ecube_t)ec2;
  assert(ecube1 != NULL && ecube2 != NULL);
  if(ecube1->entropy != ecube2->entropy)
    return false;
  return (cube_compare_equal((void *)(ecube1->cube), (void *)(ecube2->cube)));
}

int main()
{
  /**/
  const color cube_data[][8] = {
      {RED, RED, RED, RED, RED, RED, RED, RED},
      {GREEN, YELLOW, BLUE, ORANGE, ORANGE, YELLOW, GREEN, GREEN},
      {BLUE, BLUE, BLUE, ORANGE, ORANGE, GREEN, ORANGE, BLUE},
      {WHITE, WHITE, WHITE, GREEN, GREEN, WHITE, YELLOW, BLUE},
      {ORANGE, GREEN, BLUE, YELLOW, YELLOW, YELLOW, YELLOW, BLUE},
      {YELLOW, ORANGE, WHITE, WHITE, WHITE, WHITE, GREEN, ORANGE}
    };
  /*/
  const color cube_data[][8] = {
      {RED, RED, RED, RED, RED, RED, RED, RED},
      {BLUE, GREEN, GREEN, GREEN, GREEN, GREEN, BLUE, BLUE},
      {YELLOW, YELLOW, YELLOW, BLUE, BLUE, BLUE, BLUE, BLUE},
      {ORANGE, ORANGE, ORANGE, ORANGE, ORANGE, ORANGE, ORANGE, ORANGE},
      {YELLOW, YELLOW, YELLOW, YELLOW, WHITE, WHITE, WHITE, YELLOW},
      {WHITE, WHITE, GREEN, GREEN, GREEN, WHITE, WHITE, WHITE}
    };
  /*/
  int count_explored = 0, count_unexplored = 1;
  cube_t cube = populate_specific(cube_data);
  ecube_t ecube = ecube_init(cube);
  hash_t unexplored_hash = hash_init(33554432, hash_function);
  heap_t unexplored = heap_init(5000, is_better);
  
  heap_insert(unexplored, ecube);
  hash_insert(unexplored_hash, ecube);
  hash_t explored_hash = hash_init(33554432, hash_function);
  
  while(!heap_is_empty(unexplored))
  {
    printf("Unexplored nodes = %6d\n", count_unexplored);
    ecube_t x = heap_delete_min(unexplored);
    hash_delete(unexplored_hash, x, ecube_compare_equal);
    count_unexplored--;
    printf("Entropy of x: %d\n", x->entropy);
    /*
    if(count_explored > 100000)
      break;
    */
    if(x->entropy == 0)
    {
      printf("Found goal.\n");
      exit(0);
    }
    hash_insert(explored_hash, x);
    count_explored++;
    printf("Explored nodes = %6d\n", count_explored);
    for(color face = 0; face <= 5; face++)
    {
      for(rotation dir = 0; dir <= 1; dir++)
      {
        cube_t new_cube = new_cube_rotate_face(x->cube, face, dir);
        ecube_t y = ecube_init(new_cube);
        if(hash_element_exists(explored_hash, y, ecube_compare_equal))
        {
          continue;
        }
        if(!hash_element_exists(unexplored_hash, y, ecube_compare_equal))
        {
          heap_insert(unexplored, y);
          hash_insert(unexplored_hash, y);
          count_unexplored++;
        }
      }
    }
  }
}
