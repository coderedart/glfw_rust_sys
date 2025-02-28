## Usage
The library is mainly optimized on opengl usage. So, if you are using opengl, the compile times are fast, as most of the bindings are pre-generated.

If you are using vulkan or need native window handles etc.., it will have longer compile times as we need to generate the bindings at build time.

#### Generate bindings
Just run the below command to generate bindings from `glfw/include/GLFW/glfw3.h` and store them in `src/sys.rs`:

```sh
bindgen --merge-extern-blocks --raw-line='#![allow(unused)]' --raw-line='#![allow(non_upper_case_globals)]' --raw-line='#![allow(non_camel_case_types)]' --raw-line='#![allow(non_snake_case)]' -o src/sys.rs ./glfw/include/GLFW/glfw3.h -- -DGLFW_INCLUDE_NONE
```

If you are using opengl, and don't care about vulkan or window handles etc.., this is probably the best option. It requires no build time work and we only need to generate the bindings once before publishing the crate.