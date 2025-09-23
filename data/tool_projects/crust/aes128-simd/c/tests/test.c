#include "../src/aes.h"
#include<assert.h>
int main() {
    uint8_t key[16] = {
        0x2B,0x7E,0x15,0x16,0x28,0xAE,0xD2,0xA6,0xAB,0xF7,0x15,0x88,0x09,0xCF,0x4F,0x3C
    };
    printf("key : ");
    for (int i = 0; i < 16; ++i)
        printf("%02X ", key[i]);
    printf("\n");
	printf("key size : %ld\n", sizeof(key) / sizeof(key[0]));
	uint8_t expandedKey[4 * Nb * (Nr + 1)];
    Expansion(key, expandedKey);

    printf("Expanded key : ");
    for (int i = 0; i < 4 * Nb * (Nr + 1); ++i)
        printf("%02X ", expandedKey[i]);
    printf("\n");

    char str[] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor ...";

    size_t strLen = strlen(str);
	printf("\n");
    printf("str : %s\n", str);
	printf("base64 : %s\n", base64_encode((const unsigned char *)str, strLen));
	printf("\n");
    uint8_t *paddedData = NULL;
    size_t paddedLen = 0;
    PadBuffer((uint8_t*)str, strLen, &paddedData, &paddedLen);

    printf("To encrypt: ");
    for (size_t i = 0; i < paddedLen; ++i)
        printf("%02X ", paddedData[i]);
    printf("\n");

    uint8_t *ciphertext = malloc(paddedLen);

    for (size_t i = 0; i < paddedLen; i += 16) {
        Cipher(paddedData + i, ciphertext + i, expandedKey);
    }

	printf("\n");
    uint8_t *decryptedData = _mm_malloc(paddedLen, 16);

    for (size_t i = 0; i < paddedLen; i += 16) {
        InvCipher(ciphertext + i, decryptedData + i, expandedKey);
    }

    size_t unpaddedLen = RemovePadding(decryptedData, paddedLen);

    printf("Decrypted str : ");
    for (size_t i = 0; i < unpaddedLen; ++i)
        printf("%02X ", decryptedData[i]);
    printf("\n");
	
	char * decrypted = base64_encode(decryptedData, unpaddedLen);
    printf("Decrypted str : %.*s\n", (int)unpaddedLen, decryptedData);
	printf("base64 : %s\n", decrypted);
    assert(memcmp(str, decryptedData, unpaddedLen) == 0);
	// if (memcmp(str, decryptedData, unpaddedLen) == 0)
    //     printf("Done.\n");
    // else
    //     printf("Failed.\n");

    _mm_free(paddedData);
    free(ciphertext);
    free(decryptedData);

    return 0;
}
