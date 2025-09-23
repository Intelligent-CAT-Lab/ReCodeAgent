#include "dict.h"

#define DEFAULT_MOD     8
#define DEFAULT_STEP    2
#define HASH_BASE       256LLU
#define HASH_MOD        1000000007LLU
#define ASSERT_MEM(x)   if(x==NULL){fprintf(stderr,"[ERRO]: out of memory.\n");exit(1);}

typedef struct dict_elem dict_elem_t;
struct dict_elem
{
    uint64_t        code;
    dict_elem_t*    prev;
    dict_elem_t*    next;
    char            key[];
};

typedef struct dict_list
{
    size_t          size;
    dict_elem_t*    head;
    dict_elem_t*    tail;
} dict_list_t;

struct dict
{
    dict_key_attr_t     key;
    dict_val_attr_t     val;
    dict_alloc_t        alloc;
    size_t              mod;
    dict_list_t*        list;
    void*               key_temp;
};


static inline bool dict_reshape( dict_t* restrict dict, size_t step )
{
    size_t old_size = dict->mod;
    size_t new_size = old_size * step * DEFAULT_STEP;

    dict_list_t* old_list = dict->list;
    dict_list_t* new_list = dict->alloc.malloc( sizeof (dict_list_t) * new_size );

    if ( new_list == NULL ) return false;

    dict->mod   = new_size;
    dict->list  = new_list;

    memset( dict->list, 0, sizeof (dict_list_t) * new_size );

    dict_elem_t* curr;
    dict_elem_t* next;
    uint64_t index;
    for ( size_t i = 0; i < old_size; i++ )
    {
        curr = old_list[i].head;
        while ( curr != NULL )
        {
            next    = curr->next;
            index   = curr->code % new_size;
            
            if ( new_list[ index ].head == NULL )
            {
                new_list[ index ].head = new_list[ index ].tail = curr;
                curr->prev = NULL;
            }
            else
            {
                new_list[ index ].tail->next = curr;
                new_list[ index ].tail = curr;
                curr->prev = new_list[ index ].tail;
            }
            new_list[ index ].size++;
            curr = next;
        }
    }

    for ( size_t i = 0; i < new_size; i++ )
    {
        new_list[i].tail->next = NULL;
    }

    if ( dict->alloc.free != NULL )
    {
        dict->alloc.free( old_list );
    }

    return true;
}


static inline void* dict_get_key( const dict_t* restrict dict, va_list ap )
{
    void* key = dict->key_temp;
    if ( dict->key.copy != NULL )
    {
        void* data = va_arg( ap, void* );
        dict->key.copy( key, data );
    }
    else
    {
        switch ( dict->key.type )
        {
            case DICT_CHAR:         *(char*)        key = va_arg( ap, int );            break;
            case DICT_WCHAR:        *(wchar_t*)     key = va_arg( ap, int );            break;
            case DICT_I32:          *(int32_t*)     key = va_arg( ap, int32_t );        break;
            case DICT_U32:          *(uint32_t*)    key = va_arg( ap, uint32_t );       break;
            case DICT_F32:          *(float*)       key = va_arg( ap, double );         break;
            case DICT_I64:          *(int64_t*)     key = va_arg( ap, int64_t );        break;
            case DICT_U64:          *(uint64_t*)    key = va_arg( ap, uint64_t );       break;
            case DICT_F64:          *(double*)      key = va_arg( ap, double );         break;
            case DICT_PTR:          *(void**)       key = va_arg( ap, void* );          break;
            case DICT_STR:
            {
                char* str = va_arg( ap, char* );
                *(char**) key = dict->alloc.malloc( strlen(str) + 1 );
                ASSERT_MEM( *(char**) key );
                strcpy( *(char**) key, str );
                break;
            }
            case DICT_STRUCT:
            {
                void* data = va_arg( ap, void* );
                memcpy( key, data, dict->key.size );
                break;
            }
            default:                fprintf( stderr, "[ERRO]: illegal type.\n" );       exit(1);
        }
    }
    return key;
}


