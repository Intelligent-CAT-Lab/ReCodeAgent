cmake_minimum_required(VERSION 3.24)
project(matrix_multiplication C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_FLAGS "-march=native -O2")
set(INCLUDE_DIRECTORY ${CMAKE_HOME_DIRECTORY}/../include)
set(SOURCE_DIRECTORY ${CMAKE_HOME_DIRECTORY}/..)
include_directories(${INCLUDE_DIRECTORY})
add_executable(test_matrix
        test_matrix.c
        ${SOURCE_DIRECTORY}/matrix.c
        ${SOURCE_DIRECTORY}/utils.c
        )