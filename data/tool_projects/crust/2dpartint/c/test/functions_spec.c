#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include "data.h"
#include "functions.h"

// Maximum acceptable error when comparing double values.
#define TOLERANCE 0.00005d

// Terminal color constants.
#define RED   "\033[1;31m"
#define GREEN "\033[1;32m"
#define RESETCOLOR "\033[0m"

// Keeps track of the number of failed tests.
unsigned int number_failed;

/**
 * Asserts that two double values are equals, within the allowed tolerance.
 * Prints to console if the test passed or not.
 */
void assert(const double result, const double expected, const char* test_description) {
  if (fabs(result - expected) > TOLERANCE) {
    ++number_failed;
    printf(RED "Test: '%s' FAILED!\n\t%f did not equal %f\n" RESETCOLOR, test_description, expected, result);
  } else {
    printf(GREEN "Test: '%s' PASSED!\n" RESETCOLOR, test_description);
  }
}

/**
 * Asserts that two double values are equals, within the allowed tolerance.
 * Prints to console if the test passed or not, together with the iteration number.
 */
void for_assert(const double result, const double expected, const char* test_description, const int i) {
  if (fabs(result - expected) > TOLERANCE) {
    ++number_failed;
    printf(RED "Test: '%s' (%i) FAILED!\n\t%f did not equal %f\n" RESETCOLOR, test_description, i, expected, result);
  } else {
    printf(GREEN "Test: '%s' (%i) PASSED!\n" RESETCOLOR, test_description, i);
  }
}

/**
 * Checks that compute_forces works for two particles with only one contact.
 */
void test_compute_forces_one_contact() {
  #define size 2
  #define contacts_size 1

  ParticleProperties properties[size] = {
    { 0.049, 247435.829652697, 19033.5253578998 },
    { 0.049, 247435.829652697, 19033.5253578998 }
  };
  Particle particles[size] = {
    { 24.9999428493601, 25, 50 , NULL, 0},
    { 24.7762980060664, 74.6253249615872, 50, NULL, 1 }
  };
  Vector velocities[size] = {
    { -0.00159733057834793, 0 },
    { -4.64544001147225, -6.36971454859269 }
  };
  double normal_forces[size * size] = {
    0, 53.2634277334533,
    53.2634277334533, 0
  };
  double tangent_forces[size * size] = {
    0, 2.0526062679878,
    2.0526062679878, 0
  };
  double dt = 0.000025;
  Contact contacts[contacts_size] = { { 0, 1, 42.9554 } };
  Vector resultant_forces[size] = { { 0 } };

  compute_forces(dt, size, contacts_size, particles, properties, contacts,
                 velocities, normal_forces, tangent_forces, resultant_forces);

  // P1.
  assert(resultant_forces[0].x_component, -3.858892943d, "test_compute_forces_one_contact - resultant_forces[P1].x_component");
  assert(resultant_forces[0].y_component, -93.03497946d, "test_compute_forces_one_contact - resultant_forces[P1].y_component");
  assert(normal_forces[2], 92.53595901628790d, "test_compute_forces_one_contact - normal_forces[P1 <- P2]");
  assert(tangent_forces[2], 4.275960624d, "test_compute_forces_one_contact - tangent_forces[P1 <- P2]");

  // P2.
  assert(resultant_forces[1].x_component, 3.858892943d, "test_compute_forces_one_contact - resultant_forces[P2].x_component");
  assert(resultant_forces[1].y_component, 92.07359946d, "test_compute_forces_one_contact - resultant_forces[P2].y_component");
  assert(normal_forces[1], 92.53595901628790d, "test_compute_forces_one_contact - normal_forces[P2 <- P1]");
  assert(tangent_forces[1], 4.275960624d, "test_compute_forces_one_contact - tangent_forces[P2 <- P1]");

  #undef size
  #undef contacts_size
}

/**
 * Checks that compute_forces works for many particles with multiple contacts.
 */
void test_compute_forces_multiple_contacts() {
  #define size 9
  #define contacts_size 3

  ParticleProperties properties[size] = {
    { 0.049, 247435.829652697, 19033.5253578998 }, { 0.049, 247435.829652697, 19033.5253578998 },
    { 0.049, 247435.829652697, 19033.5253578998 }, { 0.049, 247435.829652697, 19033.5253578998 },
    { 0.049, 247435.829652697, 19033.5253578998 }, { 0.049, 247435.829652697, 19033.5253578998 },
    { 0.049, 247435.829652697, 19033.5253578998 }, { 0.049, 247435.829652697, 19033.5253578998 },
    { 0.049, 247435.829652697, 19033.5253578998 }
  };
  Particle particles[size] = {
    { 24.9996682317, 25, 50, NULL, 0}, { 24.3329247490, 74.1788246441, 50, NULL, 1 },
    { 20.8181703235, 122.9449251651, 50, NULL, 2 }, { 16.8606509998, 172.4918861911, 50, NULL, 3 },
    { 75.0003317683, 25, 50, NULL, 4 }, { 75.6670752510, 74.1788246441, 50, NULL, 5 },
    { 79.1818296765, 122.9449251651, 50, NULL, 6 }, { 83.1393490002, 172.4918861911, 50, NULL, 7 },
    { 50, 75.7713467697, 50, NULL, 8 }
  };
  Vector velocities[size] = {
    { -0.00728767793878, 0 }, { -10.44576385514790, -9.61529833378254 },
    { -22.72352179173930, -6.11340796404052 }, { -22.22735951506210, -4.22438903830371 },
    { 0.00728767793878, 0 }, { 10.44576385514790, -9.61529833378254 },
    { 22.72352179173930, -6.11340796404052 }, { 22.22735951506210, -4.22438903830371 },
    { -0.00000000000001, -259.77483981875700 }
  };
  double normal_forces[size * size] = {
    0, 143.165149, 0, 0, 0, 0, 0, 0, 0,
    201.763569, 0, 297.810731, 0, 0, 0, 0, 0, 6205.981479,
    0, 297.810731, 0, 81.040751, 0, 0, 0, 0, 0,
    0, 0, 81.040751, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 143.165149, 0, 0, 0,
    0, 0, 0, 0, 143.165149, 0, 297.810731, 0, 6205.981479,
    0, 0, 0, 0, 0, 297.810731, 0, 81.040751, 0,
    0, 0, 0, 0, 0, 0, 81.040751, 0, 0,
    0, 6205.981479, 0, 0, 0, 6205.981479, 0, 0, 0
  };

  double tangent_forces[size * size] = {
    0, 7.77476, 0, 0, 0, 0, 0, 0, 0,
    12.803354, 0, 61.408871, 0, 0, 0, 0, 0, -553.7956,
    0, 61.40887, 0, 46.788899128, 0, 0, 0, 0, 0,
    0, 0, 46.788899, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, -7.77476, 0, 0, 0,
    0, 0, 0, 0, -7.7747607, 0, -61.408871, 0, 553.7956,
    0, 0, 0, 0, 0, -61.4089, 0, -46.78890, 0,
    0, 0, 0, 0, 0, 0, -46.788899, 0, 0,
    0, -553.79557, 0, 0, 0, 553.7956, 0, 0, 0
  };
  double dt = 0.000025;
  Contact contacts[contacts_size] = {
    { 1, 0, 49.18334413 }, { 1, 2, 48.89259718 }, { 1, 8, 25.71643207 }
  };
  Vector resultant_forces[size] = { { 0 } };

  compute_forces(dt, size, contacts_size, particles, properties, contacts,
                 velocities, normal_forces, tangent_forces, resultant_forces);

  // P2.
  assert(resultant_forces[1].x_component, -6221.088058d, "test_compute_forces_multiple_contacts - resultant_forces[P2].x_component");
  assert(resultant_forces[1].y_component, -1131.303630d, "test_compute_forces_multiple_contacts - resultant_forces[P2].y_component");
  assert(normal_forces[1], 201.7636d, "test_compute_forces_multiple_contacts - normal_forces[P2 <- P1]");
  assert(tangent_forces[1], 12.80335446d, "test_compute_forces_multiple_contacts - tangent_forces[P2 <- P1]");
  assert(normal_forces[19], 270.7447d, "test_compute_forces_multiple_contacts - normal_forces[P2 <- P3]");
  assert(tangent_forces[19], 67.11619734d, "test_compute_forces_multiple_contacts - tangent_forces[P2 <- P3]");
  assert(normal_forces[73], 6237.3175d, "test_compute_forces_multiple_contacts - normal_forces[P2 <- P9]");
  assert(tangent_forces[73], -672.91035983d, "test_compute_forces_multiple_contacts - tangent_forces[P2 <- P9]");

  // P1.
  assert(resultant_forces[0].x_component, -14.30076729d, "test_compute_forces_multiple_contacts - resultant_forces[P1].x_component");
  assert(resultant_forces[0].y_component, -261.0604892, "test_compute_forces_multiple_contacts - resultant_forces[P1].y_component");
  assert(normal_forces[9], 260.3620, "test_compute_forces_multiple_contacts - normal_forces[P1 <- P2]");
  assert(tangent_forces[9], 17.83194748, "test_compute_forces_multiple_contacts - tangent_forces[P1 <- P2]");

  // P3.
  assert(resultant_forces[2].x_component, 47.47945664d, "test_compute_forces_multiple_contacts - resultant_forces[P3].x_component");
  assert(resultant_forces[2].y_component, 274.3883165d, "test_compute_forces_multiple_contacts - resultant_forces[P3].y_component");
  assert(normal_forces[11], 270.7447d, "test_compute_forces_multiple_contacts - normal_forces[P3 <- P2]");
  assert(tangent_forces[11], 67.11619348d, "test_compute_forces_multiple_contacts - tangent_forces[P3 <- P2]");

  // P9.
  assert(resultant_forces[8].x_component, 6183.675611d, "test_compute_forces_multiple_contacts - resultant_forces[P9].x_component");
  assert(resultant_forces[8].y_component, 1057.391839d, "test_compute_forces_multiple_contacts - resultant_forces[P9].y_component");
  assert(normal_forces[17], 6237.3175d, "test_compute_forces_multiple_contacts - normal_forces[P9 <- P2]");
  assert(tangent_forces[17], -672.91036059d, "test_compute_forces_multiple_contacts - tangent_forces[P9 <- P2]");

  #undef size
  #undef contacts_size
}

