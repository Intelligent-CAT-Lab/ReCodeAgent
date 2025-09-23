# libcborg-bitset

## Build and Install

```sh
$ git clone https://github.com/abenhlal/bitset.git
$ cd bitset
$ mkdir build
$ cd build
$ cmake ..
$ make
$ make test
$ sudo make install
```

## Uninstall

```sh
$ sudo make uninstall
```

## Usage and build

```sh
$ gcc example.c -o example.bin -L/usr/local/lib/cborg -lcborg-bitset
$ ./example.bin
bs: 1111011111
bs_string: 1111011111
```

```C
#include <stdio.h>
#include <cborg/bitset.h>

int main() {
  // bitset : 0000000000
  bitset_t *bs = cb_bitset_new(10);

  // return 0
  cb_bitset_count(bs);

  // return 10
  cb_bitset_size(bs);

  // bitset: 1000000000
  cb_bitset_set(bs, 0, true);
  
  // return true
  cb_bitset_test(bs,0);

  // return false
  cb_bitset_test(bs,1);

  // return true because bs have bit set 
  cb_bitset_any(bs);

  // return false because bs have bit set
  cb_bitset_none(bs);

  // return false 
  cb_bitset_all(bs);
  
  // bitset: 0111111111
  cb_bitset_flip(bs);

  // bitset: 0000000000
  cb_bitset_reset(bs);
  
  // bitset: 1111111111
  cb_bitset_flip(bs);

  // bitset: 1111011111
  cb_bitset_set(bs, 4, false);

  // write bitset to file
  cb_bitset_write(bs, "./bitset.bs");

  // read bitset from file
  bitset_t *bs_read = cb_bitset_read("./bitset.bs");

  char *bs_string = cb_bitset_to_string(bs);
  char *bs_read_string = cb_bitset_to_string(bs_read);
  
  // printf
  // bs: 1111011111
  // bs_read: 1111011111
  printf("bs: %s\n",bs_string);
  printf("bs_read: %s\n",bs_read_string);

  // free
  free(bs_string);
  cb_bitset_delete(bs);
  free(bs_read_string);
  cb_bitset_delete(bs_read);
  return 0;
}

```

## License

[MIT](LICENSE)