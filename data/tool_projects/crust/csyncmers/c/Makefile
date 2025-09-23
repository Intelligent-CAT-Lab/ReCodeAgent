all:
	gcc -o test_1 tests/test.c -fprofile-arcs -ftest-coverage -fPIC
	gcc -O3 -march=native -o benchmark tests/benchmark.c -fprofile-arcs -ftest-coverage -fPIC
test:
	./test_1
	./benchmark

clean:
	rm -f test_1 benchmark
	rm -rf *.gcno *.gcda *.gcov