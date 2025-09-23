add_library(cborg-bitset-h INTERFACE)
target_include_directories(cborg-bitset-h INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
install(FILES cborg/bitset.h DESTINATION  ${CMAKE_INSTALL_INCLUDEDIR}/cborg)