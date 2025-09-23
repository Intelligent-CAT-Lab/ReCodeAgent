#include "../src/dict.h"
#include <stdint.h>
#include <inttypes.h>

// define `dict_double` for not needing to cast the return value of `dict_get`. 
#define dict_double( dict, key ) ( *(double*) dict_get( dict, key ) )

int main( void )
{
    // create a new dictionary with key type int32_t, and value type double. 
    dict_t* dict = dict_new( DICT_I32, sizeof (int32_t), sizeof (double) );

    // iterate 30 times and add 30 items into the dictionary
    for ( int32_t i = 0; i < 30; i++ )
    {
        dict_double( dict, i ) = (double)i;
    }

    // iterate 30 times and print the values. 
    for ( int32_t i = 0; i < 30; i++ )
    {
        printf( "[key]: %2" PRId32 ", [val]: %4.2lf\n", i, dict_double( dict, i ) );
    }

    // destroy the dictionary, free up the memory used by it. 
    dict_destroy( dict );

    return 0;
}
