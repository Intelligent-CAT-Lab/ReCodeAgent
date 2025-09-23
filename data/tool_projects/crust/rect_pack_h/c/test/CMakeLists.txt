include(FetchContent)
set(FETCHCONTENT_QUIET FALSE)
FetchContent_Declare(
  unity
  GIT_REPOSITORY https://github.com/ThrowTheSwitch/Unity.git
  GIT_TAG        master
)
FetchContent_MakeAvailable(unity)

add_executable(rpack_tests test.c)
target_link_libraries(rpack_tests unity::framework rect_pack)
add_test(rect_pack_tests rpack_tests)