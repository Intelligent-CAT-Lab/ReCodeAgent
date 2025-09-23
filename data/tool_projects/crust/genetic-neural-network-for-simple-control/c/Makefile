# Compiler and flags
CC = gcc
CFLAGS = -g -Wall -I include/toolbox -I test/include  -pg
LIBS += -lm

# Source and object files
MAIN_SRC = ./src/main.c          # Main source file
TEST_SRC = ./test/test.c         # Test source file
MAIN_OBJ = $(MAIN_SRC:.c=.o)
TEST_OBJ = $(TEST_SRC:.c=.o)

# Automatically find all .c files in src and test directories
SRC_DIR_C  = ./src/toolbox       # Directory for source files
TST_DIR_C = ./test/tests         # Directory for test files

INCL_DIR_H = ./include           # Directory with header files for src
TEST_DIR_H = ./test/include      # Directory with header files for test

# Find all .c and .h files in subdirectories
C_FILES := $(shell find $(SRC_DIR_C)  -type f -name '*.c') \
           $(shell find $(TST_DIR_C)  -type f -name '*.c')
H_FILES := $(shell find $(TEST_DIR_H) -type f -name '*.h') \
           $(shell find $(INCL_DIR_H) -type f -name '*.h')

OBJ_FILES := $(C_FILES:.c=.o)   # Object files from source files

# Executable name
EXECUTABLE     = myprogram
TESTEXECUTABLE = testprogram 

all: $(EXECUTABLE) $(TESTEXECUTABLE) clean

$(EXECUTABLE): $(MAIN_OBJ) $(filter-out $(TEST_OBJ), $(C_FILES:.c=.o))
	$(CC) $(CFLAGS) -o $@ $^ $(LIBS)

$(MAIN_OBJ): $(MAIN_SRC) $(H_FILES)
	$(CC) $(CFLAGS) -c -o $@ $<

$(TESTEXECUTABLE): $(TEST_OBJ) $(filter-out $(MAIN_OBJ), $(C_FILES:.c=.o))
	$(CC) $(CFLAGS) -o $@ $^ $(LIBS)

$(TEST_OBJ): $(TEST_SRC) $(H_FILES)
	$(CC) $(CFLAGS) -c -o $@ $<

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<
	
# Clean up object files and other generated files
clean:
	find . -name \*.o -type f -delete
	find . -name \*Zone.Identifier -type f -delete
	find . -name \*.sig -type f -delete
	find . -name \gmon.out -type f -delete
