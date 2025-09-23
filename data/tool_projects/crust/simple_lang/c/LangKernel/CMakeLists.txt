set(SRC_FILES
        lexer.c
        parser.c
        ast.c
        vm.c
        compiler.c
        misc.c
        utils.c
        ../Include/utils.h
)

add_library(simple_lang_lib ${SRC_FILES})

add_executable(simple_lang main.c)
target_link_libraries(simple_lang simple_lang_lib)
