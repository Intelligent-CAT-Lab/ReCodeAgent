
#include <stdbool.h>

/* always called once called on app start up */
extern void dht_init();

extern bool dht_is_initialised();
/* called whenever the dht is reorganised. expanded, shrunk etc...*/
extern bool dht_init_table(unsigned int new_lower_bound, unsigned int new_upper_bound, bool migrate);

extern unsigned int dht_get_lower_bound();
extern unsigned int dht_get_upper_bound();
extern unsigned int dht_get_size();

/* use absolute positions. */
extern void *dht_read(unsigned int location);
extern void dht_write(unsigned int location, void* data);
