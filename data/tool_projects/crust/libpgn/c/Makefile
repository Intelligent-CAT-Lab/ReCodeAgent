LIB           = libpgn
EXT           = .so

MAN3          = $(LIB).3

CC            = gcc
CFLAGS        = -std=c99 -pedantic-errors -Wall -Wextra 

LD            = ld
LDLIBS        = 
LDFLAGS       =

INST          = /usr

SRC          := $(wildcard *.c utils/*.c)
DEPS         := $(SRC:.c=.h)
OBJ          := $(SRC:.c=.o)

TESTS        := $(wildcard tests/*.c)
EXE          := $(TESTS:.c=)

ifeq ($(OS), Windows_NT)
	LD  := $(CC)
	EXT := .dll
endif

all: $(OBJ)
	$(LD) $(LDLIBS) $(LDFLAGS) -shared $^ -o $(LIB)$(EXT)
	ar rcs $(LIB).a $^

test: all $(EXE)

bin: all
	$(MAKE) -C $(MAKECMDGOALS) 

$(EXE):
	$(CC) $@.c -lpgn -Wl,-rpath=. -L. -I. -o $@
	./$@
	$(RM) $@

install: all
	mkdir -p $(INST)/include/pgn
	cp *.h $(INST)/include/pgn
	mkdir -p $(INST)/include/pgn/utils
	cp utils/*.h $(INST)/include/pgn/utils
	sed -i $(INST)/include/pgn/pgn.h -e "s/#include \"/#include \"pgn\//g"
	mv $(INST)/include/pgn/pgn.h $(INST)/include
	cp $(LIB)$(EXT) $(INST)/lib
	mkdir -p $(INST)/share/man/man3
	gzip -k $(MAN3)
	mv $(MAN3).gz $(INST)/share/man/man3

uninstall:
	$(RM) $(INST)/lib/$(LIB)$(EXT)
	$(RM) -r $(INST)/include/pgn
	$(RM) $(INST)/include/pgn.h
	$(RM) $(INST)/share/man/man3/$(MAN3).gz

# Assuming every file
# depens on header DEPS
%.o: %.c $(DEPS)
	$(CC) $(CFLAGS) -fPIC -c $< -o $@

clean:
	$(RM) $(OBJ) $(LIB)$(EXT) $(LIB).a

.PHONY: clean all $(LIB) install uninstall test bin