static inline uint64_t dict_get_hash( const dict_t* restrict dict, void* restrict key )
{
    uint64_t code = 0;
    if ( dict->key.hash != NULL )
    {
        code = dict->key.hash( key );
    }
    else
    {
        size_t length;
        switch ( dict->key.type )
        {
            case DICT_CHAR:         code = *(char*)     key;    break;
            case DICT_WCHAR:        code = *(wchar_t*)  key;    break;
            case DICT_I32:          code = *(int32_t*)  key;    break;
            case DICT_U32:          code = *(uint32_t*) key;    break;
            case DICT_F32:          code = *(float*)    key;    break;
            case DICT_I64:          code = *(int64_t*)  key;    break;
            case DICT_U64:          code = *(uint64_t*) key;    break;
            case DICT_F64:          code = *(double*)   key;    break;
            case DICT_PTR:
            {
                code = *(uintptr_t*) key;
                break;
            }
            case DICT_STR:          
                length = strlen( *(char**) key );
                for ( size_t i = 0; i < length; i++ )
                {
                    code = ( code * HASH_BASE + ( *(char**) key )[i] ) % HASH_MOD;
                }
                break;
            case DICT_STRUCT:
                length = dict->key.size;
                for ( size_t i = 0; i < length; i++ )
                {
                    code = ( code * HASH_BASE + ( (char*) key )[i] ) % HASH_MOD;
                }
                break;
            default:
            {
                fprintf( stderr, "[ERRO]: illegal type.\n" );
                exit(1);
            }
        }
    }
    return code;
}


static inline void dict_free_key( const dict_t* restrict dict, void* restrict key )
{
    if ( dict->key.copy != NULL && dict->key.free != NULL )
    {
        dict->key.free( key );
    }
    else if ( dict->key.type == DICT_STR )
    {
        dict->alloc.free( *(char**) key );
    }
}


static inline void dict_free_val( const dict_t* restrict dict, void* restrict val )
{
    if ( dict->val.free != NULL )
    {
        dict->val.free( val );
    }
}


static inline void dict_free_node( const dict_t* restrict dict, dict_elem_t* restrict node )
{
    if ( dict->alloc.free != NULL )
    {
        dict->alloc.free( node );
    }
}


static inline void dict_delete_node( dict_list_t* restrict list, dict_elem_t* restrict curr )
{
    if ( curr == list->head )
    {
        list->head = curr->next;
    }
    if ( curr == list->tail )
    {
        list->tail = curr->prev;
    }
    if ( curr->prev != NULL )
    {
        curr->prev->next = curr->next;
    }
    if ( curr->next != NULL )
    {
        curr->next->prev = curr->prev;
    }
    list->size--;
}


dict_t* dict_create( dict_args_t args )
{
    size_t key_size;
    switch ( args.key.type )
    {
        case DICT_CHAR:    key_size = sizeof ( char );                      break;
        case DICT_WCHAR:   key_size = sizeof ( wchar_t );                   break;
        case DICT_I32:     key_size = sizeof ( int32_t );                   break;
        case DICT_U32:     key_size = sizeof ( uint32_t );                  break;
        case DICT_F32:     key_size = sizeof ( float );                     break;
        case DICT_I64:     key_size = sizeof ( int64_t );                   break;
        case DICT_U64:     key_size = sizeof ( uint64_t );                  break;
        case DICT_F64:     key_size = sizeof ( double );                    break;
        case DICT_PTR:     key_size = sizeof ( void* );                     break;
        case DICT_STR:     key_size = sizeof ( char* );                     break;
        case DICT_STRUCT:  
        {
            key_size = ( args.key.size + ( sizeof (uintptr_t) - 1 ) ) & ~( sizeof (uintptr_t) - 1 );
            break;
        }
        default:           fprintf( stderr, "[ERRO]: illegal type.\n" );    exit(1);
    }

    size_t val_size = ( args.val.size + ( sizeof (uintptr_t) - 1 ) ) & ~( sizeof (uintptr_t) - 1 );

    dict_t* dict = NULL;
    if ( args.alloc.malloc != NULL )
    {
        dict = args.alloc.malloc( sizeof (dict_t) );
        ASSERT_MEM( dict );
        dict->alloc = args.alloc;
    }
    else
    {
        dict = malloc( sizeof (dict_t) );
        ASSERT_MEM( dict );
        dict->alloc = (dict_alloc_t)
        {
            .malloc = malloc,
            .free   = free,
        };
    }

    dict->key = args.key;
    dict->key.size = key_size;

    dict->val = args.val;
    dict->val.size = val_size;

    dict->key_temp = dict->alloc.malloc( dict->key.size );
    ASSERT_MEM( dict->key_temp );

    dict->mod   = DEFAULT_MOD;
    dict->list  = dict->alloc.malloc( sizeof (dict_list_t) * dict->mod );
    ASSERT_MEM( dict->list );
    memset( dict->list, 0, sizeof (dict_list_t) * dict->mod );

    return dict;
}


