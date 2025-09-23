#----------------------------------------------------------------
# Generated CMake target import file for configuration "Debug".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "libm17::libm17" for configuration "Debug"
set_property(TARGET libm17::libm17 APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(libm17::libm17 PROPERTIES
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib/libm17.so.0.1"
  IMPORTED_SONAME_DEBUG "libm17.so.0.1"
  )

list(APPEND _IMPORT_CHECK_TARGETS libm17::libm17 )
list(APPEND _IMPORT_CHECK_FILES_FOR_libm17::libm17 "${_IMPORT_PREFIX}/lib/libm17.so.0.1" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
