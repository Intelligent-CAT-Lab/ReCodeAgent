ALL=ulidgen

CFLAGS=-g -O2 -Wall -Wno-switch -Wextra -Wwrite-strings -fprofile-arcs -ftest-coverage -fPIC

DESTDIR=
PREFIX=/usr/local
BINDIR=$(PREFIX)/bin
MANDIR=$(PREFIX)/share/man

all: ulid.o test


ulid.o: src/ulid.c src/ulid.h
	

test: 
	gcc $(CFLAGS) -o test_1 tests/test.c src/ulid.c
	./test_1
clean: FRC
	rm -f $(ALL) *.o
	rm -f test_1
	rm -rf *.gcno *.gcda *.gcov


install: FRC all
	mkdir -p $(DESTDIR)$(BINDIR) $(DESTDIR)$(MANDIR)/man1 $(DESTDIR)$(ZSHCOMPDIR)
	install -m0755 $(ALL) $(DESTDIR)$(BINDIR)
	install -m0644 $(ALL:=.1) $(DESTDIR)$(MANDIR)/man1

README: ulidgen.1
	mandoc -Tutf8 $< | col -bx >$@

FRC:
