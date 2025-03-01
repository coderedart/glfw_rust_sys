## Design
So, the library has two main purposes:
1. provide FFI bindings to glfw: pre-generated (fast compile times) and build-time generation (slower)
2. link to glfw library: source builds (using cmake) and pre-built official glfw libs

### Features
* `vulkan` - enables some vulkan convenience functionality (eg: surface creation)
* `static_link` - statically link glfw. not available for linux if `src_build` is disabled.
* `x11` - x11 support
* `wayland` - wayland support (both `x11` and `wayland` can be enabled)
* `native_handles` - enable APIs to get platform specific window handles or display connections. useful for raw-window-handle support.
* `native_gl` - enable APIs for getting platform specific gl contexts (wgl, egl, glx, nsgl etc..)
* `native_egl` - enable egl API even for x11 builds
* `osmesa` - I have no idea. 
* `src_build` - build glfw from source.
* `bindings` - generate glfw FFI bindings at build time from headers. 

For normal applications, you only need to care about 3 features:
1. `bindings` - if you want to generate them at runtime. and if you also enable `native_*` features, this may bloat compile times *a lot* (25+ seconds on windows) due to inclusion of **huge** platform-specific headers like `windows.h`.
2. `src_build` - if you want to build from source. adds around 10 seconds of build time.
3. `static_link` - if you want to link statically. On linux, this requires `src_build` too, so prefer dynamic linking during development for faster compile times. 

### Pre-Generated bindings
We generate FFI bindings before publishing the crate to keep the compile times fast.
If `bindings` feature is disabled, we use the pre-generated bindings.
Run `gen_bindings.sh ./src/sys.rs` script to generate bindings from `glfw/include/GLFW/glfw3.h` and store them in `src/sys.rs`:

This generates core bindings except for "native" handles/gl bindings like getting win32 handle of window or wayland surface or glx context etc..
These "native" manually maintained by hand in `lib.rs` because they include platform specific headers (eg: windows.h or wayland.h ) and we can't generate bindings for *all* platforms at once.

### Build-Time Generated Bindings
When `bindings` feature is turned on, we generate bindings with bindgen inside build script.
This is a fallback, when pre-generated bindings have any mistakes in them (we never know). But this may add significant compile-time overhead.

### Source Builds
if `src_build` feature is enabled, we will build glfw from scratch.
This requires `cmake` to be installed on your system and any other required dependencies.

### Prebuilt-libs builds
If `src_build` feature is disabled, we will link with prebuilt glfw libraries.
On windows and mac, we download the official libraries from https://github.com/glfw/glfw/releases/
On linux, we expect user to have installed glfw library for linking (eg: `libglfw3-dev` on ubuntu). 

