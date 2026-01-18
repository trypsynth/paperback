cmake_minimum_required(VERSION 3.21)
if(POLICY CMP0144)
	cmake_policy(SET CMP0144 NEW)
endif()
set(CMAKE_TOOLCHAIN_FILE "${CMAKE_SOURCE_DIR}/vcpkg/bin/scripts/buildsystems/vcpkg.cmake" CACHE STRING "Vcpkg toolchain file")
set(VCPKG_MANIFEST_DIR "${CMAKE_SOURCE_DIR}/vcpkg")
set(VCPKG_OVERLAY_TRIPLETS "${CMAKE_SOURCE_DIR}/vcpkg/triplets")
if(WIN32)
	set(VCPKG_TARGET_TRIPLET "x64-windows-static" CACHE STRING "Vcpkg triplet")
elseif(APPLE)
	set(VCPKG_TARGET_TRIPLET "x64-osx" CACHE STRING "Vcpkg triplet")
else()
	set(VCPKG_TARGET_TRIPLET "x64-linux" CACHE STRING "Vcpkg triplet")
endif()
project(paperback VERSION 0.7.0 LANGUAGES CXX)

if("${PROJECT_VERSION_TWEAK}" STREQUAL "")
	set(PROJECT_VERSION_TWEAK 0)
endif()
set(PAPERBACK_VERSION_BUILD ${PROJECT_VERSION_TWEAK})
set(PAPERBACK_GENERATED_DIR "${CMAKE_BINARY_DIR}/generated")
file(MAKE_DIRECTORY "${PAPERBACK_GENERATED_DIR}")
configure_file(${CMAKE_SOURCE_DIR}/app/version.h.in ${PAPERBACK_GENERATED_DIR}/version.h @ONLY)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_BUILD_TYPE Release)
if(MSVC)
	set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
endif()

find_package(Gettext REQUIRED)
find_package(wxWidgets CONFIG REQUIRED COMPONENTS webview)

if(GETTEXT_FOUND)
	find_program(XGETTEXT_EXECUTABLE NAMES xgettext)
	if(XGETTEXT_EXECUTABLE)
		file(GLOB_RECURSE TRANSLATABLE_FILES CONFIGURE_DEPENDS ${CMAKE_SOURCE_DIR}/app/*.cpp ${CMAKE_SOURCE_DIR}/app/*.hpp ${CMAKE_SOURCE_DIR}/app/*.h)
		set(REL_TRANSLATABLE_FILES "")
		foreach(F ${TRANSLATABLE_FILES})
			file(RELATIVE_PATH F_REL "${CMAKE_SOURCE_DIR}" "${F}")
			list(APPEND REL_TRANSLATABLE_FILES "${F_REL}")
		endforeach()
		add_custom_command(
			OUTPUT ${CMAKE_SOURCE_DIR}/po/paperback.pot
			COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/po
			COMMAND ${XGETTEXT_EXECUTABLE}
				--keyword=_
				--keyword=wxPLURAL:1,2
				--keyword=wxTRANSLATE
				--language=C++
				--from-code=UTF-8
				--add-comments=TRANSLATORS
				--add-location=file
				--package-name=paperback
				--package-version=${PROJECT_VERSION}
				--msgid-bugs-address=https://github.com/trypsynth/paperback/issues
				--copyright-holder="Quin Gillespie"
				--output=${CMAKE_SOURCE_DIR}/po/paperback.pot
				${REL_TRANSLATABLE_FILES}
			DEPENDS ${TRANSLATABLE_FILES}
			WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
		)
		add_custom_target(update-pot DEPENDS ${CMAKE_SOURCE_DIR}/po/paperback.pot)
	else()
		message(WARNING "xgettext not found. Translation template (.pot) generation will not be available.")
	endif()
	file(GLOB PO_FILES ${CMAKE_SOURCE_DIR}/po/*.po)
	set(MO_FILES "")
	foreach(PO_FILE ${PO_FILES})
		get_filename_component(LANG ${PO_FILE} NAME_WE)
		set(MO_DIR ${CMAKE_BINARY_DIR}/langs/${LANG}/LC_MESSAGES)
		set(MO_FILE ${MO_DIR}/paperback.mo)
		add_custom_command(OUTPUT ${MO_FILE} COMMAND ${CMAKE_COMMAND} -E make_directory ${MO_DIR} COMMAND ${GETTEXT_MSGFMT_EXECUTABLE} -o ${MO_FILE} ${PO_FILE} DEPENDS ${PO_FILE})
		list(APPEND MO_FILES ${MO_FILE})
	endforeach()
	if(MO_FILES)
		add_custom_target(translations ALL DEPENDS ${MO_FILES})
	endif()
endif()

find_program(CARGO_BIN cargo)
if(NOT CARGO_BIN)
	message(FATAL_ERROR "cargo not found. Install Rust to build libpaperback.")
endif()
file(GLOB_RECURSE LIBPAPERBACK_SOURCES CONFIGURE_DEPENDS ${CMAKE_SOURCE_DIR}/lib/src/*.rs ${CMAKE_SOURCE_DIR}/lib/Cargo.toml ${CMAKE_SOURCE_DIR}/lib/build.rs)
set(RUST_TARGET_DIR ${CMAKE_BINARY_DIR}/lib)
set(RUST_PROFILE_DIR $<IF:$<CONFIG:Debug>,debug,release>)
if(WIN32)
	set(RUST_LIB_NAME paperback.lib)
	set(CXX_BRIDGE_LIB cxxbridge1.lib)
else()
	set(RUST_LIB_NAME libpaperback.a)
	set(CXX_BRIDGE_LIB libcxxbridge1.a)
endif()
set(RUST_LIB_OUTPUT ${RUST_TARGET_DIR}/${RUST_PROFILE_DIR}/${RUST_LIB_NAME})
set(CXX_BRIDGE_OUTPUT ${RUST_TARGET_DIR}/${RUST_PROFILE_DIR}/build/libpaperback-*/out/${CXX_BRIDGE_LIB})

