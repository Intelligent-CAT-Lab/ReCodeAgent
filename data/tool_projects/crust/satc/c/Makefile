CC=gcc
clean-test:
	rm -rf satc-test html
test:
	gcc -std=gnu99 -Wall -O3 satc-test.c -o satc-test -lm
	./satc-test
docs:
	doxygen
