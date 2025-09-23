
# --- Simple example ---
add_executable(simple simple.c)
target_link_libraries(simple PRIVATE rect_pack)


# --- GUI example ---

set(CMAKE_CXX_STANDARD 17)

include(FetchContent)
set(FETCHCONTENT_QUIET FALSE)
FetchContent_Declare(
  quick
  GIT_REPOSITORY https://github.com/luihabl/quick.git
  GIT_TAG        main
)
FetchContent_MakeAvailable(quick)

add_executable(gui gui.cpp)
target_link_libraries(gui PRIVATE quick rect_pack)
