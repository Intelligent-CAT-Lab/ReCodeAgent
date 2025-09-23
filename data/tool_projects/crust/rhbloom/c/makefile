
test: test.c rhbloom.c rhbloom.h
	gcc -o test test.c rhbloom.c rhbloom.h -fprofile-arcs -ftest-coverage -lm
	./test
clean:
	rm -f test
	rm -f *.gcno *.gcda *.gcov
	rm -rf bin