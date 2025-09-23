CC = gcc
CFLAGS = -Wall -g -fprofile-arcs -ftest-coverage -fPIC

# Include directories
C_INCLUDES = -I./src


# Define the object files
OBJ = src/btree.o test.o

# Define the executable file
EXEC = test

# The first rule is the default rule
# Build the executable by linking object files
$(EXEC): $(OBJ)
	$(CC) $(CFLAGS) $(C_INCLUDES) -o $(EXEC)_1 $(OBJ)

# Rule for building object file for btree
btree.o: src/btree.c src/btree.h
	$(CC) $(CFLAGS) $(C_INCLUDES) -c src/btree.c -o src/btree.o

# Rule for building object file for test
test.o: tests/test.c src/btree.h
	$(CC) $(CFLAGS) $(C_INCLUDES) -c tests/test.c -o ./test.o

test: test.o src/btree.o
	$(CC) $(CFLAGS) $(C_INCLUDES) -o test_1 test.o src/btree.o
	./test_1
# Clean rule
clean:
	rm -rf *.gcno *.gcda *.gcov
	rm -rf **/*.gcno **/*.gcda **/*.gcov
	rm -rf $(EXEC)_1
	rm -rf $(OBJ)
	rm -rf *.o
