cmake_minimum_required(VERSION 3.27)
project(vaultsync C)

# Set C standard
set(CMAKE_C_STANDARD 11)

# Source files
file(GLOB_RECURSE SOURCES "src/*.c" "src/**/*.c")

# Include directories
include_directories(include)

# Create a library (e.g., static library) containing the main functionality
add_library(vaultsync_lib STATIC ${SOURCES})

# Specify include directories for the library
target_include_directories(vaultsync_lib PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

# Find OpenSSL
find_package(OpenSSL REQUIRED)

# Create the main executable
add_executable(${PROJECT_NAME} src/main.c)

# Compilation options
target_compile_options(${PROJECT_NAME} PRIVATE)

# Specify include directories for the main executable
target_include_directories(${PROJECT_NAME} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

# Link the main executable with the library and OpenSSL
target_link_libraries(${PROJECT_NAME} PRIVATE vaultsync_lib OpenSSL::SSL OpenSSL::Crypto)

# -------------------------------- test stuffs -----------------------------------
# Testing with Unity
enable_testing()

# Unity source files
set(UNITY_SOURCES
    test/unity/unity.c
    test/unity/unity.h
    test/unity/unity_internals.h
)

# Test files
set(TEST_SOURCES
    test/main_test.c
)

# Create a test executable
add_executable(vaultsync_tests ${UNITY_SOURCES} ${TEST_SOURCES})

# Link the test executable with the library
target_link_libraries(vaultsync_tests PRIVATE vaultsync_lib OpenSSL::SSL OpenSSL::Crypto)

# Add tests
add_test(NAME vaultsync_tests COMMAND vaultsync_tests)