string(TOLOWER "${CMAKE_SYSTEM_PROCESSOR}" PDFIUM_SYSTEM_PROCESSOR_LOWER)
set(PDFIUM_PLATFORM "")
if(WIN32)
	if(PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "amd64" OR PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "x86_64")
		set(PDFIUM_PLATFORM "win-x64")
	elseif(PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "arm64" OR PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "aarch64")
		set(PDFIUM_PLATFORM "win-arm64")
	elseif(PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "x86" OR PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "i686")
		set(PDFIUM_PLATFORM "win-x86")
	endif()
elseif(APPLE)
	if(PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "arm64" OR PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "aarch64")
		set(PDFIUM_PLATFORM "mac-arm64")
	elseif(PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "x86_64")
		set(PDFIUM_PLATFORM "mac-x64")
	endif()
else()
	if(PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "x86_64")
		set(PDFIUM_PLATFORM "linux-x64")
	elseif(PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "x86" OR PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "i686")
		set(PDFIUM_PLATFORM "linux-x86")
	elseif(PDFIUM_SYSTEM_PROCESSOR_LOWER STREQUAL "aarch64")
		set(PDFIUM_PLATFORM "linux-arm64")
	elseif(PDFIUM_SYSTEM_PROCESSOR_LOWER MATCHES "^arm")
		set(PDFIUM_PLATFORM "linux-arm")
	endif()
endif()
if(PDFIUM_PLATFORM STREQUAL "")
	message(FATAL_ERROR "Unsupported platform ${CMAKE_SYSTEM_NAME} (${CMAKE_SYSTEM_PROCESSOR}) for pdfium binaries")
endif()
set(PDFIUM_BASE_DIR "${RUST_TARGET_DIR}/pdfium/${PDFIUM_PLATFORM}")
set(PDFIUM_LIB_DIR "${PDFIUM_BASE_DIR}/lib")
set(PDFIUM_BIN_DIR "${PDFIUM_BASE_DIR}/bin")
set(PDFIUM_RUNTIME_FILE "")
if(WIN32)
	set(PDFIUM_RUNTIME_FILE "${PDFIUM_BIN_DIR}/pdfium.dll")
elseif(APPLE)
	set(PDFIUM_RUNTIME_FILE "")
else()
	set(PDFIUM_RUNTIME_FILE "${PDFIUM_LIB_DIR}/libpdfium.so")
endif()

add_custom_command(OUTPUT ${RUST_LIB_OUTPUT} COMMAND ${CARGO_BIN} build --manifest-path ${CMAKE_SOURCE_DIR}/lib/Cargo.toml --target-dir ${RUST_TARGET_DIR} $<$<NOT:$<CONFIG:Debug>>:--release> WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/lib DEPENDS ${LIBPAPERBACK_SOURCES})
add_custom_target(libpaperback_rust DEPENDS ${RUST_LIB_OUTPUT})

