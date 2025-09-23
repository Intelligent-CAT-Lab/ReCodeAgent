OBJECTS= compiler.o cprocess.o lexer.o parser.o node.o token.o lex_process.o helpers/buffer.o helpers/vector.o
INCLUDES = -I./
CFLAGS = -g -Wall -Wextra -std=c99 -fprofile-arcs -ftest-coverage -fPIC
all: ${OBJECTS}
	gcc $(CFLAGS) tests/main.c ${INCLUDES} ${OBJECTS} -g -o ./main
	./main

compiler.o: ./compiler.c
	gcc $(CFLAGS) ./compiler.c ${INCLUDES } -o compiler.o -g -c

cprocess.o: ./cprocess.c
	gcc $(CFLAGS) ./cprocess.c ${INCLUDES } -o cprocess.o -g -c

lexer.o: ./lexer.c 
	gcc $(CFLAGS) ./lexer.c ${INCLUDES } -o lexer.o -g -c

token.o: ./token.c 
	gcc $(CFLAGS) ./token.c ${INCLUDES } -o token.o -g -c

lex_process.o: ./lex_process.c
	gcc $(CFLAGS) ./lex_process.c ${INCLUDES } -o lex_process.o -g -c

parser.o: ./parser.c
	gcc $(CFLAGS) ./parser.c ${INCLUDES } -o parser.o -g -c

node.o: ./node.c
	gcc $(CFLAGS) ./node.c ${INCLUDES } -o node.o -g -c

helpers/buffer.o: ./helpers/buffer.c
	gcc $(CFLAGS) ./helpers/buffer.c ${INCLUDES } -o helpers/buffer.o -g -c

helpers/vector.o: ./helpers/vector.c
	gcc $(CFLAGS) ./helpers/vector.c ${INCLUDES } -o helpers/vector.o -g -c
clean:
	rm ./main
	rm -rf ${OBJECTS}
	rm -rf *.gcda *.gcno *.gcov
	rm -rf **/*.gcda **/*.gcno **/*.gcov