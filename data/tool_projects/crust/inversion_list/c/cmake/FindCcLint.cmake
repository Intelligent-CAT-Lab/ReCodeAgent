find_program(CCLINT_EXECUTABLE NAMES cclint)
mark_as_advanced(CCLINT_EXECUTABLE)

function(enable_cclint)
  if(CCLINT_EXECUTABLE)
    add_custom_target(cclint
      COMMAND
        ${CCLINT_EXECUTABLE}
          --repository="${CMAKE_SOURCE_DIR}/src/"
          --recursive
          "${CMAKE_SOURCE_DIR}/src/"
          "${CMAKE_SOURCE_DIR}/test/"
      COMMENT "Check style using cclint."
    )
  endif(CCLINT_EXECUTABLE)
endfunction(enable_cclint)

