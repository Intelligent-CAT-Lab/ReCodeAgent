# Compiler and flags
CC = gcc
CFLAGS = -Wall -Wextra -O2 -fprofile-arcs -ftest-coverage -g

# Default target: build the final executable
all: test.out
	./test.out

# Link the object files into the final executable
test.out: test.o amp.o
	$(CC) $(CFLAGS) $^ -o $@

# Compile each .c into .o
test.o: tests/test.c
	$(CC) $(CFLAGS) -c $<

amp.o: src/amp.c
	$(CC) $(CFLAGS) -c $<

# Run the test (depends on the output binary)
test: test.out
	@./test.out

# Clean generated files
clean:
	rm -f *.o test.out
	rm -f *.gcda *.gcno *.gcov coverage.info
	rm -rf coverage_report

.PHONY: all test clean
