# test: custom_tests/test.c gorilla.c gorilla.h
# 	gcc -g gorilla.c custom_tests/test.c -o test

# clean:
# 	rm -f *.o
# 	rm -f test

CC = gcc
CFLAGS = -g -Wall -fprofile-arcs -ftest-coverage
LDFLAGS = -fprofile-arcs -ftest-coverage
SRC = gorilla.c
TESTS = custom_tests/test.c
OBJ = $(SRC:.c=.o) $(TESTS:.c=.o)

test: $(SRC) $(TESTS)
	$(CC) $(CFLAGS) $(SRC) $(TESTS) -o test $(LDFLAGS)

run-test:
	./test

coverage: test
	./test

clean:
	rm -f *.o *.gcno *.gcda test coverage.info
	rm -rf coverage_report

