GCC = gcc
CFLAGS = -g
OBJECTS = simple_vector.o simple_xml.o
TESTPRG = test

all: test

# Deps
simple_vector.o: simple_vector.h
simple_xml.o: simple_vector.o simple_xml.h

%.o: %.c
	$(GCC) $(CFLAGS) -c $<

test: test.c $(OBJECTS)
	$(GCC) $(CFLAGS) -c test.c
	$(GCC) $(OBJECTS) test.o -o $(TESTPRG)

clean:
	rm -rf *.o $(TESTPRG)
