all : test

test : tests/test.c src/coroutine.c
	gcc -g -Wall -o test_1 $^ -Iinclude -fprofile-arcs -ftest-coverage -fPIC
	./test_1

clean :
	rm test_1
