CC = gcc
CFLAG = -Wall -Wextra -Wpedantic -std=c2x -g -fprofile-arcs -ftest-coverage -fPIC
DIR = src
OBJ = dict.o
LIB = 
POST_FIX = 
ELF_FILES = 

ifeq ($(OS),Windows_NT)
	POST_FIX = dll
	LIB += -L. -ldict
else
	UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        CFLAG += -Wno-unused-result
		POST_FIX = so
		CFLAG += -Wl,-rpath=./
		LIB += -L. -ldict
		ELF_FILES := $(shell find . -type f -executable -exec sh -c 'file -b {} | grep -q ELF' \; -print)
    endif
endif

all: dict
.PHONY: dict

dict: $(DIR)/dict.c $(DIR)/dict.h
	$(CC) $(CFLAG) -fPIC -shared $< -o lib$@.$(POST_FIX)

test%: tests/test%.c
	$(CC) $(CFLAG) $< -o test_dict $(LIB)
	./test_dict
test: test0 test1 test2 test3 test4 test5

clean:
	rm -f *.dll *.exe *.o *.bin $(ELF_FILES)
	rm -rf *.gcno *.gcda *.gcov
