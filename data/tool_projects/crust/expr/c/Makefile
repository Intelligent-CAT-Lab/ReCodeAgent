# Compiler and flags
CC      := gcc
CFLAGS  := -fprofile-arcs -ftest-coverage -g
LDLIBS  := -lm -fprofile-arcs -ftest-coverage

# Default target
all: test

# Build target
test: test.o mapper_expr.o
	$(CC) $(CFLAGS) -o test test.o mapper_expr.o $(LDLIBS)

# Pattern rule to compile .c files into .o files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Run test & generate coverage reports
coverage: test
	./test
	@echo "Generating coverage data..."
	@gcov test.c mapper_expr.c

# Clean build artifacts
clean:
	rm -f *.o test *.gcno *.gcda *.gcov
