CC = cc
CFLAGS = -g -std=c99 -Wall

.PHONY: all clean test

all: hash.o hash_tests

test: hash_tests
	./hash_tests

hash.h: ../helpers.h
hash_tests.c: hash.h
hash.c: hash.h

hash_tests: hash_tests.c hash.c
	$(CC) $(CFLAGS) hash_tests.c hash.c -o hash_tests

clean:
	rm -rf *.o hash_tests
