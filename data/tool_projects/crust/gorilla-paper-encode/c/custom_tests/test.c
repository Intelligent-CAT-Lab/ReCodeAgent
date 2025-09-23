/* Author: kun_tsai  GuangZhou 2023 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "../gorilla.h"

void test_float_encoding()
{
    u8 buffer[1024] = {0};
    u32 length = 0;
    f64 arr[8] = {2300.0, 2400.0, 2500.0, 2600.0, 2700.0, 2800.0, 2900.0, 3000};
    f64 de_arr[64] = {0};
    u32 de_len = 0;

    float_encoder_t encode;
    float_decoder_t decode;
    float_encoder_init(&encode);

    for (int i = 0; i < 8; i++) {
        float_encode_write(&encode, arr[i]);
    }
    float_encode_flush(&encode, buffer, &length);

    assert(length > 0); // Ensure data was actually written

    float_decode_block(&decode, buffer, length, de_arr, &de_len);

    assert(de_len == 8); // Ensure correct number of elements decoded
    for (int i = 0; i < 8; i++) {
        assert(de_arr[i] == arr[i]); // Ensure values match
    }
}

int main()
{
    test_float_encoding();
    printf("All tests passed!\n");
    return 0;
}
