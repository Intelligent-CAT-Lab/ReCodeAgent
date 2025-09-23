
CFLAGS=-Wall -Wextra -O3 -fprofile-arcs -ftest-coverage -fPIC

ifdef DEBUG
	CFLAGS:=-g $(CFLAGS)
endif

all: test

runtests: tests/test.c src/carrays.o src/carrays.h src/circ_array.h
	$(CC) $(CFLAGS) -o $@ tests/test.c src/carrays.o

carrays.o: carrays.c carrays.h
	$(CC) $(CFLAGS) -fprofile-arcs -ftest-coverage -fPIC -c -o $@ $<

test: runtests
	./runtests

clean:
	rm -rf runtests carrays.o
	rm -rf **/*.gcno **/*.gcda **/*.gcov
	rm -rf **/*.o
	rm -rf *.gcno *.gcda *.gcov
.PHONY: all clean test
