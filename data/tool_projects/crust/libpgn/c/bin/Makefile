EXE     = pgn

CC      = gcc
CFLAGS  = -std=c99 -Wall -Wextra

LD      = gcc
LDLIBS  = -lpgn

SRC    := $(wildcard *.c)
DEPS   := $(wildcard *.h)
OBJ    := $(SRC:.c=.o)

all: $(EXE)

$(EXE): $(OBJ)
	$(LD) -Wl,-rpath=$(shell pwd)/.. -L.. $^ $(LDLIBS) -o $(EXE)

%.o: %.c $(DEPS)
	$(CC) $(CFLAGS) -I.. -c $< -o $@

clean:
	$(RM) $(OBJ) $(EXE)

.PHONY: $(EXE) clean
