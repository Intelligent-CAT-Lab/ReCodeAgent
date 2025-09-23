ltrep:
	mkdir -p bin
	gcc -O2 -Wall -fPIC -fprofile-arcs -ftest-coverage -Wextra -Wpedantic -Wno-sign-compare -Wno-parentheses -Wno-missing-field-initializers -Wno-unused-parameter -std=c99 ltrep.c ltre.c -o bin/ltrep

compl:
	mkdir -p bin
	gcc -O2 -Wall -fPIC -fprofile-arcs -ftest-coverage -Wextra -Wpedantic -Wno-sign-compare -Wno-parentheses -Wno-missing-field-initializers -Wno-unused-parameter -Wdangling-pointer=1 -std=c99 compl.c ltre.c -o bin/compl

test:
	mkdir -p bin
	gcc -O2 -Wall -fPIC -fprofile-arcs -ftest-coverage -Wextra -Wpedantic -Wno-sign-compare -Wno-parentheses -Wno-missing-field-initializers -std=c99 test.c ltre.c -o bin/test
	./bin/test

clean:
	rm -rf bin
