project(tinyfseq C)

set(CMAKE_C_STANDARD 99)

enable_testing()

add_executable(test_build test/build.c)
add_test(build test_build)

add_executable(test_TFError_string test/TFError_string.c)
add_test(TFError_string test_TFError_string)
add_compile_options(--coverage)
add_link_options(--coverage)
if (ENABLE_FUZZING)
    add_executable(fuzz_TFChannelRange_read fuzz/TFChannelRange_read.c)
    target_link_libraries(fuzz_TFChannelRange_read "-fsanitize=fuzzer")
    target_compile_options(fuzz_TFChannelRange_read PRIVATE "-fsanitize=fuzzer")

    add_executable(fuzz_TFCompressionBlock_read fuzz/TFCompressionBlock_read.c)
    target_link_libraries(fuzz_TFCompressionBlock_read "-fsanitize=fuzzer")
    target_compile_options(fuzz_TFCompressionBlock_read PRIVATE "-fsanitize=fuzzer")

    add_executable(fuzz_TFHeader_read fuzz/TFHeader_read.c)
    target_link_libraries(fuzz_TFHeader_read "-fsanitize=fuzzer")
    target_compile_options(fuzz_TFHeader_read PRIVATE "-fsanitize=fuzzer")

    add_executable(fuzz_TFVarHeader_read fuzz/TFVarHeader_read.c)
    target_link_libraries(fuzz_TFVarHeader_read "-fsanitize=fuzzer")
    target_compile_options(fuzz_TFVarHeader_read PRIVATE "-fsanitize=fuzzer")
endif ()
