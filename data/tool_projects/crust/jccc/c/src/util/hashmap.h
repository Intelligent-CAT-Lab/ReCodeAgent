struct BucketNode {
    char *key;
    void *value;
    struct BucketNode *next;
};

struct Hashmap {
    struct BucketNode **buckets;
    int size;
    int cap;

    unsigned (*hash)(char *);
    unsigned (*equals)(char *, char *);
};

struct Hashmap *create_hashmap(int capacity);
void destroy_hashmap(struct Hashmap *h);

struct BucketNode *create_bucket(char *key, void *value);

// Get a value with a key
struct BucketNode *hm_get(struct Hashmap *h, char *key);
// Set a value with a key
int hm_set(struct Hashmap *h, char *key, void *value);
// Double the capacity of the hashmap (happens automatically when size >
// capacity)
void double_cap(struct Hashmap *h);

int test_hash_init();
int test_hash_init_and_store();
int test_hash_set_and_get();
int test_hash_set_and_double_get();
