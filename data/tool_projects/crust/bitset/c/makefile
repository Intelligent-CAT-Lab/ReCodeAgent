# Compiler and Flags
CC = gcc
CFLAGS = -std=c99 -Wall -Wextra -pedantic -fprofile-arcs -ftest-coverage -fPIC

# Directories
SRC_DIR = src
INCLUDE_DIR = src/include/cborg
TESTS_DIR = tests
INSTALL_DIR = /usr/local

# Library and Executables
LIB_NAME = libcborg-bitset.a
LIB_OBJ = $(SRC_DIR)/bitset.o
TEST_EXEC = test_bitset

# Source and Header Files
SRC_FILES = $(wildcard $(SRC_DIR)/*.c)
OBJ_FILES = $(SRC_FILES:.c=.o)
HEADERS = $(wildcard $(INCLUDE_DIR)/*.h)

# Default target
.PHONY: all clean test install uninstall

all: $(LIB_NAME) $(TEST_EXEC) ./test_bitset

# Build Static Library
$(LIB_NAME): $(OBJ_FILES)
	ar rcs $(LIB_NAME) $(OBJ_FILES)

# Compile Source Files
$(SRC_DIR)/%.o: $(SRC_DIR)/%.c $(HEADERS)
	$(CC) $(CFLAGS) -I$(INCLUDE_DIR) -c $< -o $@

# Build Test Executable
$(TEST_EXEC): $(TESTS_DIR)/test_bitset.c $(LIB_NAME)
	$(CC) $(CFLAGS) -I$(INCLUDE_DIR) -o $(TEST_EXEC) $(TESTS_DIR)/test_bitset.c $(LIB_NAME)

# Run Tests
test: $(TEST_EXEC)
	./$(TEST_EXEC)

# Install
install:
	mkdir -p $(INSTALL_DIR)/include/cborg
	mkdir -p $(INSTALL_DIR)/lib
	cp $(INCLUDE_DIR)/cborg/bitset.h $(INSTALL_DIR)/include/cborg/
	cp $(LIB_NAME) $(INSTALL_DIR)/lib/

# Uninstall
uninstall:
	rm -f $(INSTALL_DIR)/include/cborg/bitset.h
	rm -f $(INSTALL_DIR)/lib/$(LIB_NAME)

# Clean
clean:
	rm -f $(OBJ_FILES) $(LIB_NAME) $(TEST_EXEC)
	rm -f *.gcno *.gcda *.gcov
