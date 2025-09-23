cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 11)
project(jccc)
add_compile_options(-O0 -Wall -Wpedantic -g -Wno-gnu-zero-variadic-macro-arguments)

file(GLOB_RECURSE SOURCES src/*.c)
list(FILTER SOURCES EXCLUDE REGEX ".*\/(driver\/main|testing\/main)\.c$")

# Create a main file for the actual jccc executable
set(MAIN_DRIVER "src/driver/main.c")

add_executable(jccc ${SOURCES} ${MAIN_DRIVER})
target_include_directories(jccc PRIVATE src)

enable_testing()

file(GLOB_RECURSE TEST_SOURCES src/test_*.c)

# Set a main function for testing
set(TEST_MAIN "src/testing/main.c")

add_executable(test_jccc ${SOURCES} ${TEST_SOURCES} ${TEST_MAIN})
target_include_directories(test_jccc PRIVATE src)

add_test(NAME TestJccc COMMAND test_jccc)
