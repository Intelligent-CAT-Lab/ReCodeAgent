#include "aes.h"

const char base64_table[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

char* base64_encode(const unsigned char *input, int len) {
    int output_len = 4 * ((len + 2) / 3);
    char *output = (char*)malloc(output_len + 1); 
    
    if (output == NULL) {
        return NULL;
    }

    int i, j;
    for (i = 0, j = 0; i < len; ) {
        uint32_t octet_a = i < len ? input[i++] : 0;
        uint32_t octet_b = i < len ? input[i++] : 0;
        uint32_t octet_c = i < len ? input[i++] : 0;

        uint32_t triple = (octet_a << 16) | (octet_b << 8) | octet_c;

        output[j++] = base64_table[(triple >> 18) & 0x3F];
        output[j++] = base64_table[(triple >> 12) & 0x3F];
        output[j++] = base64_table[(triple >> 6) & 0x3F]; 
        output[j++] = base64_table[triple & 0x3F]; 
    }

    for (int k = 0; k < (3 - (len % 3)) % 3; k++) {
        output[--j] = '=';
    }

    output[output_len] = '\0';
    return output;
}

