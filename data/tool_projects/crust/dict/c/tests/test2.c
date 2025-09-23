#include "../src/dict.h"
#include <string.h>

typedef struct str
{
    size_t size;
    char* str;
} str_t;


// dict_deep_copy
void str_copy( void* dest, const void* src )
{
    str_t* d = dest;
    const str_t* s = src;
    d->size = s->size;
    d->str = malloc( s->size + 1 );
    strcpy( d->str, s->str );
}

// dict_free
void str_free( void* ptr )
{
    free( ( (str_t*) ptr )->str );
    ( (str_t*) ptr )->str = NULL;
}

// dict_hash
uint64_t str_hash( const void* ptr )
{
    const str_t* str = ptr;
    uint64_t code = 0;
    for ( size_t i = 0; i < str->size; i++ )
    {
        code = ( code * 256 + ( str->str[i] ) % 1007 );
    }
    return code;
}

// dict_cmpr
int str_cmpr( const void* s1, const void* s2 )
{
    const str_t* str1 = s1;
    const str_t* str2 = s2;
    return strcmp( str1->str, str2->str );
}

#define dict_uint64( dict, str )    ( *(uint64_t*) dict_get( dict, str ) )

int main( void )
{
    // attribute list for key. 
    dict_key_attr_t key_attr = 
    {
        .type = DICT_STRUCT,
        .size = sizeof (str_t),
        .copy = str_copy,
        .free = str_free,
        .hash = str_hash,
        .cmpr = str_cmpr,
    };
    dict_t* dict = dict_create_args( .key = key_attr, .val = { .size = sizeof (uint64_t) } );

    char* s1 = "s1";
    char* s2 = "s2";
    char* s3 = "s3";
    char* s4 = "s4";
    char* s5 = "s5";

    dict_uint64( dict, ( &(str_t){ strlen(s1), s1 } ) ) = 1;
    dict_uint64( dict, ( &(str_t){ strlen(s2), s2 } ) ) = 2;
    dict_uint64( dict, ( &(str_t){ strlen(s3), s3 } ) ) = 3;
    dict_uint64( dict, ( &(str_t){ strlen(s4), s4 } ) ) = 4;
    dict_uint64( dict, ( &(str_t){ strlen(s5), s5 } ) ) = 5;

    printf( "val: %lu\n", dict_uint64( dict, ( &(str_t){ strlen("s4"), "s4" } ) ) );
    printf( "remove: %d\n", dict_remove( dict, ( &(str_t){ strlen("s3"), "s3" } ) ) );

    size_t size;
    const str_t* arr = dict_key( dict, &size );
    printf( "key amount: %zu\n", size );
    for ( size_t i = 0; i < size; i++ )
    {
        printf( "key[%zu]: %s\n", i, arr[i].str );
    }
    free( (void*)arr );

    dict_destroy( dict );

    return 0;
}
