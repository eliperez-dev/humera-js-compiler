#----------------------------------------------------------------
# Generated CMake target import file for configuration "Debug".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "wabt::wabt" for configuration "Debug"
set_property(TARGET wabt::wabt APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(wabt::wabt PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "C;CXX"
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib/wabt.lib"
  )

list(APPEND _cmake_import_check_targets wabt::wabt )
list(APPEND _cmake_import_check_files_for_wabt::wabt "${_IMPORT_PREFIX}/lib/wabt.lib" )

# Import target "wabt::wasm-rt-impl" for configuration "Debug"
set_property(TARGET wabt::wasm-rt-impl APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(wabt::wasm-rt-impl PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "C"
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib/wasm-rt-impl.lib"
  )

list(APPEND _cmake_import_check_targets wabt::wasm-rt-impl )
list(APPEND _cmake_import_check_files_for_wabt::wasm-rt-impl "${_IMPORT_PREFIX}/lib/wasm-rt-impl.lib" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
