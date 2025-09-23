add_executable(test_bitset test_bitset.c)
add_test(test_bitset test_bitset)
target_link_libraries(test_bitset PUBLIC cborg-bitset)