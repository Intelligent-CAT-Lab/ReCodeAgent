#include <unity.h>

#define RECT_PACK_H_IMPL
#include "rect_pack.h"

void setUp(void) {}    /* Is run before every test, put unit init calls here. */
void tearDown(void) {} /* Is run after every test, put unit clean-up calls here. */

void test_rect_pack_uniform_paging() {
    rect_r rects[100];

    for (int i = 0; i < sizeof(rects) / sizeof(rect_r); i++) {
        rects[i].id = i;
        rects[i].w = i + 1;
        rects[i].h = i + 1;
    }

    TEST_ASSERT_TRUE(rect_pack(512, 512, true, rects, sizeof(rects) / sizeof(rect_r)));
}

void test_rect_pack_one_rect_per_page_paging() {
    rect_r rects[] = {
        {.id = 0, .w = 6, .h = 7},
        {.id = 1, .w = 8, .h = 7},
        {.id = 2, .w = 8, .h = 9},
        {.id = 3, .w = 10, .h = 9},
    };

    int n = sizeof(rects) / sizeof(rect_r);

    TEST_ASSERT_TRUE(rect_pack(10, 10, true, rects, n));

    for (int i = 0; i < n; i++) {
        TEST_ASSERT_TRUE(rects[i].info.packed);
        TEST_ASSERT_EQUAL(rects[i].info.page, i);
    }
}

void test_rect_pack_fill() {
    rect_r rects[100];

    for (int i = 0; i < sizeof(rects) / sizeof(rect_r); i++) {
        rects[i].id = i;
        rects[i].w = 1;
        rects[i].h = 1;
    }

    TEST_ASSERT_TRUE(rect_pack(10, 10, false, rects, sizeof(rects) / sizeof(rect_r)));

    for (int i = 0; i < sizeof(rects) / sizeof(rect_r); i++) {
        TEST_ASSERT_TRUE(rects[i].info.packed);
        TEST_ASSERT_EQUAL(rects[i].info.page, 0);
    }
}

void test_rect_pack_no_paging() {
    rect_r rects[] = {
        {.id = 0, .w = 6, .h = 7},
        {.id = 1, .w = 8, .h = 7},
        {.id = 2, .w = 8, .h = 9},
        {.id = 3, .w = 10, .h = 9},
    };

    TEST_ASSERT_FALSE(rect_pack(10, 10, false, rects, sizeof(rects) / sizeof(rect_r)));

    TEST_ASSERT_TRUE(rects[0].info.packed);
    TEST_ASSERT_FALSE(rects[1].info.packed);
    TEST_ASSERT_FALSE(rects[2].info.packed);
    TEST_ASSERT_FALSE(rects[3].info.packed);
}

void test_rect_pack_fail_paging() {
    rect_r rects[2];

    rects[0].id = 0;
    rects[0].w = 900;
    rects[0].h = 900;

    rects[1].id = 1;
    rects[1].w = 1100;
    rects[1].h = 1100;

    TEST_ASSERT_FALSE(rect_pack(1000, 1000, true, rects, sizeof(rects) / sizeof(rect_r)));
    TEST_ASSERT_FALSE(rects[0].info.packed);
    TEST_ASSERT_TRUE(rects[1].info.packed);
}

