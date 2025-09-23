#ifndef __DICT_H__
#define __DICT_H__

#include <stdio.h>
#include <errno.h>
#include <wchar.h>
#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>


typedef enum
{
    DICT_CHAR,     // char
    DICT_WCHAR,    // wchar_t
    DICT_I32,      // int32_t
    DICT_U32,      // uint32_t
    DICT_F32,      // float
    DICT_I64,      // int64_t
    DICT_U64,      // uint64_t
    DICT_F64,      // double
    DICT_PTR,      // void*
    DICT_STR,      // char*
    DICT_STRUCT,   // struct
} dict_type_t;

typedef void (*dict_deep_copy)( void* dest, const void* src );      // if not specified, memcpy will be performed for DICT_STRUCT, strdup will be performed for DICT_STR, shallow copy for all others
typedef void (*dict_desctructor)( void* ptr );                      // needs to be specified if has inner allocation

typedef int (*dict_cmpr)( const void* ptr1, const void* ptr2 );     // used to compare if key is equal, return 0 if equal. 
typedef uint64_t (*dict_hash)( const void* ptr );                   // return the hash code of the type. Two equal object should return the same code. 

typedef void* (*dict_malloc)( size_t size );                        // malloc for custom allocator
typedef void  (*dict_free)( void* ptr );                            // free for custom alloc

typedef struct
{
    dict_malloc    malloc;      // must be provided if a custom allocator is desired
    dict_free      free;        // not necessary, because things like an arena alloc may not have a free function
} dict_alloc_t;

typedef struct
{
    dict_type_t         type;
    size_t              size;
    dict_deep_copy      copy;   // if not provided, memcpy for DICT_STRUCT, and strdup for DICT_STR, shallow copy for DICT_PTR. 
    dict_desctructor    free;   // only needed if copy is provided
    dict_hash           hash;
    dict_cmpr           cmpr;
} dict_key_attr_t;

typedef struct
{
    size_t              size;
    dict_desctructor    free;   // free the inside alloation, the value address space is managed by the library. 
} dict_val_attr_t;

typedef struct
{
    dict_key_attr_t     key;    // key attribute
    dict_val_attr_t     val;    // val attribute
    dict_alloc_t        alloc;  // cumstom alloc set for dict
} dict_args_t;

typedef struct dict dict_t;


// function
dict_t*     dict_create( dict_args_t args );                                    // dictionary constructor, return a pointer of `dict_t`
dict_t*     dict_new( dict_type_t key_type, size_t key_size, size_t val_size ); // dictionary constructor, return a pointer of `dict_t`. Easier to use, but with less control. 
void        dict_destroy( dict_t* dict );                                       // dictionary destructor. Free the memory used by dict, also free each key and value if destructor provided. 
void*       dict_get( dict_t* dict, /* T key */... );                           // for DICT_STRUCT, pass in the address of the struct. Return the address of `val` to the corresponding `key`. Create new key-val pair if the input key was not in the dictionary. 
bool        dict_remove( dict_t* dict, /* T key */... );                        // for DICT_STRUCT, pass in the address of the struct. Return true if key deleted and it was in the dict. 
bool        dict_has( const dict_t* dict, /* T key */... );                     // for DICT_STRUCT, pass in the address of the struct. Return true if key is in the dict. 
size_t      dict_len( const dict_t* dict );                                     // return the total amount of pairs exist in the dict
const void* dict_key( const dict_t* dict, size_t* size );                       // return an array contains all the keys of the dict unordered. The array is allocated by `alloc.malloc` if specified, otherwise libc malloc is used. Don't change the key in the array since shallow copy is used. 
void*       dict_serialize( const dict_t* dict, size_t* bytes );                // return the pointer to the encoded data, allocated using specified `malloc`. 
dict_t*     dict_deserialize( dict_args_t args, const void* data );             // this function does not free `data`, you still need to free `data` if necessary. 



// dict_create_args( dict_key_attr_t key, dict_key_attr_t val, dict_alloc_t alloc )
// .key = { .type, .size, .copy, .free, .hash, .cmpr }
// .val = { .size, .free }
// .alloc = { .malloc, .free }
#define dict_create_args( ... )                     dict_create( (dict_args_t) { __VA_ARGS__ } )


#endif  // __DICT_H__
