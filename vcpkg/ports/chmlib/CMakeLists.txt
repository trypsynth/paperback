cmake_minimum_required(VERSION 3.5)
project(chm C)

include(GNUInstallDirs)
include(CMakePackageConfigHelpers)

include (CheckIncludeFiles)
check_include_files(strings.h HAVE_STRINGS_H) # for ffs

add_library(chm src/lzx.c src/chm_lib.c)
add_library(chmlib::chm ALIAS chm)

if(HAVE_STRINGS_H)
    target_compile_definitions(chm PRIVATE CHMLIB_HAVE_STRINGS_H)
endif()

target_include_directories(chm PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/src>
    $<INSTALL_INTERFACE:include>
)

if(BUILD_TOOLS)
    link_libraries(chm)
    add_executable(enum_chmLib src/enum_chmLib.c)
    add_executable(enumdir_chmLib src/enumdir_chmLib.c)
    add_executable(extract_chmLib src/extract_chmLib.c)

    install(TARGETS enum_chmLib extract_chmLib enumdir_chmLib
        RUNTIME DESTINATION tools/chmlib
        LIBRARY DESTINATION lib
        ARCHIVE DESTINATION lib
    )
endif()

install(TARGETS chm
    EXPORT chmlibTargets
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
)

set(CHMLIB_CMAKE_DIR "${CMAKE_INSTALL_LIBDIR}/cmake/chmlib")
write_basic_package_version_file("${CMAKE_CURRENT_BINARY_DIR}/chmlibConfigVersion.cmake" VERSION 0.40 COMPATIBILITY SameMajorVersion)
configure_package_config_file("${CMAKE_CURRENT_SOURCE_DIR}/chmlibConfig.cmake.in" "${CMAKE_CURRENT_BINARY_DIR}/chmlibConfig.cmake" INSTALL_DESTINATION "${CHMLIB_CMAKE_DIR}")

install(EXPORT chmlibTargets
    FILE chmlibTargets.cmake
    NAMESPACE chmlib::
    DESTINATION "${CHMLIB_CMAKE_DIR}"
)

install(FILES
    "${CMAKE_CURRENT_BINARY_DIR}/chmlibConfig.cmake"
    "${CMAKE_CURRENT_BINARY_DIR}/chmlibConfigVersion.cmake"
    DESTINATION "${CHMLIB_CMAKE_DIR}"
)
