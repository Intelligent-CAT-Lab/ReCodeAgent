PREFIX?=/usr

CC=gcc
CFLAGS=-Wall -g -std=c11 -O3 -fpic -Isrc/ -fprofile-arcs -ftest-coverage
LDFLAGS=

SRC_DIR=src
TEST_DIR=test
BUILD_DIR=build

HEADER=$(SRC_DIR)/vcd.h
LIB_TARGET=$(BUILD_DIR)/libvcd.so
LIB_SOURCES=$(wildcard $(SRC_DIR)/*.c)
LIB_OBJECTS=$(patsubst $(SRC_DIR)/%.c, $(BUILD_DIR)/%.o, $(LIB_SOURCES))
TEST_SOURCES=$(wildcard $(TEST_DIR)/*.c)
TEST_EXECUTABLE=$(BUILD_DIR)/test

all: $(TEST_EXECUTABLE)
	 ./$(TEST_EXECUTABLE) $(TEST_DIR)/assets/ram.vcd

install: $(LIB_TARGET)
	install -D -m 644 $(LIB_TARGET) $(PREFIX)/lib/
	install -D -m 644 $(HEADER) $(PREFIX)/include/

$(TEST_EXECUTABLE): $(LIB_TARGET) $(TEST_SOURCES) | $(BUILD_DIR)
	$(CC) $(CFLAGS) $(TEST_SOURCES) $(LIB_TARGET) -o $@ $(LDFLAGS)

$(LIB_TARGET): $(LIB_OBJECTS)
	$(CC) $(CFLAGS) $(LIB_OBJECTS) -o $@ $(LDFLAGS) -shared 

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@ $(LDFLAGS)

$(BUILD_DIR):
	mkdir -p $@

clean:
	rm -rf $(BUILD_DIR)

uninstall:
	rm $(PREFIX)/lib/$(notdir $(LIB_TARGET)) $(PREFIX)/include/$(notdir $(HEADER))

.PHONY: all clean

