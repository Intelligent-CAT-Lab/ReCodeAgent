cmake_minimum_required(VERSION 3.0)
project(cborg-bitset VERSION 0.1.0 LANGUAGES C)
set(CMAKE_C_FLAGS "-std=c99 -Wall -Wextra -pedantic")

include(GNUInstallDirs)
include(CTest)

# Build and install
add_subdirectory(include)
add_subdirectory(src)

# Tests
add_subdirectory(tests)

# Uninstall
# source: https://gitlab.kitware.com/cmake/community/-/wikis/FAQ#can-i-do-make-uninstall-with-cmake
if(NOT TARGET uninstall)
  configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/cmake_uninstall.cmake.in"
    "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
    IMMEDIATE @ONLY)

  add_custom_target(uninstall
    COMMAND ${CMAKE_COMMAND} -P ${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake)
endif()
