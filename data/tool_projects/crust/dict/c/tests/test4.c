#include "../src/dict.h"
#include <stdint.h>

#define dict_double( dict, key ) ( *(double*) dict_get( dict, key ) )

int main( void )
{
    dict_t* dict = dict_create( (dict_args_t) { .key = { .type = DICT_I32 }, .val = { .size = sizeof (double) } } );

    for ( int32_t i = 0; i < 500; i++ )
    {
        dict_double( dict, i ) = (double)i;
    }

    for ( int32_t i = 0; i < 500; i++ )
    {
        printf( "%s: ", dict_has( dict, i ) == true ? "yes" : " no" );
        printf( "%5lf\n", dict_double( dict, i ) );
    }

    dict_destroy( dict );

    return 0;
}