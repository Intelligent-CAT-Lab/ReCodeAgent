BIN = ./bin
OUT = ./emlang
TEST_OUT= ./test_runner
SRC  = $(wildcard src/*.c)
DEPS = $(wildcard src/*.h)
OBJ  = $(addsuffix .o,$(subst src/,$(BIN)/,$(basename $(SRC))))
TEST_SRC = tests/test.c
TEST_OBJ = $(BIN)/test.o
OBJ_NO_MAIN = $(filter-out $(BIN)/main.o, $(OBJ))
CSTD   = c99
CC     = gcc
CFLAGS = -O2 -std=$(CSTD) -Wall -Wextra -Werror -pedantic -Wno-deprecated-declarations -g

$(OUT): $(BIN) $(OBJ) $(SRC)
	$(CC) $(CFLAGS) -o $(OUT) $(OBJ)

$(BIN)/%.o: src/%.c $(DEPS)
	$(CC) -c $< $(CFLAGS) -o $@

$(BIN):
	mkdir -p $(BIN)

clean:
	rm $(OUT)
	rm -r $(BIN)/*.o

$(TEST_OUT): $(BIN) $(OBJ_NO_MAIN) $(TEST_SRC)
	$(CC) $(CFLAGS) -o $(TEST_OUT) $(OBJ_NO_MAIN) $(TEST_SRC)

test: $(TEST_OUT)
	./$(TEST_OUT)