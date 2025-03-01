#!/bin/bash
set -eoux pipefail
OUTPUT_PATH="$1"

PREPEND="#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]"

HEADER_PATH="./glfw/include/GLFW/glfw3.h"
if [ ! -f $HEADER_PATH ]; then
    echo "cannot find $HEADER_PATH"
fi
# GLFW_INCLUDE_VULKAN to vulkan convenience functions. requires vulkan headers.
CLANG_ARGS="-DGLFW_INCLUDE_VULKAN"

# on windows, append vulkan sdk include path to provide vulkan header location
case "$OSTYPE" in
  msys*|cygwin*)  CLANG_ARGS="$CLANG_ARGS -I${VULKAN_SDK}/Include" ;;
  *)         ;;
esac


# allowlist-file to only include what we actually need (skip most items from other headers like vulkan)
bindgen --merge-extern-blocks --raw-line="$PREPEND" --allowlist-file=".*glfw3\.h" -o "$OUTPUT_PATH" "$HEADER_PATH" -- $CLANG_ARGS