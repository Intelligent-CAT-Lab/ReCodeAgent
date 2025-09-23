#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include "../src/dict.h"

#define dict_get_int( dict, key ) ( *(int32_t*) dict_get( dict, key ) )

int main( void )
{
    dict_t* dict = dict_new( DICT_STR, 0, sizeof (int32_t) );

    dict_get_int( dict, "1" ) = 1;
    dict_get_int( dict, "2" ) = 2;
    dict_get_int( dict, "0" ) = 0;
    dict_get_int( dict, "-1" ) = -1;

    dict_destroy(dict);


    FILE* fp2 = fopen( "test5.bin", "rb" );
    fseek( fp2, 0, SEEK_END );
    size_t size2 = ftell(fp2);
    fseek( fp2, 0, SEEK_SET );
    void* data2 = malloc(size2);
    fread( data2, size2, 1, fp2 );
    fclose(fp2);
    dict_t* dict2 = dict_deserialize( (dict_args_t){ .key = { .type = DICT_STR }, .val = { .size = sizeof (int32_t) } }, data2 );
    free(data2);
    size_t size3;
    const char* const* keys = dict_key( dict2, &size3 );
    for ( size_t i = 0; i < size3; i++ )
    {
        printf( "[key]: %s, [val]: %d\n", keys[i], dict_get_int( dict2, keys[i] ) );
    }
    free( (void*) keys );
    dict_destroy(dict2);

    return 0;
}
