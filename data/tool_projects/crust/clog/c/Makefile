# Compiler
CC = gcc

# Compiler flags
CFLAGS = -Wall -Wextra -Werror -std=c11 -fprofile-arcs -ftest-coverage -fPIC

# Source files
SRC = tests/clog_test_c.c
OBJ = clog_test.o
TARGET = clog_test

# Build the test executable
$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) -o $(TARGET) $(OBJ) 

# Compile the test source file
$(OBJ): $(SRC) src/clog.h
	$(CC) $(CFLAGS) -c $(SRC) -o $(OBJ) 

all: test
# Run tests
test: $(TARGET)
	./$(TARGET)

# Clean up generated files
clean:
	rm -f $(TARGET) $(OBJ)

# Phony targets
.PHONY: clean test
