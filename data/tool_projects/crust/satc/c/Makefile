CC=clang
clean-test:
	rm -rf satc-test html
test:
	clang -std=c99 -Wall -O3 satc-test.c -o satc-test
	./satc-test
docs:
	doxygen
