CC = cc
CFLAGS = -g -std=c99 -Wall

.PHONY: all clean test

all: heap.o heap_tests

test: heap_tests
	./heap_tests

heap.h: ../helpers.h
heap.c: heap.h
heap_tests.c: heap.h

heap_tests: heap_tests.c heap.o
	$(CC) $(CFLAGS) heap_tests.c heap.o -o heap_tests

clean:
	rm -rf *~ *.o heap_tests
