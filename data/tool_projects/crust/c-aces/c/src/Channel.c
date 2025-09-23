#include "include/Channel.h"
#include "include/Common.h"

#include <math.h>

int init_channel(Channel *channel, uint64_t p, uint64_t q, uint64_t w) {
  channel->w = w;
  channel->p = p;
  channel->q = q;

  if (!(pow(p, 2) < q && are_coprime(p, q)))
    channel->q = (uint64_t)pow(p + 1, 2);

  return 0;
}
