# Compiler and Flags
CC = gcc
CFLAGS = -std=c11 -Wall -Wextra -pedantic -pthread -fprofile-arcs -ftest-coverage -fPIC
CFLAGS_DEBUG = -g -O0 -fprofile-arcs -ftest-coverage -fPIC
CFLAGS_RELEASE = -O3 -DNDEBUG -flto -fprofile-arcs -ftest-coverage -fPIC

# Include Directories
INCLUDE_DIRS = -Itests

# Source and Object Files
SRC_DIR = src
TEST_DIR = tests

SRC_FILES = $(SRC_DIR)/Aces.c \
            $(SRC_DIR)/Aces-internal.c \
            $(SRC_DIR)/Channel.c \
            $(SRC_DIR)/Matrix.c \
            $(SRC_DIR)/Common.c \
            $(SRC_DIR)/Polynomial.c

TEST_FILES = $(TEST_DIR)/polynomial-test.c \
             $(TEST_DIR)/aces-internal-test.c \
             $(TEST_DIR)/common-test.c

OBJ_FILES = $(SRC_FILES:.c=.o)
TEST_OBJ_FILES = $(TEST_FILES:.c=.o)

# Static Library
LIB_NAME = libCAces.a

# Test Files
TESTS_SOURCES = $(TEST_DIR)/main-test.c
TESTS_OBJ = $(TESTS_SOURCES:.c=.o)
TEST_EXECUTABLES = $(TESTS_SOURCES:.c=)

# Build Object Files
$(SRC_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) $(INCLUDE_DIRS) -fprofile-arcs -ftest-coverage -fPIC -c $< -o $@

$(TEST_DIR)/%.o: $(TEST_DIR)/%.c
	$(CC) $(CFLAGS) $(INCLUDE_DIRS) -c $< -o $@

# Build Static Library
$(LIB_NAME): $(OBJ_FILES) $(TEST_OBJ_FILES)
	ar rcs $@ $^

# Compile Test Executable
main-test: $(TEST_DIR)/main-test.o $(LIB_NAME)
	$(CC) $(CFLAGS) -o $@ $^ -L. -lCAces -pthread -lm

# Run Tests
test: main-test
	@echo "Running main test..."
	@./main-test
	@echo "Main test completed!"

# Clean Build
clean:
	rm -rf $(OBJ_FILES) $(LIB_NAME) $(TESTS_OBJ) $(TEST_EXECUTABLES)
	rm -rf **/*.gcno **/*.gcda **/*.gcov
	rm -rf **/*.o
	rm -rf *.gcno *.gcda *.gcov
	rm -rf main-test.o main-test

.PHONY: all clean test
