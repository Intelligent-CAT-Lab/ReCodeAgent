# `rect_pack.h`

rectangle packer (for simple texture atlas building) based on the bin tree method. 

## building

simply drop the `rect_pack.h` file into your project and define the `RECT_PACK_H_IMPL` before including the header in one file to include its implementation. If you don't want to manually add the header you can include it in your project using CMake with something like `FetchContent`. 

## usage

```c
// just define this macro once to include 
// the implementation
#define RECT_PACK_H_IMPL
#include "rect_pack.h"

int main() {

    rect_r rects[100];

    // give some random values to the rects
    for(int i = 0; i < 100; i++) {
        rects[i].id = i; // id is an identifier for you
        rects[i].w = i;
        rects[i].h = i;
    }

    int max_w = 512;
    int max_h = 512;
    bool paging = true;

    if(rect_pack(max_w, max_h, paging, rects, 100)) {
        printf("all rects packed\n");
    } else {
        printf("unable to pack\n");
    }

    return 0;
}

```

## test and examples

tests and examples are found in the `test` folder, and they can be built and run setting the `BUILD_TESTS` option in CMake.

