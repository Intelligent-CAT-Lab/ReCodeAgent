TEST_SRCS := test_001.c test_002.c
TESTS := ${TEST_SRCS:%.c=%}
RUN_TESTS := ${TESTS:%=run_%}

.PHONY = all clean $(RUN_TESTS)
default: all

CFLAGS = -D_FILE_OFFSET_BITS=64 -D_LARGEFILE_SOURCE -O0 -g

all: $(RUN_TESTS) $(TESTS)

test_% : test_%.c
	$(CC) $(CFLAGS) $< ../ljmm.o -Wl,--wrap=mmap64 -o $@

run_test_% : test_%
	./$<

clean:
	rm -f *.o $(TESTS)