dict_t* dict_new( dict_type_t key_type, size_t key_size, size_t val_size )
{
    return dict_create( (dict_args_t)
    {
        .key = (dict_key_attr_t)
        {
            .type = key_type,
            .size = key_size,
        },
        .val = (dict_val_attr_t)
        {
            .size = val_size,
        },
        .alloc = (dict_alloc_t) { 0 },
    });
}


void dict_destroy( dict_t* restrict dict )
{
    for ( size_t i = 0; i < dict->mod; i++ )
    {
        dict_elem_t* curr = dict->list[i].head;
        dict_elem_t* next;
        while ( curr != NULL )
        {
            next = curr->next;
            
            dict_free_key( dict, curr->key );
            if ( dict->val.size != 0 )
            {
                dict_free_val( dict, curr->key + dict->key.size );
            }
            dict_free_node( dict, curr );

            curr = next;
        }
    }

    if ( dict->alloc.free != NULL )
    {
        dict->alloc.free( dict->key_temp );
        dict->alloc.free( dict->list );
        dict->alloc.free( dict );
        dict = NULL;
    }
}


void* dict_get( dict_t* restrict dict, ... )
{
    va_list ap;
    va_start( ap, dict );

    // get the key
    void* key = dict_get_key( dict, ap );

    va_end(ap);

    // get hash code
    uint64_t code = dict_get_hash( dict, key );

    // get into the linked list
    size_t index = code % dict->mod;
    for ( dict_elem_t* curr = dict->list[ index ].head; curr != NULL; curr = curr->next )
    {
        if ( curr->code != code ) continue;
        if ( dict->key.cmpr != NULL )
        {
            if ( dict->key.cmpr( curr->key, key ) == 0 )
            {
                dict_free_key( dict, key );
                return curr->key + dict->key.size;
            }
        }
        else
        {
            switch ( dict->key.type )
            {
                case DICT_CHAR:
                case DICT_WCHAR:
                case DICT_I32:
                case DICT_U32:
                case DICT_F32:
                case DICT_I64:
                case DICT_U64:
                case DICT_F64:
                case DICT_PTR:
                case DICT_STRUCT:
                {
                    if ( memcmp( curr->key, key, dict->key.size ) == 0 )
                    {
                        dict_free_key( dict, key );
                        return curr->key + dict->key.size;
                    }
                    break;
                }
                case DICT_STR:
                {
                    if ( strcmp( *(char**) curr->key, *(char**) key ) == 0 )
                    {
                        dict_free_key( dict, key );
                        return curr->key + dict->key.size;
                    }
                    break;
                }
                default:
                {
                    fprintf( stderr, "[ERRO]: illegal type.\n" );
                    exit(1);
                }
            }
        }
    }

    // doesn't already appear in the list
    dict_elem_t* elem = dict->alloc.malloc( sizeof (dict_elem_t) + dict->key.size + dict->val.size );
    ASSERT_MEM( elem );
    *elem = (dict_elem_t)
    {
        .code   = code,
        .prev   = dict->list[ index ].tail,
    };
    memcpy( elem->key, key, dict->key.size );
    memset( elem->key + dict->key.size, 0, dict->val.size );
    if ( dict->list[ index ].size == 0 )
    {
        dict->list[ index ].head = dict->list[ index ].tail = elem;
    }
    else
    {
        dict->list[ index ].tail->next = elem;
        dict->list[ index ].tail = elem;
    }
    if ( dict->list[ index ].size++ > dict->mod )
    {
        if ( dict_reshape( dict, 1 ) == false )
        {
            return NULL;
        }
    }
    return elem->key + dict->key.size;
}


