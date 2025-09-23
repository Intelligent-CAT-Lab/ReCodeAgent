#ifndef HASH_H
#define HASH_H

#define RANDOM_SIZE 16

void generate_random_data(char *buffer, int length);
void generate_hash(const char *data, int length, unsigned char *hash);
#endif 
