cmake_minimum_required(VERSION 3.11)

project(
  "pam-network-filter-tests"
  VERSION 1.0.0
  DESCRIPTION "Tests for pam-network-filter in C")

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED True)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

list(INSERT CMAKE_MODULE_PATH 0 ${CMAKE_CURRENT_LIST_DIR}/cmake)

find_package(PkgConfig REQUIRED)
find_package(Threads REQUIRED)
find_library(LIBCXXABI "c++abi")

# libstdc++ implementation of <filesystem> prior to 9.1
find_library(LIBSTDCXXFS "stdc++fs")

# libc++ implementation of <filesystem> prior to 9.0
find_library(LIBCXXFS "c++fs")

find_program(CLANG_TIDY_FOUND NAMES clang-tidy)

if(CLANG_TIDY_FOUND AND USE_CLANG_TIDY)
  set(CMAKE_CXX_CLANG_TIDY
      "clang-tidy"
      CACHE INTERNAL "clang-tidy")
endif()

# clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

execute_process(
  COMMAND
    ${CMAKE_COMMAND} -E create_symlink
    ${CMAKE_CURRENT_BINARY_DIR}/compile_commands.json
    ${CMAKE_CURRENT_LIST_DIR}/compile_commands.json)

set(COMMON_WARNING_FLAGS
    -Wall
    -Wextra
    -Werror
    -Wcast-qual
    -Wdiv-by-zero
    -Wno-unknown-warning-option
    -Wno-bitwise-instead-of-logical)

set(COMMON_COMPILE_FLAGS ${CMAKE_CXX_FLAGS} ${COMMON_WARNING_FLAGS}
                         -march=native -flto -fno-rtti)

# use explicit flags for coverage instead of --coverage
if(CMAKE_ENABLE_COVERAGE)
  set(COVERAGE_FLAGS -ftest-coverage -fprofile-arcs -pg)
endif()

set(ASAN_FLAGS -fsanitize=address -fsanitize-address-use-after-scope
               -fsanitize-recover=address -fno-omit-frame-pointer)

set(DEBUG_COMPILE_FLAGS -Og ${COVERAGE_FLAGS})

set(RELEASE_COMPILE_FLAGS -ffast-math)

set(COMMON_LINK_FLAGS -flto "$<$<CONFIG:Debug>:${COVERAGE_FLAGS}>"
                      ${CMAKE_THREAD_LIBS_INIT})

if(NOT ${LIBSTDCXXFS} STREQUAL "LIBSTDCXXFS-NOTFOUND")
  set(COMMON_LINK_FLAGS ${COMMON_LINK_FLAGS} ${LIBSTDCXXFS})
endif()

if(ADDRESS_SANITIZER)
  set(COMMON_COMPILE_FLAGS ${COMMON_COMPILE_FLAGS} ${ASAN_FLAGS})
  set(COMMON_LINK_FLAGS ${COMMON_LINK_FLAGS} ${ASAN_FLAGS})
endif()

if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
  set(DEBUG_COMPILE_FLAGS ${DEBUG_COMPILE_FLAGS})
elseif(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
  set(DEBUG_COMPILE_FLAGS ${DEBUG_COMPILE_FLAGS} -fprofile-instr-generate
                          -fcoverage-mapping)

  if(NOT ${LIBCXXABI} STREQUAL "LIBCXXABI-NOTFOUND")
    set(COMMON_COMPILE_FLAGS ${COMMON_COMPILE_FLAGS} -stdlib=libc++)
    set(COMMON_LINK_FLAGS ${COMMON_LINK_FLAGS} ${LIBCXXABI})
  endif()

  if(NOT ${LIBCXXFS} STREQUAL "LIBCXXFS-NOTFOUND")
    set(COMMON_LINK_FLAGS ${COMMON_LINK_FLAGS} ${LIBCXXFS})
  endif()
endif()

enable_testing()
add_subdirectory(src)
