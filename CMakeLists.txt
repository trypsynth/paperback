cmake_minimum_required(VERSION 3.25)
project(paperback LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(wxBUILD_SHARED OFF)
add_subdirectory(lib/wxWidgets)

file(GLOB_RECURSE SRC_FILES src/*.cpp)
add_executable(paperback WIN32 ${SRC_FILES})
target_link_libraries(paperback PRIVATE wx::core wx::base)
