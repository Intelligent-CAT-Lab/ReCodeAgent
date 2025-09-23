find_program(HTML_COVERAGE_EXECUTABLE genhtml)
mark_as_advanced(HTML_COVERAGE_EXECUTABLE)
include(CMakeDependentOption)
cmake_dependent_option(BUILD_HTML_COVERAGE "Enabling html coverage" OFF "BUILD_COVERAGE;HTML_COVERAGE_EXECUTABLE" OFF)
mark_as_advanced(BUILD_HTML_COVERAGE)

function(enable_html_coverage)
    if(BUILD_HTML_COVERAGE)
      add_custom_target(html-coverage
        ${HTML_COVERAGE_EXECUTABLE}
          --output-directory "${CMAKE_BINARY_DIR}/coverage"
          --demangle-cpp
          --num-spaces 2
          --sort
          --title "${PROJECT_NAME} ${PROJECT_VERSION} Coverage"
          --function-coverage
          --branch-coverage
          --legend
          --quiet
          "${CMAKE_BINARY_DIR}/coverage.info"
        DEPENDS coverage
        COMMENT "Building html coverage"
        VERBATIM
      )
      add_custom_command(TARGET html-coverage POST_BUILD
        COMMENT "HTML coverage is in ${CMAKE_BINARY_DIR}/coverage"
      )
    endif()
endfunction(enable_html_coverage)

