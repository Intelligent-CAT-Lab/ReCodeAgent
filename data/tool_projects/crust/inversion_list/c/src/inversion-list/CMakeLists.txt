set(CMAKE_INSTALL_RPATH_USE_LINK_PATH true)
set(CMAKE_INSTALL_RPATH "${CMAKE_INSTALL_PREFIX}/lib")

add_library(inversion-list SHARED inversion-list.c inversion-list.h)

configure_file(
	${CMAKE_CURRENT_SOURCE_DIR}/inversion-list.pc.in
	${CMAKE_CURRENT_BINARY_DIR}/inversion-list.pc
	@ONLY
)
install(
	FILES ${CMAKE_CURRENT_BINARY_DIR}/inversion-list.pc
	DESTINATION share/pkgconfig
	COMPONENT "PkgConfig"
)

include(CMakePackageConfigHelpers)
configure_package_config_file(
	${CMAKE_CURRENT_SOURCE_DIR}/InversionListConfig.cmake.in
	${CMAKE_CURRENT_BINARY_DIR}/InversionListConfig.cmake
	INSTALL_DESTINATION cmake
)
install(
	FILES ${CMAKE_CURRENT_BINARY_DIR}/InversionListConfig.cmake
	DESTINATION cmake
)

install(
	TARGETS inversion-list 
	LIBRARY DESTINATION lib COMPONENT "Library"
	ARCHIVE DESTINATION lib COMPONENT "Library"
	RUNTIME DESTINATION bin COMPONENT "Runtime"
)

install(
	FILES inversion-list.h
	DESTINATION include
	COMPONENT "Header"
)

find_package(Coverage)
enable_coverage()

find_package(HtmlCoverage)
enable_html_coverage()

