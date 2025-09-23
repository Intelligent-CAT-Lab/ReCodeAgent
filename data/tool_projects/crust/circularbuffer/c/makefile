# Compiler and Flags
CC = gcc
CFLAGS = -std=c99 -Wall -Wextra -Wpedantic -pthread -fprofile-arcs -ftest-coverage -fPIC

# Directories
SRC_DIR = src
TEST_DIR = tests

# Static Library
LIB_NAME = libcircularbuffer.a
LIB_SRC = $(SRC_DIR)/circular_buffer.c
LIB_OBJ = $(LIB_SRC:.c=.o)

# Test Executable
TEST_EXEC = test_1
TEST_SRC = $(TEST_DIR)/test.c
TEST_OBJ = $(TEST_SRC:.c=.o)

# Default target: Build library and test executable
all: $(LIB_NAME) $(TEST_EXEC)

# Build circular_buffer Static Library
$(LIB_NAME): $(LIB_OBJ)
	ar rcs $(LIB_NAME) $(LIB_OBJ)

# Compile circular_buffer Source File
$(SRC_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS)  -c $< -o $@

# Compile Test Source File
$(TEST_DIR)/%.o: $(TEST_DIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

# Build Test Executable
$(TEST_EXEC): $(TEST_OBJ) $(LIB_NAME)
	$(CC) $(CFLAGS) -o $(TEST_EXEC) $(TEST_OBJ) $(LIB_NAME) -pthread -lm

# Run Tests
test: $(TEST_EXEC)
	@echo "Running tests..."
	./$(TEST_EXEC)

# Clean Build
clean:
	rm -f $(LIB_OBJ) $(LIB_NAME) $(TEST_OBJ) $(TEST_EXEC)

.PHONY: all clean test
