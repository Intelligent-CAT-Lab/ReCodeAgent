#pragma once

/**
 * Represents the parsed config file.
 */
typedef struct {
  double simulation_time;
  double dt;
  int x_particles;
  int y_particles;
  int x_squares;
  int y_squares;
  double square_in_grid_length;
  double radius;
  double kn;
  double ks;
  double rho;
  double thickness;
  double v0;
  double r0;
} Config;

/**
 * Parses the provided config file,
 * and stores the results in the provided structure.
 */
void parse_config(const char *filename, Config *config);
