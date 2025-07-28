vcpkg_find_acquire_program(PYTHON3)
vcpkg_find_acquire_program(SCONS)

vcpkg_from_github(
    OUT_SOURCE_PATH SOURCE_PATH
    REPO samtupy/UniversalSpeechMSVCStatic
    REF ad2fb95eca691eb580121d716d8b89d68cb912e2
    SHA512 37dbf79fa6264fc82829d433b572373358831e3ea407582693274c3b96bb52f750ace501ff9bf32b4d8fea56fcca5274b235a3af50223df8edbe4c8a928c5efd
    HEAD_REF master
)

set(ENV{PATH} "$ENV{PATH};${PYTHON3_DIR}")

# debug build
vcpkg_execute_build_process(
    COMMAND ${SCONS} debug=1
    WORKING_DIRECTORY ${SOURCE_PATH}
    LOGNAME build-dbg
)
file(INSTALL "${SOURCE_PATH}/UniversalSpeechStatic.lib" DESTINATION ${CURRENT_PACKAGES_DIR}/debug/lib)

# release build
vcpkg_execute_build_process(
    COMMAND ${SCONS}
    WORKING_DIRECTORY ${SOURCE_PATH}
    LOGNAME build-rel
)
file(INSTALL "${SOURCE_PATH}/UniversalSpeechStatic.lib" DESTINATION ${CURRENT_PACKAGES_DIR}/lib)

file(GLOB DLLS "${SOURCE_PATH}/bin-x64/*.dll")
if(DLLS)
    file(INSTALL ${DLLS} DESTINATION ${CURRENT_PACKAGES_DIR}/bin)
    file(INSTALL ${DLLS} DESTINATION ${CURRENT_PACKAGES_DIR}/debug/bin)
endif()

if(EXISTS "${SOURCE_PATH}/include")
    file(INSTALL "${SOURCE_PATH}/include/" DESTINATION ${CURRENT_PACKAGES_DIR}/include)
endif()

file(INSTALL "${SOURCE_PATH}/LICENSE.txt" DESTINATION ${CURRENT_PACKAGES_DIR}/share/${PORT} RENAME copyright)

file(REMOVE_RECURSE "${CURRENT_PACKAGES_DIR}/debug/include")
file(REMOVE_RECURSE "${CURRENT_PACKAGES_DIR}/debug/share")

# Create CMake config files
configure_file(
    "${CURRENT_PORT_DIR}/universalspeech-config.cmake.in"
    "${CURRENT_PACKAGES_DIR}/share/${PORT}/universalspeech-config.cmake"
    @ONLY
)