void test_rect_pack_loop() {
    const int nrects = 199;
    static rect_r rects[nrects] = {
        {.id = 0, .w = 255, .h = 255},   {.id = 1, .w = 255, .h = 253},
        {.id = 2, .w = 253, .h = 255},   {.id = 3, .w = 255, .h = 253},
        {.id = 4, .w = 255, .h = 253},   {.id = 5, .w = 253, .h = 255},
        {.id = 6, .w = 253, .h = 255},   {.id = 7, .w = 253, .h = 255},
        {.id = 8, .w = 255, .h = 251},   {.id = 9, .w = 255, .h = 251},
        {.id = 10, .w = 251, .h = 255},  {.id = 11, .w = 255, .h = 251},
        {.id = 12, .w = 255, .h = 251},  {.id = 13, .w = 251, .h = 255},
        {.id = 14, .w = 255, .h = 251},  {.id = 15, .w = 255, .h = 251},
        {.id = 16, .w = 255, .h = 249},  {.id = 17, .w = 255, .h = 249},
        {.id = 18, .w = 255, .h = 249},  {.id = 19, .w = 255, .h = 249},
        {.id = 20, .w = 255, .h = 249},  {.id = 21, .w = 249, .h = 255},
        {.id = 22, .w = 255, .h = 249},  {.id = 23, .w = 249, .h = 255},
        {.id = 24, .w = 249, .h = 255},  {.id = 25, .w = 247, .h = 255},
        {.id = 26, .w = 255, .h = 247},  {.id = 27, .w = 255, .h = 247},
        {.id = 28, .w = 255, .h = 245},  {.id = 29, .w = 245, .h = 255},
        {.id = 30, .w = 255, .h = 245},  {.id = 31, .w = 255, .h = 245},
        {.id = 32, .w = 255, .h = 243},  {.id = 33, .w = 243, .h = 255},
        {.id = 34, .w = 243, .h = 255},  {.id = 35, .w = 255, .h = 243},
        {.id = 36, .w = 243, .h = 255},  {.id = 37, .w = 243, .h = 255},
        {.id = 38, .w = 255, .h = 243},  {.id = 39, .w = 255, .h = 243},
        {.id = 40, .w = 255, .h = 241},  {.id = 41, .w = 255, .h = 241},
        {.id = 42, .w = 241, .h = 255},  {.id = 43, .w = 241, .h = 255},
        {.id = 44, .w = 255, .h = 241},  {.id = 45, .w = 241, .h = 255},
        {.id = 46, .w = 255, .h = 241},  {.id = 47, .w = 255, .h = 239},
        {.id = 48, .w = 239, .h = 255},  {.id = 49, .w = 239, .h = 255},
        {.id = 50, .w = 239, .h = 255},  {.id = 51, .w = 255, .h = 239},
        {.id = 52, .w = 239, .h = 255},  {.id = 53, .w = 237, .h = 255},
        {.id = 54, .w = 255, .h = 237},  {.id = 55, .w = 237, .h = 255},
        {.id = 56, .w = 237, .h = 255},  {.id = 57, .w = 255, .h = 237},
        {.id = 58, .w = 255, .h = 237},  {.id = 59, .w = 255, .h = 237},
        {.id = 60, .w = 237, .h = 255},  {.id = 61, .w = 237, .h = 255},
        {.id = 62, .w = 255, .h = 237},  {.id = 63, .w = 255, .h = 235},
        {.id = 64, .w = 235, .h = 255},  {.id = 65, .w = 235, .h = 255},
        {.id = 66, .w = 255, .h = 235},  {.id = 67, .w = 255, .h = 233},
        {.id = 68, .w = 233, .h = 255},  {.id = 69, .w = 255, .h = 233},
        {.id = 70, .w = 233, .h = 255},  {.id = 71, .w = 255, .h = 233},
        {.id = 72, .w = 233, .h = 255},  {.id = 73, .w = 233, .h = 255},
        {.id = 74, .w = 255, .h = 233},  {.id = 75, .w = 231, .h = 255},
        {.id = 76, .w = 253, .h = 253},  {.id = 77, .w = 253, .h = 253},
        {.id = 78, .w = 253, .h = 253},  {.id = 79, .w = 251, .h = 253},
        {.id = 80, .w = 253, .h = 251},  {.id = 81, .w = 253, .h = 251},
        {.id = 82, .w = 251, .h = 253},  {.id = 83, .w = 251, .h = 253},
        {.id = 84, .w = 253, .h = 249},  {.id = 85, .w = 253, .h = 249},
        {.id = 86, .w = 249, .h = 253},  {.id = 87, .w = 249, .h = 253},
        {.id = 88, .w = 249, .h = 253},  {.id = 89, .w = 249, .h = 253},
        {.id = 90, .w = 253, .h = 249},  {.id = 91, .w = 253, .h = 249},
        {.id = 92, .w = 253, .h = 247},  {.id = 93, .w = 247, .h = 253},
        {.id = 94, .w = 247, .h = 253},  {.id = 95, .w = 253, .h = 247},
        {.id = 96, .w = 253, .h = 247},  {.id = 97, .w = 253, .h = 247},
        {.id = 98, .w = 253, .h = 247},  {.id = 99, .w = 253, .h = 245},
        {.id = 100, .w = 253, .h = 245}, {.id = 101, .w = 245, .h = 253},
        {.id = 102, .w = 245, .h = 253}, {.id = 103, .w = 253, .h = 243},
        {.id = 104, .w = 243, .h = 253}, {.id = 105, .w = 243, .h = 253},
        {.id = 106, .w = 253, .h = 243}, {.id = 107, .w = 253, .h = 243},
        {.id = 108, .w = 253, .h = 243}, {.id = 109, .w = 243, .h = 253},
        {.id = 110, .w = 243, .h = 253}, {.id = 111, .w = 241, .h = 253},
        {.id = 112, .w = 253, .h = 241}, {.id = 113, .w = 253, .h = 241},
        {.id = 114, .w = 253, .h = 241}, {.id = 115, .w = 241, .h = 253},
        {.id = 116, .w = 239, .h = 253}, {.id = 117, .w = 253, .h = 239},
        {.id = 118, .w = 239, .h = 253}, {.id = 119, .w = 253, .h = 239},
        {.id = 120, .w = 239, .h = 253}, {.id = 121, .w = 253, .h = 239},
        {.id = 122, .w = 253, .h = 239}, {.id = 123, .w = 239, .h = 253},
        {.id = 124, .w = 239, .h = 253}, {.id = 125, .w = 237, .h = 253},
        {.id = 126, .w = 237, .h = 253}, {.id = 127, .w = 253, .h = 237},
        {.id = 128, .w = 237, .h = 253}, {.id = 129, .w = 253, .h = 237},
        {.id = 130, .w = 237, .h = 253}, {.id = 131, .w = 235, .h = 253},
        {.id = 132, .w = 235, .h = 253}, {.id = 133, .w = 235, .h = 253},
        {.id = 134, .w = 233, .h = 253}, {.id = 135, .w = 253, .h = 233},
        {.id = 136, .w = 233, .h = 253}, {.id = 137, .w = 233, .h = 253},
        {.id = 138, .w = 253, .h = 233}, {.id = 139, .w = 233, .h = 253},
        {.id = 140, .w = 253, .h = 233}, {.id = 141, .w = 233, .h = 253},
        {.id = 142, .w = 253, .h = 233}, {.id = 143, .w = 233, .h = 253},
        {.id = 144, .w = 253, .h = 233}, {.id = 145, .w = 253, .h = 233},
        {.id = 146, .w = 233, .h = 253}, {.id = 147, .w = 231, .h = 253},
        {.id = 148, .w = 253, .h = 231}, {.id = 149, .w = 253, .h = 231},
        {.id = 150, .w = 229, .h = 253}, {.id = 151, .w = 229, .h = 253},
        {.id = 152, .w = 229, .h = 253}, {.id = 153, .w = 229, .h = 253},
        {.id = 154, .w = 251, .h = 251}, {.id = 155, .w = 251, .h = 251},
        {.id = 156, .w = 251, .h = 251}, {.id = 157, .w = 251, .h = 249},
        {.id = 158, .w = 249, .h = 251}, {.id = 159, .w = 249, .h = 251},
        {.id = 160, .w = 249, .h = 251}, {.id = 161, .w = 247, .h = 251},
        {.id = 162, .w = 251, .h = 247}, {.id = 163, .w = 251, .h = 247},
        {.id = 164, .w = 247, .h = 251}, {.id = 165, .w = 247, .h = 251},
        {.id = 166, .w = 247, .h = 251}, {.id = 167, .w = 251, .h = 247},
        {.id = 168, .w = 247, .h = 251}, {.id = 169, .w = 251, .h = 247},
        {.id = 170, .w = 247, .h = 251}, {.id = 171, .w = 251, .h = 247},
        {.id = 172, .w = 251, .h = 245}, {.id = 173, .w = 245, .h = 251},
        {.id = 174, .w = 251, .h = 245}, {.id = 175, .w = 251, .h = 245},
        {.id = 176, .w = 245, .h = 251}, {.id = 177, .w = 251, .h = 245},
        {.id = 178, .w = 251, .h = 243}, {.id = 179, .w = 243, .h = 251},
        {.id = 180, .w = 243, .h = 251}, {.id = 181, .w = 243, .h = 251},
        {.id = 182, .w = 243, .h = 251}, {.id = 183, .w = 243, .h = 251},
        {.id = 184, .w = 251, .h = 243}, {.id = 185, .w = 251, .h = 243},
        {.id = 186, .w = 241, .h = 251}, {.id = 187, .w = 251, .h = 241},
        {.id = 188, .w = 241, .h = 251}, {.id = 189, .w = 251, .h = 241},
        {.id = 190, .w = 241, .h = 251}, {.id = 191, .w = 251, .h = 241},
        {.id = 192, .w = 251, .h = 239}, {.id = 193, .w = 239, .h = 251},
        {.id = 194, .w = 251, .h = 237}, {.id = 195, .w = 237, .h = 251},
        {.id = 196, .w = 251, .h = 237}, {.id = 197, .w = 251, .h = 237},
        {.id = 198, .w = 251, .h = 237},
    };

    for(int i = 0; i < 10; i++) {
        bool b = rect_pack(500, 500, true, rects, nrects);
        TEST_ASSERT_TRUE(b);
    }
}

int main() {
    UNITY_BEGIN();

    RUN_TEST(test_rect_pack_uniform_paging);
    RUN_TEST(test_rect_pack_fail_paging);
    RUN_TEST(test_rect_pack_one_rect_per_page_paging);
    RUN_TEST(test_rect_pack_fill);
    RUN_TEST(test_rect_pack_loop);

    return UNITY_END();
}