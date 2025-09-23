.PHONY: all clean test
default: all

TARGET = ljmm_objs.o
TARGET_STRESS = ljmm_objs_stress.o
SRC_FILE = ljmm.c
TEST_PROGRAM = ljmm_test

OPT_FLAGS = -O3 -g
CFLAGS = -MMD -Wall -Werror -DDEBUG $(OPT_FLAGS) -D_FILE_OFFSET_BITS=64 -D_LARGEFILE_SOURCE

CC = gcc
AR = ar
LD = ld

C_SRCS := ljmm.c
C_OBJS := ${C_SRCS:%.c=%.o}
STRESS_C_OBJS := $(patsubst ljmm.o, ljmm_stress.o, $(C_OBJS))

all : $(TARGET) $(TARGET_STRESS)

$(TARGET) : $(C_OBJS)
	$(LD) $^ -r -o $@
	cat *.d > ar_dep.txt

$(TARGET_STRESS) : $(STRESS_C_OBJS)
	$(LD) $^ -r -o $@
	cat *.d > ar_dep2.txt

ljmm_stress.o : ljmm.c
	$(CC) $(CFLAGS) -DSTRESS_TEST -c $< -o $@

%.o : %.c
	$(CC) $(CFLAGS) -c $<

-include ar_dep.txt
-include ar_dep2.txt

clean:
	rm -f $(AR_NAME) $(TEST_PROGRAM) *.o *.d *_dep.txt ar_dep2.txt
	make clean -C test

$(TEST_PROGRAM) : $(TARGET) ljmm_test.o
	$(CC) $(CFLAGS) $+ -Wl,--wrap=mmap64 -o $@

test: $(TARGET) $(TARGET_STRESS)
	make -C test
