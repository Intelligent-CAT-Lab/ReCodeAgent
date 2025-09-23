#include <stdio.h>
#include <libqueue/queue.h>

int main() {

	queue_t q;
	queue_new(&q);

	int a = 10;
	char *b = "Hello World";

	queue_push(&q, (void*) &a);
	queue_push(&q, (void*) b);

	int *k = queue_pop(&q);
	char *l = queue_pop(&q);

	printf("%d\n", *k);
	printf("%s\n", l);

	queue_free(&q);

	return 0;
}