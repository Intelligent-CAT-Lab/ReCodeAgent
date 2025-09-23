CC = gcc
CFLAGS = -Wall -Wextra -g -O0 -fprofile-arcs -ftest-coverage -fPIC
LDFLAGS = -lm -fprofile-arcs -ftest-coverage
OBJS = test.o geofence.o

geofence: src/geofence.c
	$(CC) $(CFLAGS) -c src/geofence.c
test: tests/test.c geofence.o
	$(CC) $(CFLAGS) tests/test.c geofence.o -o test $(LDFLAGS)
	./test


clean:
	rm -f test *.gcda *.gcno *.gcov coverage.info
	rm -rf coverage_report