bool dict_remove( dict_t* restrict dict, ... )
{
    va_list ap;
    va_start( ap, dict );

    // get the key
    void* key = dict_get_key( dict, ap );

    va_end(ap);

    // get hash code
    uint64_t code = dict_get_hash( dict, key );

    // get into the linked list
    size_t index = code % dict->mod;
    for ( dict_elem_t* curr = dict->list[ index ].head; curr != NULL; curr = curr->next )
    {
        if ( curr->code != code ) continue;
        if ( dict->key.cmpr != NULL )
        {
            if ( dict->key.cmpr( curr->key, key ) == 0 )
            {
                // redirect node
                dict_free_key( dict, key );
                dict_delete_node( &dict->list[ index ], curr );
                // delete key and val and node
                dict_free_key( dict, curr->key );
                dict_free_val( dict, curr->key + dict->key.size );
                dict_free_node( dict, curr );
                return true;
            }
        }
        else
        {
            switch ( dict->key.type )
            {
                case DICT_CHAR:
                case DICT_WCHAR:
                case DICT_I32:
                case DICT_U32:
                case DICT_F32:
                case DICT_I64:
                case DICT_U64:
                case DICT_F64:
                case DICT_PTR:
                case DICT_STRUCT:
                {
                    if ( memcmp( curr->key, key, dict->key.size ) == 0 )
                    {
                        // redirect node
                        dict_free_key( dict, key );
                        dict_delete_node( &dict->list[ index ], curr );
                        // delete key and val and node
                        dict_free_key( dict, curr->key );
                        dict_free_val( dict, curr->key + dict->key.size );
                        dict_free_node( dict, curr );
                        return true;
                    }
                    break;
                }
                case DICT_STR:
                {
                    if ( strcmp( curr->key, key ) == 0 )
                    {
                        // redirect node
                        dict_free_key( dict, key );
                        dict_delete_node( &dict->list[ index ], curr );
                        // delete key and val and node
                        dict_free_key( dict, curr->key );
                        dict_free_val( dict, curr->key + dict->key.size );
                        dict_free_node( dict, curr );
                        return true;
                    }
                    break;
                }
                default:
                {
                    fprintf( stderr, "[ERRO]: illegal type.\n" );
                    exit(1);
                }
            }
        }
    }

    // doesn't already appear in the list
    dict_free_key( dict, key );
    return false;
}


bool dict_has( const dict_t* restrict dict, ... )
{
    va_list ap;
    va_start( ap, dict );

    void* key = dict_get_key( dict, ap );

    va_end(ap);

    uint64_t code = dict_get_hash( dict, key );

    // get into the linked list
    size_t index = code % dict->mod;
    for ( dict_elem_t* curr = dict->list[ index ].head; curr != NULL; curr = curr->next )
    {
        if ( curr->code != code ) continue;
        if ( dict->key.cmpr != NULL )
        {
            if ( dict->key.cmpr( curr->key, key ) == 0 )
            {
                return true;
            }
        }
        else
        {
            switch ( dict->key.type )
            {
                case DICT_CHAR:
                case DICT_WCHAR:
                case DICT_I32:
                case DICT_U32:
                case DICT_F32:
                case DICT_I64:
                case DICT_U64:
                case DICT_F64:
                case DICT_PTR:
                case DICT_STRUCT:
                {
                    if ( memcmp( curr->key, key, dict->key.size ) == 0 )
                    {
                        dict_free_key( dict, key );
                        return true;
                    }
                    break;
                }
                case DICT_STR:
                {
                    if ( strcmp( curr->key, key ) == 0 )
                    {
                        dict_free_key( dict, key );
                        return true;
                    }
                    break;
                }
                default:
                {
                    fprintf( stderr, "[ERRO]: illegal type.\n" );
                    exit(1);
                }
            }
        }
    }

    // doesn't already appear in the list
    dict_free_key( dict, key );
    return false;
}


size_t dict_len( const dict_t* restrict dict )
{
    size_t size = 0;
    for ( size_t i = 0; i < dict->mod; i++ )
    {
        size += dict->list[i].size;
    }
    return size;
}


const void* dict_key( const dict_t* restrict dict, size_t* restrict size )
{
    *size = dict_len( dict );

    if ( *size == 0 )
    {
        return NULL;
    }

    char* arr = dict->alloc.malloc( dict->key.size * (*size) );

    size_t index = 0;
    for ( size_t i = 0; i < dict->mod; i++ )
    {
        for ( dict_elem_t* curr = dict->list[i].head; curr != NULL; curr = curr->next )
        {
            memcpy( arr + ( dict->key.size * index ), curr->key, dict->key.size );
            if ( index++ == *size ) return arr;
        }
    }

    return arr;
}


