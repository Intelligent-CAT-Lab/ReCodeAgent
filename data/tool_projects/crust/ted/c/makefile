CC = gcc
CFLAGS = -std=c99 -Wall -Wextra -g -fPIC -fprofile-arcs -ftest-coverage
LDFLAGS = 

# Directories
SRC_DIR = app
BUFFER_DIR = buffer
TESTS_DIR = tests
BUILD_DIR = build

# Source files
BUFFER_SRC = $(BUFFER_DIR)/gap.c $(BUFFER_DIR)/buffer.c
BUFFER_OBJ = $(BUFFER_SRC:.c=.o)

APP_SRC = $(SRC_DIR)/main.c
APP_OBJ = $(APP_SRC:.c=.o)

TEST_SRC = $(TESTS_DIR)/runtests.c
TEST_OBJ = $(TEST_SRC:.c=.o)

# Output files
LIB_BUFFER = libbuffer.a
EDITOR_BIN = teditor
TEST_BIN = runtests

# Default target
all: $(EDITOR_BIN) $(TEST_BIN)

	./$(TEST_BIN)

test: $(TEST_BIN)
	./$(TEST_BIN)
# Compile Buffer library
$(LIB_BUFFER): $(BUFFER_OBJ)
	@echo "Building static library: $@"
	ar rcs $@ $^

# Compile teditor
$(EDITOR_BIN): $(APP_OBJ) $(LIB_BUFFER)
	@echo "Linking: $@"
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

# Compile test executable
$(TEST_BIN): $(TEST_OBJ) $(LIB_BUFFER)
	@echo "Linking: $@"
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

# Compile object files
%.o: %.c
	@echo "Compiling: $<"
	$(CC) $(CFLAGS) -c $< -o $@

# Clean up build artifacts
clean:
	@echo "Cleaning up..."
	rm -f $(BUFFER_OBJ) $(APP_OBJ) $(TEST_OBJ) $(LIB_BUFFER) $(EDITOR_BIN) $(TEST_BIN)
	rm -f *.gcov *.gcda *.gcno

.PHONY: all clean
