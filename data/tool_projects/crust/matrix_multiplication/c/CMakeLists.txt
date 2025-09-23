cmake_minimum_required(VERSION 3.24)
project(matrix_multiplication C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_FLAGS "-march=native -fno-unroll-loops -O2")
set(INCLUDE_DIRECTORY ${CMAKE_HOME_DIRECTORY}/include)
include_directories(${INCLUDE_DIRECTORY})
add_executable(matrix_multiplication
        main.c matrix.c utils.c)