void* dict_serialize( const dict_t* restrict dict, size_t* restrict bytes )
{
    size_t space;
    if ( bytes == NULL )
    {
        bytes = &space;
    }

    // calculate key size and val size
    uint32_t size = dict_len( dict );
    uint32_t key_val_size[3] = { dict->key.size, dict->val.size, size };
    size_t   elem_size = dict->key.type == DICT_STR ? sizeof (uint32_t) + dict->val.size : dict->key.size + dict->val.size;

    // calculate total size
    if ( dict->key.type == DICT_STR )
    {
        *bytes = sizeof (uint32_t) * 3 + size * elem_size;
    }
    else
    {
        *bytes = sizeof (uint32_t) * 3 + size * elem_size;
    }

    // calculate key size if string type
    #ifdef __STDC_NO_VLA__
        uint32_t* strlen_table;
        if (dict->key.type == DICT_STR )
        {
            strlen_table = dict->alloc.malloc( sizeof (uint32_t) * size );
            ASSERT_MEM( strlen_table );
        }
    #else
        uint32_t strlen_table[size];
    #endif  // __STDC_NO_VLA__
    if ( dict->key.type == DICT_STR )
    {
        size_t index = 0;
        for ( size_t i = 0; i < dict->mod; i++ )
        {
            for ( dict_elem_t* curr = dict->list[i].head; curr != NULL; curr = curr->next )
            {
                strlen_table[index] = (uint32_t) strlen( *(char**) curr->key );
                *bytes += strlen_table[index];
                index++;
            }
        }
    }


    // allocate memory
    void* data = dict->alloc.malloc( *bytes );
    if ( data == NULL )
    {
        *bytes = 0;
        return NULL;
    }
    char* ptr = data;

    // store header
    memcpy( ptr, key_val_size, sizeof (uint32_t) * 3 );
    ptr += sizeof (uint32_t) * 3;

    // store individual items
    if ( dict->key.type == DICT_STR )
    {
        size_t index = 0;
        char* str_ptr = ptr + size * elem_size;
        for ( size_t i = 0; i < dict->mod; i++ )
        {
            for ( dict_elem_t* curr = dict->list[i].head; curr != NULL; curr = curr->next )
            {
                memcpy( ptr, &strlen_table[index], sizeof (uint32_t) );
                ptr += sizeof (uint32_t);
                memcpy( ptr, curr->key + dict->key.size, dict->val.size );
                ptr += dict->val.size;
                memcpy( str_ptr, *(char**) curr->key, strlen_table[index] );
                str_ptr += strlen_table[index];
                index++;
            }
        }
    }
    else
    {
        for ( size_t i = 0; i < dict->mod; i++ )
        {
            for ( dict_elem_t* curr = dict->list[i].head; curr != NULL; curr = curr->next )
            {
                memcpy( ptr, curr->key, elem_size );
                ptr += elem_size;
            }
        }
    }

    // clean up memory allocation
    #ifdef __STDC_NO_VLA__
        if ( dict->key.type != DICT_STR && dict->alloc.free != NULL )
        {
            dict->alloc.free( strlen_table );
        }
    #endif  // __STDC_NO_VLA__

    return data;
}


