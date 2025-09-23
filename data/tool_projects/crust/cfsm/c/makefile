# Compiler and Flags
CC = gcc
CFLAGS = -std=c99 -Wall -Wextra -Wpedantic -pthread -fprofile-arcs -ftest-coverage -fPIC
CFLAGS_DEBUG = -g -O0
CFLAGS_RELEASE = -O3 -DNDEBUG -flto

# Include Directories
INCLUDE_DIRS = -I. -IUnity-2.5.2 -Itests

# Source Files
SRC_DIR = src
TEST_DIR = tests
# EXAMPLES_DIR = examples/mario

# Static Library
LIB_NAME = libcfsm.a
LIB_SRC = src/c_fsm.c
LIB_OBJ = $(LIB_SRC:.c=.o)

# Unity Library
UNITY_DIR = tests/Unity-2.5.2
UNITY_SRC = $(UNITY_DIR)/src/unity.c
UNITY_OBJ = $(UNITY_SRC:.c=.o)

# Test Files
TEST_EXEC = test_c_fsm
TEST_SRC = $(TEST_DIR)/test_c_fsm.c
TEST_OBJ = $(TEST_SRC:.c=.o)

# Mario Example
# MARIO_EXEC = cfsm_mario
# MARIO_SRC = $(EXAMPLES_DIR)/main.c \
#             $(EXAMPLES_DIR)/mario.c \
#             $(EXAMPLES_DIR)/states/small_mario.c \
#             $(EXAMPLES_DIR)/states/super_mario.c \
#             $(EXAMPLES_DIR)/states/fire_mario.c \
#             $(EXAMPLES_DIR)/states/cape_mario.c \
#             $(EXAMPLES_DIR)/states/dead_mario.c
# MARIO_OBJ = $(MARIO_SRC:.c=.o)

# Default Build Type
BUILD_TYPE ?= Release

# Set Optimization Flags Based on Build Type
ifeq ($(BUILD_TYPE), Debug)
    CFLAGS += $(CFLAGS_DEBUG)
else
    CFLAGS += $(CFLAGS_RELEASE)
endif

# Default target
all: $(LIB_NAME) $(TEST_EXEC) 

# Build CFSM Static Library
$(LIB_NAME): $(LIB_OBJ)
	ar rcs $(LIB_NAME) $(LIB_OBJ)

# Compile Source Files
%.o: %.c
	$(CC) $(CFLAGS) $(INCLUDE_DIRS) -c $< -o $@

# Build Unity (test framework)
$(UNITY_OBJ): $(UNITY_SRC)
	$(CC) $(CFLAGS) $(INCLUDE_DIRS) -c $< -o $@

# Build Test Executable
$(TEST_EXEC): $(TEST_OBJ) $(LIB_NAME) $(UNITY_OBJ)
	$(CC) $(CFLAGS) -o $(TEST_EXEC) $(TEST_OBJ) $(LIB_NAME) $(UNITY_OBJ) -pthread -lm

# Run Tests
test: $(TEST_EXEC)
	@echo "Running tests..."
	./$(TEST_EXEC)

# Build Mario Example
# $(MARIO_EXEC): $(MARIO_OBJ) $(LIB_NAME)
# 	$(CC) $(CFLAGS) -o $(MARIO_EXEC) $(MARIO_OBJ) $(LIB_NAME) -pthread -lm

# Clean Build
clean:
	rm -f $(LIB_OBJ) $(LIB_NAME) $(TEST_OBJ) $(TEST_EXEC) $(MARIO_OBJ) $(MARIO_EXEC) $(UNITY_OBJ)

.PHONY: all clean test
