.PHONY: clean

all: test_1

test_1: utf8.o test.o
	gcc -o test_1 utf8.o test.o -Wall -Wextra -O0 -g -fprofile-arcs -ftest-coverage -fPIC

test: test_1
	./test_1

utf8.o: src/utf8.c src/utf8.h
	gcc -c src/utf8.c -Wall -Wextra -O0 -g -fprofile-arcs -ftest-coverage -fPIC -o utf8.o

test.o: tests/test.c src/utf8.h
	gcc -c tests/test.c -Wall -Wextra -O0 -g -fprofile-arcs -ftest-coverage -fPIC -o test.o

clean:
	rm -f test_1 *.o *.gcda *.gcno *.gcov
