DESTDIR   ?=
PREFIX    ?= /usr/local
MANPREFIX ?= $(PREFIX)/man

CFLAGS += -Wall -Wextra -fPIC -fprofile-arcs -ftest-coverage

all: leftpad libleftpad.so libleftpad.a

check: leftpad tests
	LD_LIBRARY_PATH=. ./leftpad test 10 >/dev/null
	LD_LIBRARY_PATH=. ./tests

clean:
	rm -f leftpad tests libleftpad.so libleftpad.a
	rm -f leftpad.o main.o tests.o
	rm -rf *.gcno *.gcda *.gcov

install: all
	install -d $(DESTDIR)$(PREFIX)/bin
	install -d $(DESTDIR)$(PREFIX)/lib
	install -d $(DESTDIR)$(PREFIX)/include
	install -d $(DESTDIR)$(MANPREFIX)/man1
	install -d $(DESTDIR)$(MANPREFIX)/man3
	install -m755 leftpad       $(DESTDIR)$(PREFIX)/bin
	install -m755 libleftpad.so $(DESTDIR)$(PREFIX)/lib
	install -m644 libleftpad.a  $(DESTDIR)$(PREFIX)/lib
	install -m644 src/leftpad.h $(DESTDIR)$(PREFIX)/include
	install -m644 leftpad.1     $(DESTDIR)$(MANPREFIX)/man1
	install -m644 leftpad.3     $(DESTDIR)$(MANPREFIX)/man3

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/leftpad
	rm -f $(DESTDIR)$(PREFIX)/lib/libleftpad.so
	rm -f $(DESTDIR)$(PREFIX)/lib/libleftpad.a
	rm -f $(DESTDIR)$(PREFIX)/include/leftpad.h
	rm -f $(DESTDIR)$(MANPREFIX)/man1/leftpad.1
	rm -f $(DESTDIR)$(MANPREFIX)/man3/leftpad.3

# Build the leftpad binary by linking main.o against the shared library.
leftpad: main.o libleftpad.so
	$(LINK.c) -Wl,-rpath=. -L. -o $@ main.o -lleftpad

# Build the shared library.
libleftpad.so: leftpad.o
	$(LINK.c) -shared -nostdlib -o $@ leftpad.o

# Build the static library.
libleftpad.a: leftpad.o
	$(AR) $(ARFLAGS) $@ leftpad.o

# Compile the object for leftpad from src/leftpad.c
leftpad.o: src/leftpad.c src/leftpad.h
	$(COMPILE.c) -fPIC -o $@ src/leftpad.c

# Build the tests binary by linking tests.o against the shared library.
tests: tests.o libleftpad.so
	$(LINK.c) -Wl,-rpath=. -L. -o $@ tests.o -lleftpad

# Compile main.o from bin/main.c
main.o: bin/main.c src/leftpad.h
	$(COMPILE.c) -o $@ bin/main.c

# Compile tests.o from tests.c (assuming tests.c in top-level directory)
tests.o: tests.c src/leftpad.h
	$(COMPILE.c) -o $@ tests.c

.PHONY: all check clean install uninstall
