# .PHONY: check
# check: test bench
# 	./test
# 	./bench
# CFLAGS=
# .PHONY: clean
# clean:
# 	rm -f *.o test bench

# test.o: tests/test.c morton.h Makefile

# test: morton.o test.o
# bench: morton.o bench.o
# bench.o: bench.c morton.h Makefile
# morton.o: morton.c morton.h Makefile

test: 
	gcc $(CFLAGS) -o test tests/test.c morton.c morton.h
	./test