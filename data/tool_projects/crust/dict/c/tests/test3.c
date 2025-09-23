#include "../src/dict.h"
#include <stdint.h>
#include <inttypes.h>

#define dict_double( dict, key ) ( *(double*) dict_get( dict, key ) )

int main( void )
{
    dict_args_t dict_args = 
    {
        .key = { .type = DICT_I32 },
        .val = { .size = sizeof (double) },
    };
    dict_t* dict1 = dict_create( dict_args );

    for ( int32_t i = 0; i < 500; i++ )
    {
        dict_double( dict1, i ) = (double)i;
    }

    // serialize
    size_t size1;
    void* data1 = dict_serialize( dict1, &size1 );
    if ( data1 == NULL )
    {
        fprintf( stderr, "Fail to serialize.\n" );
        exit(1);
    }
    dict_destroy( dict1 );
    
    // save to file
    FILE* fp1 = fopen( "test3.bin", "wb+" );
    fprintf( fp1, "%zu\n", size1 );
    fwrite( data1, size1, 1, fp1 );
    fclose( fp1 );
    free( data1 );

    // read from file
    FILE* fp2 = fopen( "test3.bin", "rb" );
    size_t size2;
    void* data2;
    fscanf( fp2, "%zu\n", &size2 );
    data2 = malloc( size2 );
    fread( data2, size2, 1, fp2 );

    // deserialize
    dict_t* dict2 = dict_deserialize( dict_args, data2 );
    if ( dict2 == NULL )
    {
        fprintf( stderr, "Fail to deserialize.\n" );
        exit(1);
    }
    fclose( fp2 );
    free( data2 );

    // print
    size_t len = dict_len( dict2 );
    for ( int32_t i = 0; i < (int32_t) len; i++ )
    {
        printf( "[key]: %2" PRId32 ", [val]: %4.2lf\n", i, dict_double( dict2, i ) );
    }
    dict_destroy( dict2 );

    return 0;
}
