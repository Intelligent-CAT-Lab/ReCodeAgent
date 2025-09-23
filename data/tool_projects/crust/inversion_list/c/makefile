# Compiler and flags
CC = gcc
CFLAGS = -Wall -Wextra -g -fPIC -Wno-unused-variable -Wno-unused-function -fprofile-arcs -ftest-coverage -fPIC
LDFLAGS = -shared
PREFIX = /usr/local
LIBDIR = $(PREFIX)/lib
INCLUDEDIR = $(PREFIX)/include
PKGCONFIGDIR = $(PREFIX)/share/pkgconfig

# Source files and targets
SRCDIR = src/inversion-list
OBJDIR = build
SOURCES = $(SRCDIR)/inversion-list.c
OBJECTS = $(OBJDIR)/inversion-list.o
TARGET = libinversion-list.so

# Directories
SUBDIRS = src test docs

all: $(TARGET)

$(OBJDIR):
	mkdir -p $(OBJDIR)

$(OBJECTS): $(SOURCES) | $(OBJDIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(TARGET): $(OBJECTS)
	$(CC) $(LDFLAGS) $^ -o $@

install: $(TARGET)
	install -D $(TARGET) $(DESTDIR)$(LIBDIR)/$(TARGET)
	install -D inversion-list.h $(DESTDIR)$(INCLUDEDIR)/inversion-list.h
	install -D inversion-list.pc $(DESTDIR)$(PKGCONFIGDIR)/inversion-list.pc

uninstall:
	rm -f $(DESTDIR)$(LIBDIR)/$(TARGET)
	rm -f $(DESTDIR)$(INCLUDEDIR)/inversion-list.h
	rm -f $(DESTDIR)$(PKGCONFIGDIR)/inversion-list.pc


check:
	@echo "Running static analysis..."
	cppcheck --enable=all --inconclusive --quiet $(SRCDIR)
	cclint $(SRCDIR)
	flawfinder $(SRCDIR)

clean:
	rm -rf $(OBJDIR) $(TARGET)
	cd tests && $(MAKE) clean

# Find Valgrind
VALGRIND = valgrind

# Find test source files
TEST_SRC := $(wildcard tests/inversion-list/test-*.c)
TEST_EXES := $(TEST_SRC:.c=)

# Build test executables
$(TEST_EXES): %: %.c
	$(CC) $(CFLAGS) $< -o $@ $(TARGET)

# Run tests with Valgrind
test: $(TEST_EXES)
	@for test in $(TEST_EXES); do \
		echo "Running $$test with Valgrind..."; \
		$(VALGRIND) --leak-check=full ./$$test; \
	done

.PHONY: all install uninstall test check clean 

