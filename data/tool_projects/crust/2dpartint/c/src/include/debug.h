#pragma once
#include <fstream>

/**
 * Writes the current step/state of the system - all the data structures that represent a particle -
 * to a file in a specified folder.
 */
void write_debug_information(const unsigned long step, const size_t particle_index,
                             const size_t contacts_size, const char *debug_folder);

/**
 * Writes a header in given file. The header contains the columns that represent
 * each attribute of each simulation data structure.
 */
void write_header(const unsigned long step, const size_t particle_index,
                  std::ofstream &file);

/**
 * Writes the value of each data structure to a file.
 */
void write_values(const size_t particle_index, const size_t contacts_size, std::ofstream &file);
