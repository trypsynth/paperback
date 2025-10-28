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
project(paperback VERSION 0.5 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_BUILD_TYPE Release)
if(MSVC)
	set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
endif()

find_program(CLANG_FORMAT_EXE NAMES clang-format)
if(CLANG_FORMAT_EXE)
	file(GLOB_RECURSE ALL_SOURCE_FILES CONFIGURE_DEPENDS ${CMAKE_SOURCE_DIR}/src/*.cpp ${CMAKE_SOURCE_DIR}/src/*.hpp)
	add_custom_target(format COMMAND ${CLANG_FORMAT_EXE} -i ${ALL_SOURCE_FILES} WORKING_DIRECTORY ${CMAKE_SOURCE_DIR})
endif()
find_program(CLANG_TIDY_EXE NAMES clang-tidy)
if(CLANG_TIDY_EXE)
	file(GLOB_RECURSE ALL_SOURCE_FILES CONFIGURE_DEPENDS ${CMAKE_SOURCE_DIR}/src/*.cpp ${CMAKE_SOURCE_DIR}/src/*.hpp)
	add_custom_target(lint COMMAND ${CLANG_TIDY_EXE} -warnings-as-errors=* --header-filter=^${CMAKE_SOURCE_DIR}/src/ --extra-arg=/EHsc ${ALL_SOURCE_FILES} WORKING_DIRECTORY ${CMAKE_SOURCE_DIR} USES_TERMINAL)
endif()

find_package(chmlib CONFIG REQUIRED)
find_package(lexbor CONFIG REQUIRED)
find_package(maddy CONFIG REQUIRED)
find_package(pdfium CONFIG REQUIRED)
find_package(Poco REQUIRED COMPONENTS Foundation JSON XML)
find_package(wxWidgets CONFIG REQUIRED COMPONENTS webview)
find_package(Gettext)

if(GETTEXT_FOUND)
	find_program(XGETTEXT_EXECUTABLE xgettext PATHS ${VCPKG_INSTALLED_DIR}/${VCPKG_TARGET_TRIPLET}/tools/gettext)
	if(XGETTEXT_EXECUTABLE)
		file(GLOB_RECURSE TRANSLATABLE_FILES CONFIGURE_DEPENDS src/*.cpp src/*.hpp)
		add_custom_command(
			OUTPUT ${CMAKE_SOURCE_DIR}/po/paperback.pot
			COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/po
			COMMAND ${XGETTEXT_EXECUTABLE}
				--keyword=_
				--keyword=wxPLURAL:1,2
				--keyword=wxTRANSLATE
				--language=C++
				--add-comments=TRANSLATORS
				--package-name=paperback
				--package-version=${PROJECT_VERSION}
				--msgid-bugs-address=https://github.com/trypsynth/paperback/issues
				--copyright-holder="Quin Gillespie"
				--output=${CMAKE_SOURCE_DIR}/po/paperback.pot
				${TRANSLATABLE_FILES}
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
else()
	message(WARNING "Gettext not found. Translations will not be built.")
endif()

file(GLOB_RECURSE SRC_FILES CONFIGURE_DEPENDS src/*.cpp)
if(WIN32)
	add_executable(paperback WIN32 ${SRC_FILES})
else()
	add_executable(paperback ${SRC_FILES})
endif()
target_link_libraries(paperback PRIVATE
	chmlib::chm
	lexbor::lexbor
	maddy::maddy
	pdfium::pdfium
	Poco::Foundation
	Poco::JSON
	Poco::XML
	wx::base
	wx::core
	wx::net
	wx::webview
)
if(WIN32)
	target_link_libraries(paperback PRIVATE "${CMAKE_BINARY_DIR}/vcpkg_installed/${VCPKG_TARGET_TRIPLET}/lib/WebView2LoaderStatic.lib")
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

if(WIN32)
	find_program(INNO_SETUP_COMPILER ISCC.exe PATHS "$ENV{ProgramFiles\(x86\)}/Inno Setup 6" "$ENV{LOCALAPPDATA}/Programs/Inno Setup 6" "$ENV{ProgramFiles}/Inno Setup 6")
	if(INNO_SETUP_COMPILER)
		add_custom_command(OUTPUT "${CMAKE_BINARY_DIR}/paperback_setup.exe" COMMAND ${INNO_SETUP_COMPILER} "${CMAKE_SOURCE_DIR}/paperback.iss" DEPENDS "${CMAKE_SOURCE_DIR}/paperback.iss" paperback WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}")
		add_custom_target(installer DEPENDS "${CMAKE_BINARY_DIR}/paperback_setup.exe")
		add_custom_target(release DEPENDS package installer)
	else()
		add_custom_target(release DEPENDS package)
	endif()
else()
	add_custom_target(release DEPENDS package)
endif()