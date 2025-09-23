prefix ?= /usr
CCFLAGS = -O3 -Wextra -Wall -fprofile-arcs -ftest-coverage -fPIC

hydra.o: src/hydra.c src/hydra.h
	$(CC) $(CCFLAGS) -c src/hydra.c

test: hydra.o tests/test.c
	$(CC) $(CCFLAGS) -o test_1 tests/test.c hydra.o -lpthread
	./test_1

.PHONY: run, run-test, install, clean

run: hydra
	./hydra hydras/git

run-test: test
	./test

install: hydra
	install -Dm755 hydra $(DESTDIR)$(prefix)/bin/hydra
	install -Dm755 hydra-completion.bash $(DESTDIR)$(prefix)/share/hydra/hydra-completion.bash
	cp --recursive --force hydras $(DESTDIR)$(prefix)/share/hydra/

clean:
	rm -f *.o hydra test