/**
 * Checks that the compute_acceleration function works for arrays of one element.
 */
void test_compute_acceleration_one_element() {
  #define size 1
  Vector forces[size] = { { 30, 30 } };
  ParticleProperties particle_properties[size] = { { 3, 0, 0 } };
  Vector accelerations[size] = { { 0 } };

  compute_acceleration(0, particle_properties, forces, accelerations);

  assert(accelerations[0].x_component, 10.0d, "test_compute_acceleration_one_element - x_component");
  assert(accelerations[0].y_component, 10.0d, "test_compute_acceleration_one_element - y_component");
  #undef size
}

/**
 * Checks that the compute_acceleration function works for arrays of multiple elements.
 */
void test_compute_acceleration_multiple_elements() {
  #define size 3
  Vector forces[size] = { { -12.58, -15.896 }, { 13.945, -200.826 }, { -543.62, -0.62 } };
  ParticleProperties particle_properties[size] = { { 0.367, 0, 0 }, { 3.967, 0, 0 }, { 0.52, 0, 0 } };
  Vector accelerations[size] = { { 0 } };

  compute_acceleration(0, particle_properties, forces, accelerations);
  compute_acceleration(1, particle_properties, forces, accelerations);
  compute_acceleration(2, particle_properties, forces, accelerations);

  Vector expected[size] = { { -34.2779d, -43.31335149863761d }, { 3.5152508192588856d, -50.6241d }, { -1045.4231d, -1.1923d } };
  for (size_t i = 0; i < size; ++i) {
    for_assert(accelerations[i].x_component, expected[i].x_component, "test_compute_acceleration_multiple_elements - x_component", i);
    for_assert(accelerations[i].y_component, expected[i].y_component, "test_compute_acceleration_multiple_elements - y_component", i);
  }
  #undef size
}

/**
 * Checks that the compute_velocity function works for arrays of one element.
 */
