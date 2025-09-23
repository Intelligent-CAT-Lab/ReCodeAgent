CC = gcc
CFLAGS = -Wall -Wextra -g -fprofile-arcs -ftest-coverage
LDFLAGS = --coverage 

SRC = src/cJSON.c
HEADERS = src/cJSON.h
TESTS = tests/main.c

OBJS = $(SRC:.c=.o)
TEST_OBJS = $(TESTS:.c=.o)
EXEC = test_runner

.PHONY: all clean coverage

all: $(EXEC)

$(EXEC): $(OBJS) $(TEST_OBJS)
	$(CC) $(CFLAGS) $(LDFLAGS) $^ -o $@ -lm

%.o: %.c $(HEADERS)
	$(CC) $(CFLAGS) $(LDFLAGS) -c $< -o $@ -lm 

test: $(EXEC)
	./$(EXEC)

coverage: run-tests
	gcov $(SRC) $(TESTS)
	lcov --capture --directory . --output-file coverage.info
	genhtml coverage.info --output-directory coverage_report
	@echo "Coverage report generated in ./coverage_report/index.html"

clean:
	rm -f $(OBJS) $(TEST_OBJS) $(EXEC) *.gcno *.gcda *.gcov coverage.info
	rm -rf coverage_report
