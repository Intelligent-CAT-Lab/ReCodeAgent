# CFLAGS=-g -std=c99 -O2 -Wall -Wextra -Wfloat-equal -Wpointer-arith -Wcast-align -Wstrict-overflow=5 -Wwrite-strings -Wunreachable-code -Wno-unused-parameter -Wpedantic 
# override LDFLAGS+= -L.
# override CFLAGS+= -fPIC -I.
# TESTS=$(patsubst tests/test_helpers,,$(patsubst %.c,%,$(wildcard tests/*.c)))

# .PHONY: all
# all: libbostree.so libbostree.a $(TESTS)

# .PHONY: test
# test: $(TESTS)
# 	@for test in $(TESTS); do \
# 		echo "--- Starting: $$test"; \
# 		$$test; \
# 		if [ "$$?" != "0" ]; then \
# 			echo; \
# 			echo "FAILED"; \
# 			exit 1; \
# 		else \
# 			echo "--- $$test passed"; \
# 			echo; \
# 		fi; \
# 	done; \
# 	echo; \
# 	echo "All tests passed."

# libbostree.so: bostree.o
# 	$(LD) -shared -o $@ $+
# 	ln -s ../$@ tests/ || true

# libbostree.a: bostree.o
# 	$(AR) rcs $@ $+
# 	ranlib $@

# tests/test_helpers.o: tests/test_helpers.c
# 	$(CC) -c $(CFLAGS) -o $@ $+

# %: %.c tests/test_helpers.o libbostree.so
# 	$(CC) $(CFLAGS) -o $@ $< tests/test_helpers.o $(LDFLAGS) -Wl,-rpath=. -lbostree

# .PHONY: clean

# clean:
# 	rm -f *.o bos *.so tests/*.so tests/*.o libbostree.a $(TESTS)

CFLAGS=-g -std=c99 -O2 -Wall -Wextra -Wfloat-equal -Wpointer-arith -Wcast-align -Wstrict-overflow=5 -Wwrite-strings -Wunreachable-code -Wno-unused-parameter -Wpedantic 
override CFLAGS+= -I. -Itests  # Add `-I.` to include the current directory

TESTS=$(patsubst tests/test_helpers,,$(patsubst %.c,%,$(wildcard tests/*.c)))
SRCS=$(wildcard *.c)  # All source files in the main directory
OBJS=$(SRCS:.c=.o)    # Object files

# Coverage flags
COVERAGE_FLAGS=-fprofile-arcs -ftest-coverage
override CFLAGS+= $(COVERAGE_FLAGS)
override LDFLAGS+= --coverage -lgcov

.PHONY: all
all: $(TESTS)
	./$(TESTS)

.PHONY: test
test: $(TESTS)
	@for test in $(TESTS); do \
		echo "--- Starting: $$test"; \
		$$test; \
		if [ "$$?" != "0" ]; then \
			echo; \
			echo "FAILED"; \
			exit 1; \
		else \
			echo "--- $$test passed"; \
			echo; \
		fi; \
	done; \
	echo; \
	echo "All tests passed."

# Compile object files with coverage
%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

tests/test_helpers.o: tests/test_helpers.c
	$(CC) $(CFLAGS) -c -o $@ $<

# Compile tests with coverage enabled
tests/%: tests/%.c tests/test_helpers.o $(OBJS)
	$(CC) $(CFLAGS) -o $@ $< tests/test_helpers.o $(OBJS) $(LDFLAGS)

.PHONY: coverage
coverage: test
	gcov -o . $(SRCS)
	lcov --capture --directory . --output-file coverage.info
	genhtml coverage.info --output-directory coverage_report
	@echo "Coverage report generated in coverage_report/index.html"

.PHONY: clean
clean:
	rm -f *.o tests/*.o $(TESTS) *.gcda *.gcno coverage.info
	rm -rf coverage_report