void test_compute_velocity_one_element() {
  #define size 1
  Vector accelerations[size] = { { 42.53, -631.431 } };
  double dt = 0.00025;
  Vector velocities[size] = { { 0, 0 } };

  compute_velocity(dt, 0, accelerations, velocities);

  assert(velocities[0].x_component, 0.010632500000000001d, "test_compute_velocity_one_element - x_component");
  assert(velocities[0].y_component, -0.15785775000000002d, "test_compute_velocity_one_element - y_component");
  #undef size
}

/**
 * Checks that the compute_velocity function works for arrays of multiple elements.
 */
void test_compute_velocity_multiple_elements() {
  #define size 3
  Vector accelerations[size] = { { -10.59, 162.35 }, { -53223.12, 4212.124}, { 532521.2124124, 12142.512124} };
  double dt = 0.00025;
  Vector velocities[size] = { { 5.332, 2.123 }, { 7.12, 8.96}, { 61.52, 1293.123} };

  compute_velocity(dt, 0, accelerations, velocities);
  compute_velocity(dt, 1, accelerations, velocities);
  compute_velocity(dt, 2, accelerations, velocities);

  Vector expected[size] = { { 5.3293525d, 2.1635875d }, { -6.18578d, 10.013031d }, { 194.6503031031d, 1296.158628031d } };
  for (size_t i = 0; i < size; ++i) {
    for_assert(velocities[i].x_component, expected[i].x_component, "test_compute_velocity_multiple_elements - x_component", i);
    for_assert(velocities[i].y_component, expected[i].y_component, "test_compute_velocity_multiple_elements - y_component", i);
  }
  #undef size
}

/**
 * Checks that the displace_particles function works for arrays of one element.
 */
void test_displace_particles_one_element() {
  #define size 1
  Particle particles[size] = { { 0, 100, 0, NULL, 0 } };
  Vector displacements[size] = { { 0.05, -0.05 } };

  displace_particle(0, displacements, particles);

  assert(particles[0].x_coordinate, 50.0d, "test_displace_particles_one_element - x_coordinate");
  assert(particles[0].y_coordinate, 50.0d, "test_displace_particles_one_element - y_coordinate");
  #undef size
}

/**
 * Checks that the displace_particles function works for arrays of multiple elements.
 */
void test_displace_particles_multiple_elements() {
  #define size 3
  Particle particles[size] = { { 0, 100, 0, NULL, 0 }, { 111, 210, 0, NULL, 1 }, { 10, -30, 0, NULL, 2 } };
  Vector displacements[size] = { { 0, 0 }, { 0.015, -0.033 }, { -0.015, 0.03 } };

  displace_particle(0, displacements, particles);
  displace_particle(1, displacements, particles);
  displace_particle(2, displacements, particles);

  Particle expected[size] = { { 0.0d, 100.0d, 0, NULL, 0 }, { 126, 177, 0, NULL, 1 }, { -5.0d, 0.0d, 0, NULL, 2 } };
  for (size_t i = 0; i < size; ++i) {
    for_assert(particles[i].x_coordinate, expected[i].x_coordinate, "test_displace_particles_multiple_elements - x_coordinate", i);
    for_assert(particles[i].y_coordinate, expected[i].y_coordinate, "test_displace_particles_multiple_elements - y_coordinate", i);
  }
  #undef size
}

/**
 * Tests entry point.
 * All tests run here.
 */
int main(void) {
  // Initialize the number of failed tests.
  number_failed = 0;

  // Execute all tests.
  //test_compute_forces_one_contact(); COMMENTED BECAUSE OF THE NEW COLLISION DETECTION MODULE. We no longer collide p1 with p2 and p2 with p1, but rather we found both collisions separetaly, and generate twice the number of contacts, so comptue_forces only computes  p1 with p2, and on another function call p2 with p1
  //test_compute_forces_multiple_contacts();
  test_compute_acceleration_one_element();
  test_compute_acceleration_multiple_elements();
  test_compute_velocity_one_element();
  test_compute_velocity_multiple_elements();
  test_displace_particles_one_element();
  test_displace_particles_multiple_elements();

  // If, at least one test failed, exit with an error code.
  return (number_failed == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}
