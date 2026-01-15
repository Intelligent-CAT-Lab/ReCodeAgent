CC = gcc
CFLAGS = -Wall -I src

.PHONY: all test clean

all: test

test: test/ut_test1 test/ut_test2 test/ut_test3
	./test/ut_test1
	./test/ut_test2
	./test/ut_test3

test/ut_test1: test/ut_test1.c src/skp.h src/skp.c
	$(CC) $(CFLAGS) -o $@ $<

test/ut_test2: test/ut_test2.c src/skp.h src/skp.c
	$(CC) $(CFLAGS) -o $@ $<

test/ut_test3: test/ut_test3.c src/skp.h src/skp.c
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -f test/ut_test1 test/ut_test2 test/ut_test3
