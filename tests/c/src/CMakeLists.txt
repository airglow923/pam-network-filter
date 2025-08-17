# disable clang-tidy for testing as it uses third-party framework
set(CMAKE_CXX_CLANG_TIDY "")

find_package(GTest REQUIRED)

set(gtest_force_shared_crt
    ON
    CACHE BOOL "" FORCE)

file(GLOB TEST_FILES ${CMAKE_CURRENT_LIST_DIR}/*.cc)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Debug)
endif()

function(configure_target target)
  set_target_properties(${target} PROPERTIES RUNTIME_OUTPUT_DIRECTORY
                                             "${CMAKE_BINARY_DIR}")

  target_include_directories(
    ${target} PRIVATE "$<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/include>"
                      "$<INSTALL_INTERFACE:${PROJECT_SOURCE_DIR}/include>")

  target_sources(${target} PRIVATE ${TEST_FILES})

  target_compile_options(
    ${target}
    PRIVATE ${COMMON_COMPILE_FLAGS} "$<$<CONFIG:Debug>:${DEBUG_COMPILE_FLAGS}>"
            "$<$<CONFIG:Release>:${RELEASE_COMPILE_FLAGS}>" -fPIE)

  if("${target}" MATCHES "-shared$")
    set(PROJECT_LIBRARY_FILE "${PROJECT_LIBRARY_NAME}.so")
  else()
    set(PROJECT_LIBRARY_FILE "${PROJECT_LIBRARY_NAME}.a")
  endif()

  target_link_libraries(
    ${target} PRIVATE ${COMMON_LINK_FLAGS} GTest::gtest GTest::gtest_main
                      "${PROJECT_LIBRARY_PATH}/${PROJECT_LIBRARY_FILE}")

  add_test(NAME "${target}" COMMAND "${target}")
endfunction()

string(TOLOWER "${CMAKE_BUILD_TYPE}" PROJECT_BUILD_TYPE)
set(PROJECT_LIBRARY_PATH
    "${PROJECT_SOURCE_DIR}/../../target/${PROJECT_BUILD_TYPE}")
set(PROJECT_LIBRARY_NAME "libpam_network_filter")

add_executable("${CMAKE_PROJECT_NAME}-shared" "")
add_executable("${CMAKE_PROJECT_NAME}-static" "")

configure_target("${CMAKE_PROJECT_NAME}-shared")
configure_target("${CMAKE_PROJECT_NAME}-static")
