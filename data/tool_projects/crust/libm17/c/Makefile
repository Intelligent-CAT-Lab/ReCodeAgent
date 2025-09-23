INC     = -I ./ -I ./unit_tests/unity/src

TARGET  = libm17.so
CFLAGS  = $(INC) -fPIC -Wall -Wextra
LDFLAGS = -shared -lm
SRCS    = m17.c $(filter-out unit_tests/unit_tests.c, $(wildcard */*.c))
OBJS    = $(SRCS:.c=.o)

all: $(TARGET) 

clean:
	rm -f $(OBJS)

fclean:
	rm -f $(TARGET)

test:
	cc -I./ -I./unit_tests/unity/src -L. -Wall -Wextra -fPIC     unit_tests/unit_tests.c unit_tests/unity/src/unity.c     -o unit_tests/unit_tests     -lm -lm17
	LD_LIBRARY_PATH=./ ./unit_tests/unit_tests

testrun:
	LD_LIBRARY_PATH=./ ./unit_tests/unit_tests

install:
	sudo install $(TARGET) /usr/local/lib

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) $(OBJS) -o $@ $(LDFLAGS)

.PHONY: all clean fclean
