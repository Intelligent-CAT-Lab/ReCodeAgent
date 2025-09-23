CC=clang
#CC=gcc -pg
CFLAGS=-Wall -Wextra -pedantic -O2 -std=c99 -g

holdemodds: build
	${CC} ${CFLAGS} \
		src/cards.o \
		src/hands.o \
		src/deck.o  \
		src/bucket.o \
		src/holdemodds.c -o holdemodds

build: src/cards.o src/hands.o src/bucket.o src/deck.o


clean:
	rm -f holdemodds src/*.o src/tests/*.o src/tests/all_tests


test: build
	${CC} ${CFLAGS} -Isrc -Isrc/tests -o src/tests/all_tests \
		src/cards.o \
		src/hands.o \
		src/bucket.o \
		src/deck.o \
		src/tests/all_tests.c \
		src/tests/test_cards.c \
		src/tests/test_hands.c \
		src/tests/test_bucket.c \
		src/tests/test_deck.c \
		src/tests/CuTest.c
	src/tests/all_tests

cards.o:
	${CC} ${CFLAGS} -c src/cards.c -o src/cards.o

hands.o: src/cards.o
	${CC} ${CFLAGS} -c src/hands.c -o src/hands.o

bucket.o: src/cards.o
	${CC} ${CFLAGS} -c src/bucket.c -o src/bucket.o

deck.o: srrc/cards.o
	${CC} ${CFLAGS} -c src/deck.c -o src/deck.o
