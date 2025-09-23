.PHONY: all test install clean

prefix = /usr/local
exec_prefix = $(prefix)
libdir = $(exec_prefix)/lib
includedir = $(prefix)/include

CC = cc
INSTALL = install

ALL =  chtrie.o

all: $(ALL)

ALL: chtrie.o test

chtrie.o: src/chtrie.c src/chtrie.h
	$(CC) $(CFLAGS) -fprofile-arcs -ftest-coverage -fPIC -c -o src/$@ $<

# libchtrie.a: src/chtrie.o
# 	ar -cr $@ $^

run.tmp: src/chtrie.o src/chtrie.h tests/test.c
	$(CC) $(CFLAGS) -fprofile-arcs -ftest-coverage -fPIC -o $@ src/chtrie.o tests/test.c

test: run.tmp
	./run.tmp

install: all
	$(INSTALL) -d $(includedir) $(libdir)
	$(INSTALL) src/chtrie.h $(includedir)/
	$(INSTALL) libchtrie.a $(libdir)/

clean:
	rm -rf $(ALL) *.tmp
	rm -rf src/*.gcno src/*.gcda src/*.gcov
	rm -rf tests/*.gcno tests/*.gcda tests/*.gcov
