cmake_minimum_required(VERSION 3.21)
set(CMAKE_TOOLCHAIN_FILE "${CMAKE_SOURCE_DIR}/vcpkg/bin/scripts/buildsystems/vcpkg.cmake" CACHE STRING "Vcpkg toolchain file")
set(VCPKG_MANIFEST_DIR "${CMAKE_SOURCE_DIR}/vcpkg")
if(WIN32)
	set(VCPKG_TARGET_TRIPLET "x64-windows-static" CACHE STRING "Vcpkg triplet")
elseif(APPLE)
	set(VCPKG_TARGET_TRIPLET "x64-osx" CACHE STRING "Vcpkg triplet")
else()
	set(VCPKG_TARGET_TRIPLET "x64-linux" CACHE STRING "Vcpkg triplet")
endif()
project(paperback VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_BUILD_TYPE Release)
if(MSVC)
	set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
endif()

find_package(lexbor CONFIG REQUIRED)
find_package(unofficial-maddy CONFIG REQUIRED)
find_package(Poco REQUIRED COMPONENTS Foundation XML Zip)
if(WIN32)
	find_package(universalspeech CONFIG REQUIRED)
endif()
find_package(wxWidgets CONFIG REQUIRED)

file(GLOB_RECURSE SRC_FILES CONFIGURE_DEPENDS src/*.cpp)
if(WIN32)
	add_executable(paperback WIN32 ${SRC_FILES})
else()
	add_executable(paperback ${SRC_FILES})
endif()
target_link_libraries(paperback PRIVATE
	lexbor::lexbor
	Poco::Foundation
	Poco::XML
	Poco::Zip
	unofficial::maddy::maddy
	wx::core
	wx::base
)
if(WIN32)
	target_link_libraries(paperback PRIVATE psapi universalspeech::universalspeech)
endif()

if(WIN32)
	file(GLOB DLLS "${VCPKG_INSTALLED_DIR}/${VCPKG_TARGET_TRIPLET}/bin/*.dll")
	set(LOCAL_DLLS "")
	foreach(DLL IN LISTS DLLS)
		get_filename_component(DLL_NAME "${DLL}" NAME)
		add_custom_command(TARGET paperback POST_BUILD 
			COMMAND ${CMAKE_COMMAND} -E copy_if_different "${DLL}" "$<TARGET_FILE_DIR:paperback>")
		list(APPEND LOCAL_DLLS "${DLL_NAME}")
	endforeach()
else()
	set(LOCAL_DLLS "")
endif()

find_program(PANDOC_EXECUTABLE pandoc)
if(PANDOC_EXECUTABLE)
	add_custom_command(
		OUTPUT ${CMAKE_BINARY_DIR}/readme.html
		COMMAND ${PANDOC_EXECUTABLE} --defaults=${CMAKE_SOURCE_DIR}/doc/pandoc.yaml ${CMAKE_SOURCE_DIR}/doc/readme.md -o ${CMAKE_BINARY_DIR}/readme.html
		DEPENDS ${CMAKE_SOURCE_DIR}/doc/readme.md ${CMAKE_SOURCE_DIR}/doc/pandoc.yaml
	)
	add_custom_target(doc ALL DEPENDS ${CMAKE_BINARY_DIR}/readme.html)
else()
	message(WARNING "Pandoc not found. Documentation will not be generated.")
endif()

if(WIN32)
	set(PACKAGE_NAME "paperback.zip")
elseif(APPLE)
	set(PACKAGE_NAME "paperback_mac.zip")
else()
	set(PACKAGE_NAME "paperback.tar.gz")
endif()
set(PACKAGE_PATH "${CMAKE_BINARY_DIR}/${PACKAGE_NAME}")
if(WIN32)
	add_custom_command(
		TARGET paperback POST_BUILD
		COMMAND ${CMAKE_COMMAND} -E chdir "$<TARGET_FILE_DIR:paperback>" 
		${CMAKE_COMMAND} -E tar cf "${PACKAGE_PATH}" --format=zip 
		"$<TARGET_FILE_NAME:paperback>" "${CMAKE_BINARY_DIR}/readme.html" ${LOCAL_DLLS}
	)
else()
	add_custom_command(
		TARGET paperback POST_BUILD
		COMMAND ${CMAKE_COMMAND} -E chdir "$<TARGET_FILE_DIR:paperback>" 
		${CMAKE_COMMAND} -E tar czf "${PACKAGE_PATH}" 
		"$<TARGET_FILE_NAME:paperback>" "${CMAKE_BINARY_DIR}/readme.html" ${LOCAL_DLLS}
	)
endif()
