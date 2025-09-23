###############################################################################
# Global variables

PROGRAM_NAME                    = 2DpartInt
SRC_C_DIR                       = src
BIN_DIR                         = bin
BUILD_DIR                       = build
INC_DIR                         = include
TEST_DIR                        = test
CC                              = gcc
CXX                             = g++
CFLAGS                          = -fprofile-arcs -ftest-coverage -g
CXXFLAGS                        =
BASE_FLAGS                      = -Wall -Wextra -O3
EXTRA_FLAGS                     =
COMMON_FLAGS                    = $(EXTRA_FLAGS) $(BASE_FLAGS)
ALL_CFLAGS                      = -std=c11 $(COMMON_FLAGS) $(CFLAGS)
ALL_CXXFLAGS                    = -std=c++11 $(COMMON_FLAGS) $(CXXFLAGS)
EXTRA_LDFLAGS                   =
LDFLAGS                         = -lm $(EXTRA_LDFLAGS)
RM                              = rm -rf
MKDIR                           = mkdir -p

COMMON_OBJECT_FILES             = $(BUILD_DIR)/config.o $(BUILD_DIR)/csv.o $(BUILD_DIR)/functions.o $(BUILD_DIR)/initialization.o $(BUILD_DIR)/main.o $(BUILD_DIR)/collisions.o
EXTRA_OBJECT_FILES              =
OBJECT_FILES                    = $(COMMON_OBJECT_FILES) $(EXTRA_OBJECT_FILES)

COMMON_MAIN_DEPENDENCIES        = $(SRC_CXX_DIR)/main.cpp $(INC_DIR)/config.h $(INC_DIR)/csv.h $(INC_DIR)/data.h $(INC_DIR)/functions.h $(INC_DIR)/initialization.h $(INC_DIR)/collisions.h
EXTRA_MAIN_DEPENDENCIES         =
MAIN_O_DEPENDENCIES             = $(COMMON_MAIN_DEPENDENCIES) $(EXTRA_MAIN_DEPENDENCIES)

# If compilation in debug mode.
ifdef DEBUG_STEP
CXXFLAGS                        = -DDEBUG_STEP
EXTRA_FLAGS                     = -g
EXTRA_OBJECT_FILES              = $(BUILD_DIR)/debug.o
EXTRA_MAIN_DEPENDENCIES         = $(INC_DIR)/debug.h
else
EXTRA_FLAGS                     =
endif

# If compilation in profiling mode.
ifdef PROFILING
EXTRA_LDFLAGS                   = -lprofiler
endif

###############################################################################
# Main program compilation

.DEFAULT_GOAL := all

.PHONY: all
all: $(BIN_DIR)/$(PROGRAM_NAME)

$(BIN_DIR)/$(PROGRAM_NAME): $(OBJECT_FILES)
	$(MKDIR) $(BIN_DIR)
	$(CXX) $(ALL_CXXFLAGS) -o $@ $^ $(LDFLAGS)

$(BUILD_DIR)/config.o: $(SRC_CXX_DIR)/config.cpp $(INC_DIR)/config.h
	$(MKDIR) $(BUILD_DIR)
	$(CXX) $(ALL_CXXFLAGS) -I$(INC_DIR) -o $@ -c $<

$(BUILD_DIR)/csv.o: $(SRC_CXX_DIR)/csv.cpp $(INC_DIR)/csv.h $(INC_DIR)/data.h
	$(MKDIR) $(BUILD_DIR)
	$(CXX) $(ALL_CXXFLAGS) -I$(INC_DIR) -o $@ -c $<

$(BUILD_DIR)/initialization.o: $(SRC_CXX_DIR)/initialization.cpp $(INC_DIR)/initialization.h
	$(MKDIR) $(BUILD_DIR)
	$(CXX) $(ALL_CXXFLAGS) -I$(INC_DIR) -o $@ -c $<

$(BUILD_DIR)/functions.o: $(SRC_C_DIR)/functions.c $(INC_DIR)/data.h $(INC_DIR)/functions.h
	$(MKDIR) $(BUILD_DIR)
	$(CC) $(ALL_CFLAGS) -I$(INC_DIR) -o $@ -c $<

$(BUILD_DIR)/collisions.o: $(SRC_C_DIR)/collisions.c $(INC_DIR)/data.h $(INC_DIR)/collisions.h
	$(MKDIR) $(BUILD_DIR)
	$(CC) $(ALL_CFLAGS) -I$(INC_DIR) -o $@ -c $<

$(BUILD_DIR)/debug.o: $(SRC_CXX_DIR)/debug.cpp $(INC_DIR)/debug.h $(INC_DIR)/data.h
	$(MKDIR) $(BUILD_DIR)
	$(CXX) $(ALL_CXXFLAGS) -I$(INC_DIR) -o $@ -c $<

$(BUILD_DIR)/main.o: $(MAIN_O_DEPENDENCIES)
	$(MKDIR) $(BUILD_DIR)
	$(CXX) $(ALL_CXXFLAGS) -I$(INC_DIR) -o $@ -c $<

###############################################################################
# Tests

.PHONY: test
test: $(BIN_DIR)/functions_spec
	$(BIN_DIR)/functions_spec

$(BIN_DIR)/functions_spec: $(BUILD_DIR)/functions.o $(BUILD_DIR)/functions_spec.o
	$(MKDIR) $(BIN_DIR)
	$(CC) $(ALL_CFLAGS) -o $@ $^ $(LDFLAGS)

$(BUILD_DIR)/functions_spec.o: $(TEST_DIR)/functions_spec.c $(INC_DIR)/data.h $(INC_DIR)/functions.h
	$(MKDIR) $(BUILD_DIR)
	$(CC) $(ALL_CFLAGS) -I$(INC_DIR) -o $@ -c $<

###############################################################################
# Clean

.PHONY: clean
clean:
	$(RM) $(BIN_DIR)
	$(RM) $(BUILD_DIR)
