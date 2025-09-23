cmake_minimum_required(VERSION 3.10)

# Включаем поддержку управления зависимостями
include(ExternalProject)

# Создаем новый проект
project(TEST VERSION 1.0 LANGUAGES C)

# Настраиваем внешний проект для скачивания и сборки check
ExternalProject_Add(
    check_project
    GIT_REPOSITORY https://github.com/libcheck/check.git
    GIT_TAG tags/0.15.2 # Версия check, которую мы хотим скачать
    CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${CMAKE_BINARY_DIR}/check_install
)

# Добавляем путь к заголовочным файлам check
include_directories(
  ${CMAKE_BINARY_DIR}/check_install/include
  ${CMAKE_SOURCE_DIR}/../include
)

# Добавляем исполняемый файл с тестами
add_executable(${PROJECT_NAME}
  ${CMAKE_SOURCE_DIR}/tests.c
)

# Связываем исполняемый файл с библиотекой check
add_dependencies(${PROJECT_NAME}
  check_project
)

add_library(CastomMath STATIC IMPORTED)
set_target_properties(CastomMath PROPERTIES IMPORTED_LOCATION ${CMAKE_SOURCE_DIR}/../lib/CastomMath.a)

# Добавляем флаги компилятора для цели TEST
target_compile_options(${PROJECT_NAME} PRIVATE -fsanitize=address)

# Добавляем флаги связывания для цели TEST
target_link_options(${PROJECT_NAME} PRIVATE -fsanitize=address)

target_link_libraries(${PROJECT_NAME}
  ${CMAKE_BINARY_DIR}/check_install/lib/libcheck.a
  CastomMath
)
