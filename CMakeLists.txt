cmake_minimum_required(VERSION 3.10)
project(SimpleDB)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(db src/main.cpp)

include(FetchContent)
FetchContent_Declare(
  Catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG v2.13.10
)
FetchContent_MakeAvailable(Catch2)

add_executable(tests tests/test_db.cpp)
target_link_libraries(tests PRIVATE Catch2::Catch2)

# Enable testing and add the tests executable as a test
enable_testing()
add_test(NAME dbTests COMMAND tests)

if(APPLE)
  message(STATUS "Building on macOS")
endif()
