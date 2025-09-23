cmake_minimum_required(VERSION 3.16)

project (CAces C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED On)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Release")
endif()

if(CMAKE_BUILD_TYPE STREQUAL "Release")
    add_definitions(-DNDEBUG)
    set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
endif()

set(CMAKE_CXX_FLAGS_RELEASE "-O3")
set(CMAKE_EXPORT_COMPILE_COMMANDS On)
add_compile_options(-Wall -Wextra -pedantic -Werror)

set(SOURCE_FILES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/Aces.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/Aces-internal.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/Channel.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/Matrix.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/Common.c
    ${CMAKE_CURRENT_SOURCE_DIR}/src/Polynomial.c
)

add_library(CAces STATIC ${SOURCE_FILES})

target_link_libraries(CAces PRIVATE m)
target_include_directories(CAces PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/lib
)

option(CACES_BENCHMARK "Enable benchmarking" OFF)
option(CACES_TEST "Enable tests" OFF)
option(CACES_EXAMPLE "Enable examples" OFF)

if (CACES_BENCHMARK)
    add_subdirectory(benchmarks)
endif()

if (CACES_TEST)
    add_subdirectory(tests)
endif()

if (CACES_EXAMPLE)
    add_subdirectory(examples)
endif()

add_custom_target(
    clang-tidy-check clang-tidy -p ${CMAKE_BINARY_DIR}/compile_commands.json -checks=cert* --warnings-as-errors=* -header-filter=.* ${SOURCE_FILES}
    DEPENDS ${SOURCE_FILES}
)
