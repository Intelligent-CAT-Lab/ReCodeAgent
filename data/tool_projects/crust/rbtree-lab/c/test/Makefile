
.PHONY: test

CFLAGS=-I ../src -Wall -g -DSENTINEL -fPIC -fprofile-arcs -ftest-coverage

test: test-rbtree
	./test-rbtree
	valgrind ./test-rbtree

test-rbtree: test-rbtree.o ../src/rbtree.o
	gcc $(CFLAGS) -o test-rbtree test-rbtree.o ../src/rbtree.o

../src/rbtree.o:
	$(MAKE) -C ../src rbtree.o

clean:
	rm -f test-rbtree *.o
	rm -rf *.gcda *.gcno *.gcov