#include <math.h>
#include <stdlib.h>
#include <string.h> // For memset.
#include "data.h"
#include "functions.h"

// tan((30 * PI) / 180).
#define TAN_30_PI_180 0.5773502691896257

/**
 * Computes the distance between two particles.
 */
inline double compute_distance(const Particle *p1, const Particle *p2) {
  const double x_diff = p1->x_coordinate - p2->x_coordinate;
  const double y_diff = p1->y_coordinate - p2->y_coordinate;

  return sqrt((x_diff * x_diff) + (y_diff * y_diff));
}

/**
 * Computes the overlap between two particles.
 * Note: If the overlap is negative, there is no overlap.
 */
inline double compute_overlap(const Particle *p1, const Particle *p2) {
  const double d = p1->radius + p2->radius;
  const double distance = compute_distance(p1, p2);

  return d - distance;
}

/**
 * Applies the forces to the particles with the same index,
 * and computes the resultant acceleration.
 */
inline void compute_acceleration(const size_t particle_index,
                                 const ParticleProperties *particles_properties,
                                 const Vector *forces, Vector *accelerations) {
  accelerations[particle_index].x_component =
    forces[particle_index].x_component / particles_properties[particle_index].mass;
  accelerations[particle_index].y_component =
    forces[particle_index].y_component / particles_properties[particle_index].mass;
}

/**
 * Apply gravity to a vector of forces.
 */
inline void apply_gravity(const size_t size,
                          const ParticleProperties *particles_properties,
                          Vector *forces) {
  for (size_t i = 0; i < size; ++i) {
    forces[i].y_component -= (particles_properties[i].mass * 9.81d);
  }
}

/**
 * Compute the forces applied to P2 given it was collided by P1.
 * Note: previous_normal and previous_tangent correspond to P2 with respect to P1.
 */
void collide_two_particles(const double dt, const double distance,
                           const Particle *p1, const Particle *p2,
                           const Vector *velocity_p1, const Vector *velocity_p2,
                           const ParticleProperties *properties_p2,
                           double *previous_normal, double *previous_tangent,
                           Vector *force_p2) {
  const Vector normal = {
    .x_component = (p1->x_coordinate - p2->x_coordinate) / distance,
    .y_component = (p1->y_coordinate - p2->y_coordinate) / distance
  };

  const double velocity_x_diff = velocity_p2->x_component - velocity_p1->x_component;
  const double velocity_y_diff = velocity_p2->y_component - velocity_p1->y_component;
  const double normal_velocity = (normal.x_component * velocity_x_diff)
    + (normal.y_component * velocity_y_diff);
  const double tangent_velocity = (normal.y_component * velocity_x_diff)
    - (normal.x_component * velocity_y_diff);

  const double dfn = normal_velocity * properties_p2->kn * dt;
  const double dfs = tangent_velocity * properties_p2->ks * dt;

  // Forces for P2 with respect to P1.
  double Fn_1_2 = *previous_normal + dfn;
  double Fs_1_2 = *previous_tangent + dfs;

  if (Fn_1_2 < 0) {
    Fn_1_2 = 0;
    Fs_1_2 = 0;
  }

  const double Fs_1_2_max = Fn_1_2 * TAN_30_PI_180;
  if (fabs(Fs_1_2) > Fs_1_2_max) {
    Fs_1_2 = (fabs(Fs_1_2_max) * fabs(Fs_1_2)) / Fs_1_2;
  }

  // Update the forces of p2.
  force_p2->x_component += (-normal.x_component * Fn_1_2) - (normal.y_component * Fs_1_2);
  force_p2->y_component += (-normal.y_component * Fn_1_2) + (normal.x_component * Fs_1_2);

  // Update the normal and tangent forces between p1 and p2 for the next simulation step.
  *previous_normal = Fn_1_2;
  *previous_tangent = Fs_1_2;
}

/**
 * Computes the resulting forces each particle.
 */
inline void compute_forces(const double dt, const size_t particles_size,
                           const size_t contacts_size, const Particle *particles,
                           const ParticleProperties *properties, const Contact *contacts,
                           const Vector *velocities, double *normal_forces,
                           double *tangent_forces, Vector *forces) {

  for (size_t i = 0; i < contacts_size; ++i) {
    const size_t p1_idx = contacts[i].p1_idx;
    const size_t p2_idx = contacts[i].p2_idx;
    const size_t p2_p1_idx = (p1_idx * particles_size) + p2_idx;
    const Particle *p1 = &particles[p1_idx];
    const Particle *p2 = &particles[p2_idx];
    const double distance = compute_distance(p1, p2);

    // P1 collides P2.
    collide_two_particles(
      dt,
      distance,
      p1,
      p2,
      &velocities[p1_idx],
      &velocities[p2_idx],
      &properties[p2_idx],
      &normal_forces[p2_p1_idx],
      &tangent_forces[p2_p1_idx],
      &forces[p2_idx]
    );
  }
  apply_gravity(particles_size, properties, forces);
}

/**
 * Derives the resultant velocity,
 * of an initial velocity with an applied acceleration for given a time delta.
 */
inline void compute_velocity(const double dt, const size_t particle_index,
                             const Vector *accelerations, Vector *velocities) {
  velocities[particle_index].x_component =
    velocities[particle_index].x_component + accelerations[particle_index].x_component * dt;
  velocities[particle_index].y_component =
    velocities[particle_index].y_component + accelerations[particle_index].y_component * dt;
}

/**
 * Computes the displacement of the particles,
 * with an applied velocity for a given time delta.
 */
inline void compute_displacement(const double dt, const size_t particle_index,
                                 const Vector *velocities, Vector *displacements) {
  displacements[particle_index].x_component =
    displacements[particle_index].x_component + velocities[particle_index].x_component * dt;
  displacements[particle_index].y_component =
    displacements[particle_index].y_component + velocities[particle_index].y_component * dt;
}

/**
 * Displace all particles given their displacements.
 */
inline void displace_particle(const size_t particle_index, const Vector *displacements,
                              Particle *particles) {
  particles[particle_index].x_coordinate += (displacements[particle_index].x_component * 1000);
  particles[particle_index].y_coordinate += (displacements[particle_index].y_component * 1000);
}

/**
  * Changes the displacement if the new position would surpass the Y limit.
  */
inline void fix_displacement(const size_t particle_index, Vector *velocities, Particle *particles) {

  double diff = particles[particle_index].y_coordinate - particles[particle_index].radius;
  if (diff < 0) {
    particles[particle_index].y_coordinate = particles[particle_index].radius;
    velocities[particle_index].y_component = 0;
  }
}
