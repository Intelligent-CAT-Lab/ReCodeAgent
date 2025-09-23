# CFLAGS=--std=gnu99 -Wall -O3

# blt_test: testblt_test.c blt.c

# blt_bm: blt_bm.c blt.c bm.c
# 	$(CC) $(CFLAGS) -o $@ $^ -ltcmalloc

# cbt_bm: cbt_bm.c cbt.c bm.c
# 	$(CC) $(CFLAGS) -o $@ $^ -ltcmalloc

# push:
# 	git push git@github.com:blynn/blt.git master
# 	git push https://code.google.com/p/blynn-blt/ master
CFLAGS=--std=gnu99 -Wall -O3 -fprofile-arcs -ftest-coverage
LDFLAGS=-lgcov --coverage
CC=gcc

SRC_DIR=src
TESTS_DIR=tests
OBJ_DIR=.

SRC_FILES=$(wildcard $(SRC_DIR)/*.c)
TEST_FILES=$(wildcard $(TESTS_DIR)/*.c)

SRC_OBJ_FILES=$(patsubst $(SRC_DIR)/%.c, $(OBJ_DIR)/%.o, $(SRC_FILES))
TEST_OBJ_FILES=$(patsubst $(TESTS_DIR)/%.c, $(OBJ_DIR)/%.o, $(TEST_FILES))

# Targets for building executables
blt_test: $(SRC_OBJ_FILES) $(OBJ_DIR)/blt_test.o
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)
	./$@

blt_bm: $(SRC_OBJ_FILES) $(OBJ_DIR)/blt_bm.o $(OBJ_DIR)/bm.o
	$(CC) $(CFLAGS) -o $@ $^ -ltcmalloc $(LDFLAGS)

# cbt_bm: $(SRC_OBJ_FILES) $(OBJ_DIR)/cbt_bm.o $(OBJ_DIR)/cbt.o $(OBJ_DIR)/bm.o
# 	$(CC) $(CFLAGS) -o $@ $^ -ltcmalloc $(LDFLAGS)

# Rules to compile source and test files into object files in the current directory
$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) -c -o $@ $<

$(OBJ_DIR)/%.o: $(TESTS_DIR)/%.c
	$(CC) $(CFLAGS) -c -o $@ $<

# Coverage Target
coverage: blt_test
	./blt_test
	gcov -o . $(SRC_FILES)
	lcov --capture --directory . --output-file coverage.info
	genhtml coverage.info --output-directory coverage_report
	@echo "Coverage report generated in coverage_report/index.html"

# Cleanup
clean:
	rm -f blt_test blt_bm *.o *.gcda *.gcno *.gcov coverage.info
	rm -rf coverage_report
	rm -rf cbt_bm

# Push changes
push:
	git push git@github.com:blynn/blt.git master
	git push https://code.google.com/p/blynn-blt/ master
