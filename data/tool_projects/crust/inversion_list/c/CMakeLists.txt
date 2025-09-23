cmake_minimum_required(VERSION 3.0)

project(Inversion-List C)

if(POLICY CMP0110) # cmake version >= 3.19
  cmake_policy(SET CMP0110 NEW)
endif()

set(PROJECT_VERSION_MAJOR 0)
set(PROJECT_VERSION_MINOR 0)
set(PROJECT_VERSION_PATCH 1)
set(PROJECT_VERSION ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH})
set(VERSION ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR})

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

enable_testing()

find_package(CppCheck)
enable_cppcheck()

find_package(CcLint)
enable_cclint()

find_package(ClangFormat)
enable_clang_format()

find_package(FlawFinder)
enable_flaw_finder()

add_subdirectory(src)
add_subdirectory(test)
add_subdirectory(docs)

set(CPACK_GENERATOR "ZIP")
set(CPACK_SOURCE_GENERATOR "ZIP")
set(CPACK_SOURCE_IGNORE_FILES ".*/\\\\..*;~$;${CPACK_SOURCE_IGNORE_FILES}")
include(CPack)

