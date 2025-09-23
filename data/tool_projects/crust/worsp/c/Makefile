MAIN_SRC_FILES := src/worsp.c bin/main.c
EXECUTABLE_MAIN := main
DEBUGGABLE_MAIN := debug_main

TEST_SRC_FILES := src/worsp.c tests/test.c
EXECUTABLE_TEST := test_1

REPL_SRC_FILES := src/worsp.c bin/repl.c
EXECUTABLE_REPL := repl

EXECUTABLE_SNAPSHOT_TEST := ./snapshot/snapshot.sh

CC := gcc
CFLAGS := -Wall -Wextra -fprofile-arcs -ftest-coverage -fPIC

CLANG_FORMAT := clang-format
FORMAT_FILES := $(wildcard *.c) $(wildcard *.h)

LLDB := lldb

BASH := bash

.PHONY: format clean test run-repl run-main lldb-main check-snapshot update-snapshot

$(EXECUTABLE_MAIN): $(MAIN_SRC_FILES)
	$(CC) $(CFLAGS) $(MAIN_SRC_FILES) -o $(EXECUTABLE_MAIN) -lm

$(DEBUGGABLE_MAIN): $(MAIN_SRC_FILES)
	$(CC) $(CFLAGS) -g $(MAIN_SRC_FILES) -o $(DEBUGGABLE_MAIN) -lm

$(EXECUTABLE_TEST): $(TEST_SRC_FILES)
	$(CC) $(CFLAGS) $(TEST_SRC_FILES) -o $(EXECUTABLE_TEST) -lm

$(EXECUTABLE_REPL): $(REPL_SRC_FILES)
	$(CC) $(CFLAGS) $(REPL_SRC_FILES) -o $(EXECUTABLE_REPL) -lm

run-main: $(EXECUTABLE_MAIN)
	./$(EXECUTABLE_MAIN) $(WORSP_FILE)

lldb-main: $(DEBUGGABLE_MAIN)
	$(LLDB) $(DEBUGGABLE_MAIN) $(WORSP_FILE)

test: $(EXECUTABLE_TEST)
	./$(EXECUTABLE_TEST)

check-snapshot: $(EXECUTABLE_MAIN)
	$(BASH) $(EXECUTABLE_SNAPSHOT_TEST)

update-snapshot: $(EXECUTABLE_MAIN)
	$(BASH) $(EXECUTABLE_SNAPSHOT_TEST) -u

run-repl: $(EXECUTABLE_REPL)
	./$(EXECUTABLE_REPL)

format:
	$(CLANG_FORMAT) -i $(FORMAT_FILES)

clean:
	rm -f $(EXECUTABLE_MAIN) $(EXECUTABLE_TEST) $(EXECUTABLE_REPL)
	rm -f *.gcno *.gcda *.gcov
	
