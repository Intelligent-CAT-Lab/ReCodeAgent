option(MDB_COVERAGE "enable build for coverage" OFF)
project (minidb)
set(CMAKE_C_STANDARD 11)
cmake_minimum_required (VERSION 2.8)

include_directories(.)

add_library(mdb mdb.c)

add_executable(testmdb testmdb.c)
target_link_libraries(testmdb mdb)

add_executable(testmdb_int testmdb_int.c)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall -Wextra -Wno-unused-function")

if(MDB_COVERAGE)
    link_libraries(gcov)
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O0 -g -fprofile-arcs -ftest-coverage")
    enable_testing()
    add_test(cov_mdb testmdb)
    add_test(cov_mdbint testmdb_int)
endif()
