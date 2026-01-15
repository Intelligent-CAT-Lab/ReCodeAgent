HEADERS = xopt.h
OBJECTS = xopt.o

.PHONY: all clean test

all: libxopt.a
	$(MAKE) -C test

%.o: %.c $(HEADERS)
	$(CC) -std=c99 -Wall -Wextra $(CFLAGS) $(DEFINES) -c $< -o $@

libxopt.a: $(OBJECTS)
	ar rcs $@ $(OBJECTS)

test: all
	./test/simple-test
	./test/macro-test

clean:
	-rm -f $(OBJECTS)
	-rm -f libxopt.a
	$(MAKE) -C test clean
