CC = cc
CFLAGS = -std=c99 -g -Wall

.PHONY: all clean heap hash test

all: heap hash hash rubik_model_tests solve_rubik

test: rubik_model_tests
	$(MAKE) -C hash test
	$(MAKE) -C heap test
	./rubik_model_tests

heap/*:
	$(MAKE) -C heap

hash/*:
	$(MAKE) -C hash

rubik_model_tests.o: rubik_model.h

rubik_model.o: rubik_model.h

solve_rubik.o: rubik_model.h heap/heap.h hash/hash.h

rubik_model_tests: rubik_model_tests.c rubik_model.c
	$(CC) $(CFLAGS) rubik_model_tests.c rubik_model.c -o rubik_model_tests

solve_rubik: solve_rubik.c rubik_model.c heap/heap.o hash/hash.o
	$(CC) $(CFLAGS) solve_rubik.c rubik_model.c heap/heap.o hash/hash.o -o solve_rubik

clean:
	rm -rf *~ *.o *.o rubik_model_tests solve_rubik
	find . -type d ! -name . -exec $(MAKE) -C {} clean \;

