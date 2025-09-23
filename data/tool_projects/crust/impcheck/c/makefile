CC = gcc
CFLAGS = -std=gnu99 -Wall -Wextra -Werror -pedantic-errors -flto -g -fprofile-arcs -ftest-coverage -fPIC

ifdef IMPCHECK_WRITE_DIRECTIVES
    CFLAGS += -DIMPCHECK_WRITE_DIRECTIVES=$(IMPCHECK_WRITE_DIRECTIVES)
endif

ifdef IMPCHECK_FLUSH_ALWAYS
    CFLAGS += -DIMPCHECK_FLUSH_ALWAYS=$(IMPCHECK_FLUSH_ALWAYS)
endif

SRCDIR = src/trusted
TESTDIR = test
OBJDIR = obj

SOURCES_PARSE = $(SRCDIR)/hash.c $(SRCDIR)/secret.c $(SRCDIR)/siphash.c \
                $(SRCDIR)/trusted_parser.c $(SRCDIR)/trusted_utils.c $(SRCDIR)/vectors.c \
                src/writer.c $(SRCDIR)/main_parse.c

SOURCES_CHECK = $(SRCDIR)/confirm.c $(SRCDIR)/hash.c $(SRCDIR)/lrat_check.c \
                $(SRCDIR)/secret.c $(SRCDIR)/siphash.c $(SRCDIR)/trusted_checker.c \
                $(SRCDIR)/top_check.c $(SRCDIR)/trusted_utils.c $(SRCDIR)/vectors.c \
                src/writer.c $(SRCDIR)/main_check.c

SOURCES_CONFIRM = $(SRCDIR)/confirm.c $(SRCDIR)/hash.c $(SRCDIR)/secret.c \
                  $(SRCDIR)/siphash.c $(SRCDIR)/trusted_parser.c $(SRCDIR)/trusted_utils.c \
                  $(SRCDIR)/vectors.c src/writer.c $(SRCDIR)/main_confirm.c

SOURCES_TEST_HASH = $(SRCDIR)/trusted_utils.c $(SRCDIR)/hash.c src/writer.c \
                    $(TESTDIR)/test.c $(TESTDIR)/test_hash.c

SOURCES_TEST_FULL = $(SRCDIR)/trusted_utils.c src/writer.c $(TESTDIR)/test.c \
                    $(SRCDIR)/vectors.c $(TESTDIR)/test_full.c

OBJS_PARSE = $(SOURCES_PARSE:.c=.o)
OBJS_CHECK = $(SOURCES_CHECK:.c=.o)
OBJS_CONFIRM = $(SOURCES_CONFIRM:.c=.o)
OBJS_TEST_HASH = $(SOURCES_TEST_HASH:.c=.o)
OBJS_TEST_FULL = $(SOURCES_TEST_FULL:.c=.o)

all: impcheck_parse impcheck_check impcheck_confirm test_hash test_full
	./test_full_1

impcheck_parse: $(OBJS_PARSE)
	$(CC) $(CFLAGS) -o $@ $^

impcheck_check: $(OBJS_CHECK)
	$(CC) $(CFLAGS) -o $@ $^

impcheck_confirm: $(OBJS_CONFIRM)
	$(CC) $(CFLAGS) -o $@ $^

test_hash: $(OBJS_TEST_HASH)
	$(CC) $(CFLAGS) -o $@ $^

test_full: $(OBJS_TEST_FULL)
	$(CC) $(CFLAGS) -o test_full_1 $^

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f $(OBJS_PARSE) $(OBJS_CHECK) $(OBJS_CONFIRM) $(OBJS_TEST_HASH) $(OBJS_TEST_FULL) \
	       impcheck_parse impcheck_check impcheck_confirm test_hash test_full *.gcda *.gcno *.gcov
