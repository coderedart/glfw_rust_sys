#!/bin/bash
set -eoux pipefail
OUTPUT_PATH="$1"

PREPEND="#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(rustdoc::invalid_codeblock_attributes)]
#![allow(rustdoc::invalid_rust_codeblocks)]
#![allow(rustdoc::broken_intra_doc_links)]"

HEADER_PATH="./glfw/include/GLFW/glfw3.h"
if [ ! -f $HEADER_PATH ]; then
    echo "cannot find $HEADER_PATH"
fi
# GLFW_INCLUDE_VULKAN to vulkan convenience functions. requires vulkan headers.
CLANG_ARGS="-DGLFW_INCLUDE_VULKAN -DGLFW_INCLUDE_NONE"

# on windows/mac, append vulkan sdk include path to provide vulkan header location
case "$OSTYPE" in
  msys*|cygwin*|darwin*)  CLANG_ARGS="$CLANG_ARGS -I${VULKAN_SDK}/include" ;;
  *)         ;;
esac
# on mac, for glfw3native.h, add frameworks path for bindgen to correctly find ApplicationServices.h
# case "$OSTYPE" in
#   darwin*)  CLANG_ARGS="$CLANG_ARGS -F/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/" ;;
#   *)         ;;
# esac


# allowlist-file to only include what we actually need (skip most items from other headers like vulkan)
bindgen --merge-extern-blocks --default-macro-constant-type signed --raw-line="$PREPEND" --allowlist-file=".*glfw3\.h" -o "$OUTPUT_PATH" "$HEADER_PATH" -- $CLANG_ARGS