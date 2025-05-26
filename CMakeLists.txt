cmake_minimum_required(VERSION 3.21) # Recommended for good vcpkg + manifest support
project(paperback LANGUAGES CXX VERSION 0.1)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Optional: set release build as default
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release CACHE STRING "Build type" FORCE)
endif()

find_package(wxwidgets REQUIRED COMPONENTS core base)

file(GLOB_RECURSE SRC_FILES src/*.cpp)
add_executable(paperback WIN32 ${SRC_FILES})

# Link to the imported targets vcpkg provides
target_link_libraries(paperback PRIVATE wx::core wx::base Poco::Foundation)
