
cmake_minimum_required(VERSION 3.11.4)
project (impcheck_modules)

add_definitions("-std=gnu99 -Wall -Wextra -Werror -pedantic-errors -flto -g")
if(IMPCHECK_WRITE_DIRECTIVES)
    add_definitions("-DIMPCHECK_WRITE_DIRECTIVES=${IMPCHECK_WRITE_DIRECTIVES}")
endif()
if(IMPCHECK_FLUSH_ALWAYS)
    add_definitions("-DIMPCHECK_FLUSH_ALWAYS=${IMPCHECK_FLUSH_ALWAYS}")
endif()

add_executable(impcheck_parse
    src/trusted/hash.c src/trusted/secret.c src/trusted/siphash.c src/trusted/trusted_parser.c src/trusted/trusted_utils.c src/trusted/vectors.c src/writer.c 
    src/trusted/main_parse.c)
add_executable(impcheck_check 
    src/trusted/confirm.c src/trusted/hash.c src/trusted/lrat_check.c src/trusted/secret.c src/trusted/siphash.c src/trusted/trusted_checker.c src/trusted/top_check.c src/trusted/trusted_utils.c src/trusted/vectors.c src/writer.c
    src/trusted/main_check.c)
add_executable(impcheck_confirm
    src/trusted/confirm.c src/trusted/hash.c src/trusted/secret.c src/trusted/siphash.c src/trusted/trusted_parser.c src/trusted/trusted_utils.c src/trusted/vectors.c src/writer.c 
    src/trusted/main_confirm.c)

add_executable(test_hash src/trusted/trusted_utils.c src/trusted/hash.c src/writer.c test/test.c
    test/test_hash.c)
add_executable(test_full src/trusted/trusted_utils.c src/writer.c test/test.c src/trusted/vectors.c
    test/test_full.c)
