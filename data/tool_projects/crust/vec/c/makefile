CC = gcc
CFLAGS = -Wall -Wextra -g 
LDFLAGS = -fprofile-arcs -ftest-coverage -fPIC
OBJ = vec.o
EXEC = test_1

all: $(EXEC)

test: $(EXEC)
	./$(EXEC)

$(EXEC): tests/test.c $(OBJ)
	$(CC) $(CFLAGS) $(LDFLAGS) tests/test.c $(OBJ) -o $(EXEC) -lgcov

vec.o: src/vec.c src/vec.h
	$(CC) $(CFLAGS) -c src/vec.c -o vec.o $(LDFLAGS)

clean:
	rm -f $(EXEC) $(OBJ)
	rm -f *.gcno *.gcda *.gcov
