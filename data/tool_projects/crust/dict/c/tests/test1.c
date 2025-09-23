#include <string.h>
#include "../src/dict.h"


#define dict_test( dict, key )      ( *(test_t*) dict_get( dict, key ) )


typedef struct test
{
    int64_t x;
    double  y;
} test_t;


int main( void )
{
    dict_t* dict = dict_create_args( .key = { .type = DICT_STR }, .val = { .size = sizeof (test_t) } );


    dict_test( dict, "Hello" ) = (test_t) { .x = 69, .y = 3.14 };
    dict_test( dict, "World" ) = (test_t) { .x = 3, .y = 69.0 };

    char* str = malloc(6);
    strcpy( str, "Hello" );
    test_t temp = dict_test( dict, str );
    free(str);
    
    printf( "%ld, %lf\n", temp.x, temp.y );

    size_t arr_size;
    char* const* arr = dict_key( dict, &arr_size );
    for ( size_t i = 0; i < arr_size; i++ )
    {
        printf( "%s\n", arr[i] );
    }
    free( (void*) arr );

    dict_destroy( dict );

    return 0;
}
