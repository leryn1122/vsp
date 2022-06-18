CMAKE_MINIMUM_REQUIRED(VERSION 3.10)

PROJECT(vsp)

ADD_COMPILE_OPTIONS(-std=c++20)
SET(CMAKE_CXX_STANDARD 20)
SET(CMAKE_CXX_STANDARD_REQUIRED True)

SET(vsp_SRC_DIR   "${PROJECT_SOURCE_DIR}/src")

# Support `gdb` debug.
SET(CMAKE_BUILD_TYPE "Debug")
SET(CMAKE_CXX_FLAGS_DEBUG "$ENV{CXXFLAGS} -O0 -Wall -g -ggdb")
SET(CMAKE_CXX_FLAGS_RELEASE "$ENV{CXXFLAGS} -O3 -Wall")

#-------------------------------------------------------------------------------

FUNCTION(MESSAGE_LIST)
  FOREACH(_arg IN LISTS ARGN)
    MESSAGE("    * ${_arg}")
  ENDFOREACH()
ENDFUNCTION(MESSAGE_LIST)



#-------------------------------------------------------------------------------

MESSAGE(">>> Determine current system and architecture.")
IF(CMAKE_SIZEOF_VOID_P EQUAL 8)
  SET(vsp_BUILD_ARCH "x64")
ELSE()
  SET(vsp_BUILD_ARCH "x86")
ENDIF(CMAKE_SIZEOF_VOID_P EQUAL 8)
MESSAGE("    Current build architecture: ${vsp_BUILD_ARCH}")

IF(CMAKE_SYSTEM_NAME MATCHES "Linux")
  SET(vsp_BUILD_OS "linux")
ELSEIF(CMAKE_SYSTEM_NAME MATCHES "Windows")
  SET(vsp_BUILD_OS "windows")
ELSEIF(CMAKE_SYSTEM_NAME MATCHES "Darwin")
  SET(vsp_BUILD_OS "darwin")
ELSEIF(CMAKE_SYSTEM_NAME MATCHES "Android")
  SET(vsp_BUILD_OS "android")
ELSE()
  SET(vsp_BUILD_OS "linux")
ENDIF(CMAKE_SYSTEM_NAME MATCHES "Linux")
MESSAGE("    Current build OS: ${vsp_BUILD_OS}")



#-------------------------------------------------------------------------------

MESSAGE(">>> Generate version header (src/version.hpp):")
SET(vsp_VERSION_MAJOR 0)
SET(vsp_VERSION_MINOR 0)
SET(vsp_VERSION_PATCH 1)
SET(vsp_VERSION       ${vsp_VERSION_MAJOR}.${vsp_VERSION_MINOR}.${vsp_VERSION_PATCH})
CONFIGURE_FILE(
  "${vsp_SRC_DIR}/version.hpp.in"
  "${vsp_SRC_DIR}/version.hpp"
)
MESSAGE("    Current version: ${vsp_VERSION_MAJOR}.${vsp_VERSION_MINOR}.${vsp_VERSION_PATCH}")



#-------------------------------------------------------------------------------

MESSAGE(">>> Set the build directory:")
SET(vsp_BUILD_DIR "${PROJECT_BINARY_DIR}/build-${vsp_VERSION}-${vsp_BUILD_OS}-${vsp_BUILD_ARCH}")
SET(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${vsp_BUILD_DIR}/bin")
SET(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${vsp_BUILD_DIR}/lib")
SET(CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${vsp_BUILD_DIR}/lib")
MESSAGE("    Current build directory: ${vsp_BUILD_DIR}")


#-------------------------------------------------------------------------------

MESSAGE(">>> Source files (*.hpp, *.cpp):")
FILE(GLOB_RECURSE vsp_SRC_FILES
  "${vsp_SRC_DIR}/*.hpp"
  "${vsp_SRC_DIR}/*.cpp"
)
FILE(GLOB_RECURSE vsp_BIN_FILES
  "${vsp_SRC_DIR}/bin/*.cpp"
)
LIST(REMOVE_ITEM vsp_SRC_FILES ${vsp_BIN_FILES})
MESSAGE_LIST(${vsp_SRC_FILES})



#-------------------------------------------------------------------------------

MESSAGE(">>> Header directory (src/path/to/*.hpp):")
SET(vsp_HEAD_DIR ${vsp_SRC_FILES})
# TODO This part should be optimized in the future
LIST(FILTER vsp_HEAD_DIR INCLUDE REGEX "(.*).hpp$")
LIST(TRANSFORM vsp_HEAD_DIR REPLACE "(.*)/(.*).hpp$" "\\1")
LIST(REMOVE_DUPLICATES vsp_HEAD_DIR)
INCLUDE_DIRECTORIES(${vsp_HEAD_DIR})
MESSAGE_LIST(${vsp_HEAD_DIR})



#-------------------------------------------------------------------------------

MESSAGE(">>> Executables (src/bin/*.cpp):")
LIST(TRANSFORM vsp_BIN_FILES REPLACE "${vsp_SRC_DIR}/bin/(.*).cpp" "\\1" OUTPUT_VARIABLE vsp_EXECUTABLES)
set(vsp_EXECUTABLES "vspc") # This is a test code.
MESSAGE_LIST(${vsp_EXECUTABLES})
# TODO Enabled to build the multi executables
FOREACH(_executables ${vsp_EXECUTABLES})
  ADD_EXECUTABLE(${_executables}
                "${vsp_SRC_DIR}/bin/${_executables}.cpp"
                "${vsp_SRC_FILES}")
ENDFOREACH()



#-------------------------------------------------------------------------------

MESSAGE(">>> Copy conf, copyright and docs")
FILE(COPY "conf" "includes" "LICENSE" "README.md"
     DESTINATION ${vsp_BUILD_DIR})



#-------------------------------------------------------------------------------

# MESSAGE(">>> Build a CPack install package")
# INCLUDE(InstallRequiredSystemLibraries)
# SET(CPACK_RESOURCE_FILE_LICENSE "${CMAKE_CURRENT_SOURCE_DIR}/LICENSE")
# SET(CPACK_PACKAGE_VERSION_MAJOR "${vsp_VERSION_MAJOR}")
# SET(CPACK_PACKAGE_VERSION_MINOR "${vsp_VERSION_MINOR}")
# INCLUDE(CPack)