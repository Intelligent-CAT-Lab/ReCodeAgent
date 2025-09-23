find_program(FLAW_FINDER_EXECUTABLE NAMES flawfinder)
mark_as_advanced(FLAW_FINDER_EXECUTABLE)

function(enable_flaw_finder)
  if(FLAW_FINDER_EXECUTABLE)
    add_custom_target(flawfinder
      COMMAND
        ${FLAW_FINDER_EXECUTABLE}
          "${CMAKE_SOURCE_DIR}/src/"
      COMMENT "Check flaw finders."
    )
  endif(FLAW_FINDER_EXECUTABLE)
endfunction(enable_flaw_finder)

