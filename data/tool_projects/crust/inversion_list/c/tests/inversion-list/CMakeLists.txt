include_directories(${CMAKE_SOURCE_DIR}/src)

find_package(Valgrind)

file(GLOB TEST_SRC
  RELATIVE "${CMAKE_CURRENT_SOURCE_DIR}"
  "${CMAKE_CURRENT_SOURCE_DIR}/test-*.c"
)

foreach(TEST_FILE_NAME ${TEST_SRC})
  string(REPLACE ".c" "" TEST_NAME ${TEST_FILE_NAME})
  add_executable(${TEST_NAME} ${TEST_FILE_NAME})

  target_link_libraries(${TEST_NAME} inversion-list)

  valgrind_test(${TEST_NAME} ./${TEST_NAME})
endforeach(TEST_FILE_NAME ${TEST_SRC})
