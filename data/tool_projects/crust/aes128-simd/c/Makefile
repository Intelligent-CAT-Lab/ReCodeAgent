NAME = aes256
CC = gcc
CFLAGS = -Wall -Wextra -O0 -g 
LDFLAGS = -lgcov --coverage -shared

SRCS = $(wildcard src/*.c)
OBJ = $(SRCS:.c=.o)
LIB = lib$(NAME).so

# Test files
TEST_SRC = tests/test.c
TEST_OBJ = $(TEST_SRC:.c=.o)
TEST_EXE = test_runner

# Default target: Build everything
all: $(LIB) $(TEST_EXE)

# Compile shared library
$(LIB): $(OBJ)
	$(CC) $(CFLAGS) -fprofile-arcs -ftest-coverage -fPIC $(LDFLAGS) -o $(LIB) $(OBJ) 

# Compile object files for library
%.o: %.c
	$(CC) $(CFLAGS) -fprofile-arcs -ftest-coverage -fPIC -c $< -o $@

# Compile test program
$(TEST_EXE): $(TEST_SRC) $(LIB)
	$(CC) $(CFLAGS) -o $(TEST_EXE) $(TEST_SRC) -L. -l$(NAME)
	LD_LIBRARY_PATH=. ./$(TEST_EXE)

# Run tests
run_tests: $(TEST_EXE)
	LD_LIBRARY_PATH=. ./$(TEST_EXE)

test: $(TEST_EXE)
	LD_LIBRARY_PATH=. ./$(TEST_EXE)
# Clean up build files
clean:
	rm -f $(OBJ) $(LIB) $(TEST_OBJ) $(TEST_EXE)
	rm -f src/*.gcno src/*.gcda src/*.gcov
	rm -rf *.gcov *.gcda *.gcno

# Rebuild everything
re: clean all

.PHONY: all clean re run_tests
