cmake_minimum_required(VERSION 3.21)
set(CMAKE_TOOLCHAIN_FILE "${CMAKE_SOURCE_DIR}/vcpkg/bin/scripts/buildsystems/vcpkg.cmake" CACHE STRING "Vcpkg toolchain file")
set(VCPKG_MANIFEST_DIR "${CMAKE_SOURCE_DIR}/vcpkg")
set(VCPKG_TARGET_TRIPLET "x64-windows-static" CACHE STRING "Vcpkg triplet")
project(paperback VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_BUILD_TYPE release)
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")

find_package(lexbor CONFIG REQUIRED)
find_package(Poco REQUIRED COMPONENTS Foundation xml Zip)
find_package(universalspeech CONFIG REQUIRED)
find_package(wxWidgets CONFIG REQUIRED)

file(GLOB_RECURSE SRC_FILES CONFIGURE_DEPENDS src/*.cpp)
add_executable(paperback WIN32 ${SRC_FILES})
target_link_libraries(paperback PRIVATE
	lexbor::lexbor
	Poco::Foundation
	Poco::XML
	Poco::Zip
	psapi
	universalspeech::universalspeech
	wx::core
	wx::base
)

file(GLOB DLLS "${VCPKG_INSTALLED_DIR}/${VCPKG_TARGET_TRIPLET}/bin/*.dll")
set(LOCAL_DLLS "")
foreach(DLL IN LISTS DLLS)
	get_filename_component(DLL_NAME ${DLL} NAME)
	add_custom_command(TARGET paperback POST_BUILD COMMAND ${CMAKE_COMMAND} -E copy_if_different "${DLL}" "$<TARGET_FILE_DIR:paperback>")
	list(APPEND LOCAL_DLLS "${DLL_NAME}")
endforeach()

find_program(PANDOC_EXECUTABLE pandoc)
if(PANDOC_EXECUTABLE)
	add_custom_command(
		OUTPUT ${CMAKE_BINARY_DIR}/readme.html
		COMMAND ${PANDOC_EXECUTABLE} --standalone --toc --metadata=title:"Paperback Documentation" ${CMAKE_SOURCE_DIR}/doc/readme.md -o ${CMAKE_BINARY_DIR}/readme.html
		DEPENDS ${CMAKE_SOURCE_DIR}/doc/readme.md
	)
	add_custom_target(doc ALL DEPENDS ${CMAKE_BINARY_DIR}/readme.html)
else()
	message(WARNING "Pandoc not found. Documentation will not be generated.")
endif()

set(PACKAGE_NAME "paperback.zip")
set(PACKAGE_PATH "${CMAKE_BINARY_DIR}/${PACKAGE_NAME}")
add_custom_command(
	TARGET paperback POST_BUILD
	COMMAND ${CMAKE_COMMAND} -E chdir "$<TARGET_FILE_DIR:paperback>" ${CMAKE_COMMAND} -E tar cf ${PACKAGE_PATH} --format=zip "$<TARGET_FILE_NAME:paperback>" "${CMAKE_BINARY_DIR}/readme.html" COMMAND ${CMAKE_COMMAND} -E chdir "$<TARGET_FILE_DIR:paperback>" ${CMAKE_COMMAND} -E tar cf ${PACKAGE_PATH} --format=zip "$<TARGET_FILE_NAME:paperback>" "${CMAKE_BINARY_DIR}/readme.html" ${LOCAL_DLLS}
)
