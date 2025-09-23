.PHONY: clean

CFLAGS=-Wall -g -fPIC -fprofile-arcs -ftest-coverage

driver: driver.o rbtree.o
    gcc $(CFLAGS) -o driver driver.o rbtree.o
test_rbtree_rotate: test_rbtree_rotate.o rbtree.o
    gcc $(CFLAGS) -o test_rotate test_rbtree_rotate.o rbtree.o
test_rbtree_insert: test_rbtree_insert.o rbtree.o
    gcc $(CFLAGS) -o test_rbtree_insert test_rbtree_insert.o rbtree.o
clean:
	rm -f driver test_rotate test_rbtree_insert *.o
	rm -rf *.gcda *.gcno *.gcov