CC ?= gcc
AR ?= ar
RANLIB ?= ranlib
CFLAGS ?= -g -Wall -O3
LIBS =
EXTRA_CFLAGS_PIC = -fpic
LDFLAGS =
LDLIBS =
INCLUDES = 

# Coverage flags
COVERAGE_FLAGS = -fprofile-arcs -ftest-coverage
LDFLAGS += --coverage
CFLAGS += $(COVERAGE_FLAGS)

prefix = ${DESTDIR}/usr/local
includedir = $(prefix)/include
libdir = $(exec_prefix)/lib

.PHONY: all clean lib test doc coverage

.SUFFIXES: .c .o .pico

OBJS = 2bit.o

all: lib

lib: lib-static lib-shared

lib-static: lib2bit.a

lib-shared: lib2bit.so

doc:
	doxygen

.c.o:   
	$(CC) -I. $(CFLAGS) $(INCLUDES) -c -o $@ $<

.c.pico:
	$(CC) -I. $(CFLAGS) $(INCLUDES) $(EXTRA_CFLAGS_PIC) -c -o $@ $<

lib2bit.a: $(OBJS)
	-@rm -f $@
	$(AR) -rcs $@ $(OBJS)
	$(RANLIB) $@

lib2bit.so: $(OBJS:.o=.pico)
	$(CC) -shared $(LDFLAGS) -o $@ $(OBJS:.o=.pico) $(LDLIBS) $(LIBS)

test/exampleRead: lib2bit.so
	$(CC) -o $@ -I. -L. $(CFLAGS) test/exampleRead.c -l2bit $(LIBS) -Wl,-rpath .

test: test/exampleRead
	./test/test.py

# Run coverage analysis
coverage: test
	lcov --capture --directory . --output-file coverage.info
	genhtml coverage.info --output-directory coverage_report
	@echo "Coverage report generated in coverage_report/index.html"

clean:
	rm -f *.o *.a *.so *.pico test/exampleRead *.gcda *.gcno coverage.info
	rm -rf coverage_report

install: lib2bit.a lib2bit.so
	install -d $(prefix)/lib $(prefix)/include
	install lib2bit.a $(prefix)/lib
	install lib2bit.so $(prefix)/lib
	install *.h $(prefix)/include