dict_t* dict_deserialize( dict_args_t args, const void* restrict data )
{
    const char* ptr = data;
    uint32_t key_val_size[3];
    memcpy( key_val_size, ptr, sizeof (uint32_t) * 3 );
    ptr += sizeof (uint32_t) * 3;

    size_t key_size;
    switch ( args.key.type )
    {
        case DICT_CHAR:    key_size = sizeof ( char );                      break;
        case DICT_WCHAR:   key_size = sizeof ( wchar_t );                   break;
        case DICT_I32:     key_size = sizeof ( int32_t );                   break;
        case DICT_U32:     key_size = sizeof ( uint32_t );                  break;
        case DICT_F32:     key_size = sizeof ( float );                     break;
        case DICT_I64:     key_size = sizeof ( int64_t );                   break;
        case DICT_U64:     key_size = sizeof ( uint64_t );                  break;
        case DICT_F64:     key_size = sizeof ( double );                    break;
        case DICT_PTR:     key_size = sizeof ( void* );                     break;
        case DICT_STR:     key_size = sizeof ( char* );                     break;
        case DICT_STRUCT:
        {
            key_size = ( args.key.size + ( sizeof (uintptr_t) - 1 ) ) & ~( sizeof (uintptr_t) - 1 );
            break;
        }
        default:           fprintf( stderr, "[ERRO]: illegal type.\n" );    exit(1);
    }

    size_t val_size = ( args.val.size + ( sizeof (uintptr_t) - 1 ) ) & ~( sizeof (uintptr_t) - 1 );

    if ( key_size != key_val_size[0] )
    {
        fprintf( stderr, "[ERRO]: key type conflict, data corrupted.\n" );
        return NULL;
    }

    if ( val_size != key_val_size[1] )
    {
        fprintf( stderr, "[ERRO]: val type conflict, data corrupted.\n" );
        return NULL;
    }

    dict_t* dict = NULL;
    if ( args.alloc.malloc != NULL )
    {
        dict = args.alloc.malloc( sizeof (dict_t) );
        ASSERT_MEM( dict );
        dict->alloc = args.alloc;
    }
    else
    {
        dict = malloc( sizeof (dict_t) );
        ASSERT_MEM( dict );
        dict->alloc = (dict_alloc_t)
        {
            .malloc = malloc,
            .free   = free,
        };
    }

    dict->key = args.key;
    dict->key.size = key_size;

    dict->val = args.val;
    dict->val.size = val_size;

    dict->key_temp = dict->alloc.malloc( dict->key.size );
    ASSERT_MEM( dict->key_temp );

    dict->mod = DEFAULT_MOD;
    dict->list  = dict->alloc.malloc( sizeof (dict_list_t) * dict->mod );
    ASSERT_MEM( dict->list );
    memset( dict->list, 0, sizeof (dict_list_t) * dict->mod );

    // assign all the values
    size_t elem_size = dict->key.type == DICT_STR ? sizeof (uint32_t) + dict->val.size : dict->key.size + dict->val.size;
    size_t index;
    uint64_t code;
    dict_elem_t* elem;
    if ( dict->key.type == DICT_STR )
    {
        const char* str_ptr = ptr + key_val_size[2] * elem_size;
        for ( size_t i = 0; i < key_val_size[2]; i++ )
        {
            elem = dict->alloc.malloc( sizeof (dict_elem_t) + dict->key.size + dict->val.size );
            ASSERT_MEM( elem );
            // copy string
            *(char**) elem->key = dict->alloc.malloc( *(uint32_t*) ptr + 1 );
            ASSERT_MEM( elem->key );
            memcpy( *(char**) elem->key, str_ptr, *(uint32_t*) ptr );
            ( *(char**) elem->key )[ *(uint32_t*) ptr ] = 0;
            str_ptr += *(uint32_t*) ptr;
            ptr += sizeof (uint32_t);
            memcpy( elem->key + dict->key.size, ptr, dict->val.size );
            ptr += dict->val.size;
            code = dict_get_hash( dict, elem->key );
            elem->code = code;
            index = code % dict->mod;
            elem->prev = dict->list[index].tail;
            elem->next = NULL;
            if ( dict->list[index].size++ == 0 )
            {
                dict->list[index].head = dict->list[index].tail = elem;
            }
            else 
            {
                dict->list[index].tail->next = elem;
                dict->list[index].tail = elem;
            }
        }
    }
    else 
    {
        for ( size_t i = 0; i < key_val_size[2]; i++ )
        {
            elem = dict->alloc.malloc( sizeof (dict_elem_t) + dict->key.size + dict->val.size );
            ASSERT_MEM( elem );
            memcpy( elem->key, ptr, elem_size );
            ptr += elem_size;
            code = dict_get_hash( dict, elem->key );
            elem->code = code;
            index = code % dict->mod;
            elem->prev = dict->list[index].tail;
            elem->next = NULL;
            if ( dict->list[index].size++ == 0 )
            {
                dict->list[index].head = dict->list[index].tail = elem;
            }
            else
            {
                dict->list[index].tail->next = elem;
                dict->list[index].tail = elem;
            }
        }
    }

    size_t max = 0;
    for ( size_t i = 0; i < dict->mod; i++ )
    {
        if ( dict->list[i].size > max )
        {
            max = dict->list[i].size;
        }
    }

    if ( max <= DEFAULT_MOD ) return dict;
    max /= DEFAULT_MOD;
    if ( dict_reshape( dict, max ) == false )
    {
        return NULL;
    }
    return dict;
}