file(GLOB_RECURSE SRC_FILES CONFIGURE_DEPENDS app/*.cpp)
if(WIN32)
	add_executable(paperback WIN32 ${SRC_FILES} app/paperback.rc)
else()
	add_executable(paperback ${SRC_FILES})
endif()
add_dependencies(paperback libpaperback_rust)
target_include_directories(paperback PRIVATE ${CMAKE_SOURCE_DIR}/app ${CMAKE_SOURCE_DIR} ${PAPERBACK_GENERATED_DIR} ${RUST_TARGET_DIR}/cxxbridge)
target_link_directories(paperback PRIVATE ${PDFIUM_LIB_DIR})
target_link_directories(paperback PRIVATE "${CMAKE_BINARY_DIR}/vcpkg_installed/${VCPKG_TARGET_TRIPLET}/lib")
target_link_libraries(paperback PRIVATE
	${RUST_LIB_OUTPUT}
	pdfium
	wx::base
	wx::core
	wx::net
	wx::webview
)
if (WIN32)
	target_link_libraries(paperback PRIVATE bcrypt ntdll)
endif()

if (MSVC)
	target_compile_options(paperback PRIVATE /W4 /permissive- /WX)
else()
	target_compile_options(paperback PRIVATE -Wall -Wextra -Wpedantic -Werror)
endif()

set(LOCAL_DLLS "")
if(WIN32)
	file(GLOB DLLS "${VCPKG_INSTALLED_DIR}/${VCPKG_TARGET_TRIPLET}/bin/*.dll")
	foreach(DLL IN LISTS DLLS)
		get_filename_component(DLL_NAME "${DLL}" NAME)
		add_custom_command(TARGET paperback POST_BUILD
			COMMAND ${CMAKE_COMMAND} -E copy_if_different "${DLL}" "$<TARGET_FILE_DIR:paperback>")
		list(APPEND LOCAL_DLLS "${DLL_NAME}")
	endforeach()
endif()

if(PDFIUM_RUNTIME_FILE)
	get_filename_component(PDFIUM_RUNTIME_NAME "${PDFIUM_RUNTIME_FILE}" NAME)
	add_custom_command(TARGET paperback POST_BUILD
		COMMAND ${CMAKE_COMMAND} -E copy_if_different "${PDFIUM_RUNTIME_FILE}" "$<TARGET_FILE_DIR:paperback>")
	list(APPEND LOCAL_DLLS "${PDFIUM_RUNTIME_NAME}")
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
	add_custom_target(package
		COMMAND ${CMAKE_COMMAND} -E chdir "$<TARGET_FILE_DIR:paperback>"
		${CMAKE_COMMAND} -E tar cf "${PACKAGE_PATH}" --format=zip
		"$<TARGET_FILE_NAME:paperback>" "${CMAKE_BINARY_DIR}/readme.html" ${LOCAL_DLLS} "langs"
		DEPENDS paperback doc translations
	)
else()
	add_custom_target(package
		COMMAND ${CMAKE_COMMAND} -E chdir "$<TARGET_FILE_DIR:paperback>"
		${CMAKE_COMMAND} -E tar czf "${PACKAGE_PATH}"
		"$<TARGET_FILE_NAME:paperback>" "${CMAKE_BINARY_DIR}/readme.html" ${LOCAL_DLLS} "langs"
		DEPENDS paperback doc translations
	)
endif()

set(PAPERBACK_INNO_SCRIPT "${CMAKE_BINARY_DIR}/paperback.iss")
configure_file(${CMAKE_SOURCE_DIR}/paperback.iss.in ${PAPERBACK_INNO_SCRIPT} @ONLY)

if(WIN32)
	find_program(INNO_SETUP_COMPILER ISCC.exe PATHS "$ENV{ProgramFiles\(x86\)}/Inno Setup 6" "$ENV{LOCALAPPDATA}/Programs/Inno Setup 6" "$ENV{ProgramFiles}/Inno Setup 6")
	if(INNO_SETUP_COMPILER)
		add_custom_command(OUTPUT "${CMAKE_BINARY_DIR}/paperback_setup.exe" COMMAND ${INNO_SETUP_COMPILER} "${PAPERBACK_INNO_SCRIPT}" DEPENDS "${PAPERBACK_INNO_SCRIPT}" paperback WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}")
		add_custom_target(installer DEPENDS "${CMAKE_BINARY_DIR}/paperback_setup.exe")
		add_custom_target(release DEPENDS package installer)
	else()
		add_custom_target(release DEPENDS package)
	endif()
else()
	add_custom_target(release DEPENDS package)
endif()
