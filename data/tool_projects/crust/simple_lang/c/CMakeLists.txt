cmake_minimum_required(VERSION 3.27)

project(simple_lang)

if(WIN32)
    add_definitions(-D_CRT_SECURE_NO_WARNINGS)
endif()

include_directories(Include)

add_subdirectory(LangKernel)
add_subdirectory(Tests)

set(CMAKE_C_STANDARD 11)

