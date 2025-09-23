find_program(CPPCHECK_EXECUTABLE NAMES cppcheck)
mark_as_advanced(CPPCHECK_EXECUTABLE)
include(CMakeDependentOption)
cmake_dependent_option(RUN_CPPCHECK "Enabling coverage support" OFF "CPPCHECK_EXECUTABLE" OFF)
mark_as_advanced(RUN_CPPCHECK)

function(enable_cppcheck)
  if(RUN_CPPCHECK)
    set(CMAKE_C_CPPCHECK ${CPPCHECK_EXECUTABLE} PARENT_SCOPE)
    mark_as_advanced(CMAKE_C_CPPCHECK)
    list(
      APPEND CMAKE_C_CPPCHECK 
         "--language=c"
         "--std=c11"
         "--enable=all"
         "--inconclusive"
         "--force"
         "--template='[{file}:{line}]: ({severity}) {id} {message}"
         "--suppressions-list=${CMAKE_SOURCE_DIR}/CppCheckSuppressions.txt"
         "--error-exitcode=1"
    )
  endif(RUN_CPPCHECK)
endfunction(enable_cppcheck)

