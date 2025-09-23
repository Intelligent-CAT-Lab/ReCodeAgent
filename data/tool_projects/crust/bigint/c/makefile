# Compiler
CC = gcc
CFLAGS = -std=c99 -Wall -Wextra -pedantic
DEBUG_FLAGS = -g -O0 -fsanitize=address
COVERAGE_FLAGS = -fprofile-arcs -ftest-coverage -fPIC

# Include directories
INCLUDES = -I.

# Source files for actual implementation (tracked for coverage)
SRC_FILES = src/bigint.h  # Add all source files here
OBJ_FILES = $(SRC_FILES:.c=.o)  # Convert to object files

# Test files (NOT tracked for coverage)
SRCS_TEST1 = tests/test1.c
SRCS_TEST2 = tests/test2.c
SRCS_TEST3 = tests/test3.c

# Output binaries
BIN_TEST1 = test1
BIN_TEST2 = test2
BIN_TEST3 = test3

# Default build target
all: $(OBJ_FILES) $(BIN_TEST1) $(BIN_TEST2) $(BIN_TEST3)

# ✅ Compile each source file separately
%.o: %.c
	$(CC) $(CFLAGS) $(COVERAGE_FLAGS) -c $< -o $@

# ✅ Build test1 using precompiled object files
$(BIN_TEST1): $(SRCS_TEST1) $(OBJ_FILES)
	$(CC) $(CFLAGS) $(DEBUG_FLAGS) $(INCLUDES) -o $(BIN_TEST1) $(SRCS_TEST1) $(OBJ_FILES) $(COVERAGE_FLAGS)

# ✅ Build test2 using precompiled object files
$(BIN_TEST2): $(SRCS_TEST2) $(OBJ_FILES)
	$(CC) $(CFLAGS) $(DEBUG_FLAGS) $(INCLUDES) -o $(BIN_TEST2) $(SRCS_TEST2) $(OBJ_FILES) $(COVERAGE_FLAGS)

# ✅ Build test3 using precompiled object files
$(BIN_TEST3): $(SRCS_TEST3) $(OBJ_FILES)
	$(CC) $(CFLAGS) $(DEBUG_FLAGS) $(INCLUDES) -o $(BIN_TEST3) $(SRCS_TEST3) $(OBJ_FILES) $(COVERAGE_FLAGS)

# ✅ Run tests
test: all
	./$(BIN_TEST1)
	./$(BIN_TEST2)
	./$(BIN_TEST3)

# ✅ Generate coverage report ONLY for source files
coverage: test
	@gcov -b -c $(SRC_FILES)

# ✅ Clean build files and coverage files
clean:
	rm -f $(BIN_TEST1) $(BIN_TEST2) $(BIN_TEST3)
	rm -f *.gcno *.gcda *.gcov
	rm -f src/*.gcno src/*.gcda src/*.gcov
	rm -f tests/*.gcno tests/*.gcda tests/*.gcov
