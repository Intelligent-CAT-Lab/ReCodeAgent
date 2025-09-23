.PHONY: clean doc test

CFLAGS := -DNDEBUG -O3 -Werror $(CFLAGS) -fprofile-arcs -ftest-coverage -lm

razz: razz.o card.o razz_simulation.o

razz.o: razz_simulation.h card.h

razz_simulation.o: razz_simulation.h

card.o: card.h

card_test.o: card.h

card_test: card_test.o card.o

test: card_test
	valgrind --leak-check=full ./card_test

doc:
	doxygen

clean:
	rm -f *.o
