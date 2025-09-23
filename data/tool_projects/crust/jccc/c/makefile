CC := gcc
CFLAGS := -O0 -Wall -Wpedantic -g -Wno-gnu-zero-variadic-macro-arguments -std=c11 -fprofile-arcs -ftest-coverage -fPIC
INCLUDE := -I src

# Find all source files recursively, excluding main drivers
SOURCES := $(shell find src -type f -name "*.c" ! -path "src/driver/main.c" ! -path "src/testing/main.c")
TEST_SOURCES := $(shell find src -type f -name "test_*.c")

# Define targets
JCCC := jccc
test_JCCC := test_jccc

all: $(JCCC) $(test_JCCC)
	./$(test_JCCC)
$(JCCC): $(SOURCES) src/driver/main.c
	$(CC) $(CFLAGS) $(INCLUDE) $^ -o $@

$(test_JCCC): $(SOURCES) $(TEST_SOURCES) src/testing/main.c
	$(CC) $(CFLAGS) $(INCLUDE) $^ -o $@

test: $(test_JCCC)
	./$(test_JCCC)

clean:
	rm -f $(JCCC) $(test_JCCC)
	rm -f *.gcno *.gcda *.gcov
