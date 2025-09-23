CC=gcc

CFLAGS=-Wall -Wextra -Werror -Wshadow -std=c99 -pedantic -g -fwrapv -fPIC -fprofile-arcs -ftest-coverage

.PHONY: sloth
default: sloth

sloth: slothvm.c sloth.c
	$(CC) $(CFLAGS) -o sloth sloth.c slothvm.c lib/throw.c lib/stack.c parser.c $(CFLAGSEXTRA)

test: slothvm.c tests.c
	$(CC) $(CFLAGS) -o slothTest tests.c slothvm.c lib/throw.c lib/stack.c parser.c $(CFLAGSEXTRA)
	./slothTest
clean:
	rm -Rf sloth
	rm -rf *.gcov *.gcda *.gcno
	

cleanTest:
	rm -Rf slothTest