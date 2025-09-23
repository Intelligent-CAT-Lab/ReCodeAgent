#include "../src/queue.c"
#include <assert.h>
#include <stdio.h>

void
crud_test()
{
	printf("\n!!--CRUD Test--!!\n");

	queue_t q;
	queue_new(&q);

	printf("✅ Queue initialization\n");

	assert(queue_empty(&q));

	printf("✅ Queue Empty\n");

	int a = 10, b = 15;

	queue_push(&q, (void*) &a);
	assert(q.size == 1);
	assert(queue_front(&q) == queue_back(&q));

	printf("✅ Queue Push\n");

	queue_push(&q, (void*) &b);
	assert(q.size == 2);

	assert(*((int*) queue_front(&q)) == a);

	printf("✅ Queue Front\n");

	assert(*((int*) queue_back(&q)) == b);

	printf("✅ Queue Back\n");

	assert(*((int*)queue_pop(&q)) == a);
	assert(q.size == 1);

	printf("✅ Queue Pop\n");

	queue_free(&q);

	printf("✅ Queue Free\n");
}

int main() {
	crud_test();
}