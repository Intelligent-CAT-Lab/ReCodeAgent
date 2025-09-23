test: tests/test.c nand.c
	gcc -Werror -Wall -Wextra -O2 $< -o $@ -fprofile-arcs -ftest-coverage
	./test
clean:
	rm -f test
	rm -f *.gcno *.gcda *.gcov
	rm -rf bin