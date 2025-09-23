
#define RECT_PACK_H_IMPL
#include "rect_pack.h"

#include <stdio.h>

int main() {

    rect_r rects[100];

    for(int i = 0; i < sizeof(rects)/sizeof(rect_r); i++) {
        rects[i].id = i;
        rects[i].w = i;
        rects[i].h = i;
    }

    if(rect_pack(512, 512, true, rects, sizeof(rects)/sizeof(rect_r))) {
        printf("packed\n");
    } else {
        printf("unable to pack\n");
    }

    return 0;
}