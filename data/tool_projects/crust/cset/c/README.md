# Typesafe & Fast as fuck Set in C
![](https://i.imgur.com/4JE58Ew.png)

### Key Features
* Extremely fast non-cryptographic hash algorithm [XXHash](https://cyan4973.github.io/xxHash/)
* Complete Typesafe APIs
* **Double Hashing** to avoid both Primary & Secondary clustering
* **Robinhood Hash**(*in progress*) for near constant time access
* Iterators for streaming usecase
* Allows **Custom Hash** & **Custom Comparator**
* Single header file to rule them all 🚀 🚀 🚀

Go to this [blogpost](https://robus.dev/typesafe-and-fast-as-fk-set-in-c) for more information and motivation for writing Set from scratch.

### Getting Started
```c
#include <stdio.h>
#include "cset.h"

// Declare Cset of int type
Cset(int) cset_int_t;
// Declare Cset iterator of `cset_int_t` type
Cset_iterator(cset_int_t) cset_iterator_int_t;

int main() {
    // Instance of cset_int_t
    cset_int_t cset_int;

    // Initialize
    cset__init(&cset_int);

    // Add elements
    cset__add(&cset_int, 1);
    cset__add(&cset_int, 2);
    cset__add(&cset_int, 2); // Duplicate

    size_t size = cset__size(&cset_int);
    printf("Size of set is: %ld\n", size);

    // Iteration
    cset_iterator_int_t cset_iterator_int;

    // Initialize iterator with cset_int
    cset_iterator__init(&cset_iterator_int, &cset_int);

    // Traverse
    for(;;) {
      if (cset_iterator__done(&cset_iterator_int)) break;

      int* buffer;
      cset_iterator__next(&cset_iterator_int, buffer);

      printf("Got: %d\n", *buffer);
    }
}
```

###
### Installation
Simply copy paste `cset.h` in your source directly or you can use [Clib](https://github.com/clibs/clib) package manager for installation
```sh
clib install robusgauli/cset
```

### APIs
* `cset__add(cset, value)`: *no return*
Add element to set.
    ```c
    #include "cset.h"

    Cset(char) cset_char_t;

    int main() {
      cset_char_t cset_char;
      cset__init(&cset_char);

      // Add elements to set
      cset__add(&cset_char, 'a');
      cset__add(&cset_char, 'b');
    }

    cset__free(&cset_int);
    ```

* `cset__remove(cset, value)`: *no return*
Removes element from the set.

    ```c
    #include <assert.h>
    #include "cset.h"

    Cset(int) cset_int_t;

    int main() {
      cset_int_t cset_int;
      cset__init(&cset_int);

      cset__add(&cset_int, 34);
      assert(cset__size(&cset_int) == 1);

      // Remove element from the set
      cset__remove(&cset_int, 34);
      assert(cset__size(&cset_int) == 0);

      cset__free(&cset_int);
    }
    ```
* `cset__size(cset)`: *returns `size_t`*
Returns the size of set.
    ```c
    #include <assert.h>
    #include "cset.h"

    Cset(int) cset_int_t;

    int main() {
      cset_int_t cset_int;
      cset__init(&cset_int);

      cset__add(&cset_int, 34);
      cset__add(&cset_int, 25);
      cset__add(&cset_int, 56);

      size_t size = cset__size(&cset_int);
      printf("Size: %ld\n", size);

      cset__free(&cset_int);
    }
    ```
* `cset__clear(cset)`: *no return*
Removes all the elements from the set.
    ```c
    #include <assert.h>
    #include "cset.h"

    Cset(int) cset_int_t;

    int main() {
      cset_int_t cset_int;
      cset__init(&cset_int);

      cset__add(&cset_int, 34);
      cset__add(&cset_int, 25);
      cset__add(&cset_int, 56);

      // Clear the set
      cset__clear(&cset_int);
      assert(cset__size(&cset_int) == 0);

      cset__free(&cset_int);
    }
    ```

* `cset__contains(cset, value, flag)`: *no return*
Test whether an element is in set.
    ```c
    #include <assert.h>
    #include "cset.h"

    Cset(int) cset_int_t;

    int main() {
      cset_int_t cset_int;
      cset__init(&cset_int);

      cset__add(&cset_int, 34);
      cset__add(&cset_int, 25);
      cset__add(&cset_int, 56);

      // Positive membership test
      bool contains = false;
      cset__contains(&cset_int, 34, &contains);
      assert(contains == true);

      // Negative membershi test
      cset__contains(&cset_int, 100, &contains);
      assert(contains == false);

      cset__free(&cset_int);
    }
    ```
* `cset__set_comparator` & `cset__set_hash()`: *no return*

   **NOTE:** Both **Custom Comparator and Hash** must be implemented.

    These above methods allows us to use custom hashing and comparator for complex structs. Please go through this [link](https://stackoverflow.com/questions/2265503/why-do-i-need-to-override-the-equals-and-hashcode-methods-in-java) for why we need to implement both functions for correctness.

    ```c
    #include <assert.h>
    #include "cset.h"

    typedef struct {
      int x;
      int y;
    } Node;

    // Custom comparator
    bool custom_comparator(Node* self, Node* other) {
      return (self -> x) == (other -> x);
    }
    // Custom hash function
    // It takes pointer to Node and function as an arguments
    cset__u64 custom_hash(Node* self, cset__u64 (*hash)(void*, size_t)) {
      // VVIP: Hash function requires pointer to data and the size in bytes
      return hash(&(self -> x), sizeof(self -> x));
    }

    Cset(Node) cset_node_t;

    int main() {
      cset_node_t cset_node;
      cset__init(&cset_node);
      cset__set_hash(&cset_node, custom_hash);
      cset__set_comparator(&cset_node, custom_comparator);

      cset__add(&cset_node, ((Node){.x=4,.y=5}));
      // Duplicate because our custom comparator and hash uses `x`
      // instead of whole struct for hash
      cset__add(&cset_node, ((Node){.x=4,.y=4}));

      assert(cset__size(&cset_node) == 1);

      cset__add(&cset_node, ((Node){.x=1, .y=2}));
      assert(cset__size(&cset_node) == 2);

      // Removed because our comparator uses `x` i.e 1 which already exists
      cset__remove(&cset_node, ((Node){.x=1, .y=45}));
      assert(cset__size(&cset_node) == 1);

      cset__free(&cset_node);
    }
    ```

* `cset__free`: *no return*
Frees up heap allocation

    ```c
    #include "cset.h"

    Cset(int) cset_int_t;

    int main() {
      cset_int_t cset_int;
      cset__init(&cset_int);

      cset__add(&cset_int, 34);

      // Free up the memory
      cset__free(&cset_int);
    }
    ```

### APIS for Iteration

* `Cset_iterator(type)`
    Creates Iterator type definition for a given cset type.

* `cset_iterator__init(iter, cset)`: *no return*
    Initializes iterator `iter` with `cset`.
  ```c
  #include "cset.h"

  Cset(int) cset_int_t;
  // 1. Define iterator type for `cset_int_t`
  Cset_iterator(cset_int_t) cset_iterator_int_t;

  int main() {
    cset_int_t cset_int;
    cset__init(&cset_int);

    cset_iterator_int_t cset_int_iterator;
    // 2. Initialize iterator with `cset_int`
    cset_iterator__init(&cset_int_iterator, &cset_int);
  }
  ```

* `cset__done(iter)`: *returns `bool`*
    Returns whether the iteration is complete or not.
  ```c
  #include <assert.h>
  #include "cset.h"

  Cset(int) cset_int_t;
  Cset_iterator(cset_int_t) cset_iterator_int_t;

  int main() {
    cset_int_t cset_int;
    cset__init(&cset_int);

    cset__add(&cset_int, 34);
    cset__add(&cset_int, 56);

    cset_iterator_int_t cset_int_iterator;
    cset_iterator__init(&cset_int_iterator, &cset_int);

    // Checks whether the iteration is complete or not
    bool done = cset_iterator__done(&cset_int_iterator);
    assert(done == false);
  }
  ```

* `cset__next(iter, buffer)`: *no return*

    Yields pointer to next element into buffer. Note: buffer must be pointer type.

  ```c
  #include <assert.h>
  #include "cset.h"


  Cset(int) cset_int_t;
  Cset_iterator(cset_int_t) cset_iterator_int_t;

  int main() {
    cset_int_t cset_int;
    cset__init(&cset_int);

    cset__add(&cset_int, 34);
    cset__add(&cset_int, 56);

    cset_iterator_int_t cset_int_iterator;
    cset_iterator__init(&cset_int_iterator, &cset_int);

    for (;;) {
      if (cset_iterator__done(&cset_int_iterator)) break;

      // Yields pointer to next element into buffer
      int* buffer;
      cset_iterator__next(&cset_int_iterator, buffer);

      printf("Got: %d\n", *buffer);
    }

    cset__free(&cset_int);
  }
  ```

### APIS for basic set operations

* `cset__intersection(result, a, b)`: *no return*
Puts the intersection of two sets `a` and `b` in `result`.

  ```c
  #include <assert.h>
  #include "cset.h"

  Cset(int) cset_int_t;

  int main() {
    cset_int_t cset_int_a;
    cset_int_t cset_int_b;

    cset__init(&cset_int_a);
    cset__init(&cset_int_b);

    // Add elements to cset_int_a
    cset__add(&cset_int_a, 12);
    cset__add(&cset_int_a, 13);
    cset__add(&cset_int_a, 14);

    // Add elements to cset_int_b
    cset__add(&cset_int_b, 12);
    cset__add(&cset_int_b, 13);
    cset__add(&cset_int_b, 16);

    cset_int_t result;
    cset__init(&result);

    // Perform intersection between cset_int_a and cset_int_b
    cset__intersect(&result, &cset_int_a, &cset_int_b);

    assert(cset__size(&result) == 2);

    cset__free(&cset_int_a);
    cset__free(&cset_int_b);
    cset__free(&result);
  }
  ```

* `cset__union(result, a, b)`: *no return*
Puts all the elements that are in either set into `result`

  ```c
  #include <assert.h>
  #include "cset.h"

  Cset(int) cset_int_t;
  Cset_iterator(cset_int_t) cset_iterator_int_t;

  int main() {
    cset_int_t cset_int_a;
    cset_int_t cset_int_b;
    cset_int_t cset_result;

    cset__init(&cset_int_a);
    cset__init(&cset_int_b);
    cset__init(&cset_result);

    cset__add(&cset_int_a, 34);
    cset__add(&cset_int_a, 25);
    cset__add(&cset_int_a, 12);

    cset__add(&cset_int_b, 1);
    cset__add(&cset_int_b, 4);
    cset__add(&cset_int_b, 34);

    // Perform union between `cset_int_a` and `cset_int_b`
    cset__union(&cset_result, &cset_int_a, &cset_int_b);

    // Size of the `cset_result` set must be 5 with elements { 34, 25, 12, 1, 4 }
    assert(cset__size(&cset_result) == 5);

    cset__add(&cset_int_b, 100);
    cset__union(&cset_result, &cset_int_a, &cset_int_b);
    // Size of the `cset_result` set must be 6 as we added new elements to `cset_int_b`
    assert(cset__size(&cset_result) == 6);

    cset__free(&cset_int_a);
    cset__free(&cset_int_b);
    cset__free(&cset_result);
  }
  ```

### License

Copyright © 2020-20121 Robus, LLC. This source code is licensed under the MIT license found in
the [LICENSE.txt]
The documentation to the project is licensed under the [CC BY-SA 4.0](http://creativecommons.org/licenses/by-sa/4.0/)
license.


---
Made with ♥ by Robus Gauli ([@robusgauli]
