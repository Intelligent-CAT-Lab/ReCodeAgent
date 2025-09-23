find_package(Threads REQUIRED)

set(TESTS_SOURCES
    polynomial-test.c
    aces-internal-test.c
    common-test.c
    matrix-test.c
    main-test.c
)

add_executable(run_test ${TESTS_SOURCES})
target_link_libraries(run_test PRIVATE
    CAces
    Threads::Threads
)
