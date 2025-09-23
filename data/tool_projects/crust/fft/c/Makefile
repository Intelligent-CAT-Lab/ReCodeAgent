CC = gcc
OPTIMIZE = -O3
DEBUG = -g
AR = ar rcs
LDLIBS += -lm
CFLAGS = $(OPTIMIZE) $(DEBUG) -fprofile-arcs -ftest-coverage -fPIC

all: libfft.a

libfft.a: fft.o
	$(AR) $@ $^

fft.o: src/fft.c src/fft.h
	$(CC) -c $< -o $@ $(CFLAGS)


test: tests/test.c libfft.a
	$(CC) $< -o test_1 $(CFLAGS) -L . -lfft -lm
	./test_1
	


.PHONY: clean

clean:
	$(RM) fft.o libfft.a *.exe
