cmake_minimum_required(VERSION 3.21) # Recommended for good vcpkg + manifest support
set(CMAKE_TOOLCHAIN_FILE "${CMAKE_SOURCE_DIR}/external/vcpkg/scripts/buildsystems/vcpkg.cmake" CACHE STRING "Vcpkg toolchain file")
set(VCPKG_TARGET_TRIPLET "x64-windows-static" CACHE STRING "")
project(paperback LANGUAGES CXX VERSION 0.1)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_BUILD_TYPE Release)

find_package(wxWidgets CONFIG REQUIRED)
find_package(Poco REQUIRED COMPONENTS Foundation xml Zip)

file(GLOB_RECURSE SRC_FILES src/*.cpp)
add_executable(paperback WIN32 ${SRC_FILES})
target_link_libraries(paperback PRIVATE wx::core wx::base Poco::Foundation Poco::XML Poco::Zip)
