
### Tips
* Most of the functions have `#doc[alias = "glfwName"]` attribute, so just search for the C FFI name and you will get the rust function.
* MonitorIds stop being valid after disconnection, so, all monitor-related functions will return an error if monitor is not connected at the time of the call.
* opengl contexts have some quirks, so, our API will try its best to detect any invalid uses and crash if it detects soundness bugs.
* Please check your tracing logs (`default_error_callback` logs errors) to find any errors in your glfw usage. If we returned `Result` in every function for every possible error, it will add lots of `unwrap` noise in downstream code. So, we just silently ignore non-fatal errors. 
* For fatal errors (soundness bugs), we prefer to panic, as it is better to squash them early. eg: using a window's gl context even after window is destroyed. 

# Design
There's four important areas to consider before wrapping the GLFW FFI API in safe-rust:
1. Initialization and Termination
2. Lifetimes
2. Error handling
3. Soundness Preconditions (including thread-safety)

### Initialization and Termination

* Almost all of glfw API requires a live (initialized) context. So, most of the API that can be called on *any* thread (eg: `EventLoopProxy` or `WindowProxy`) will assert that the `EventLoop` is alive before dispatching to the FFI.
* `EventLoop` destroys windows/cursors and other objects on termination, so, our `Window` struct stores `Rc<EventLoop>` to keep glfw alive until the last window is dropped. 
* There can only ever be one `EventLoop` instance live at any time. So, we use thread-local storage to keep track of the current glfw instance. When `EventLoop` is created, it panics if thread-local data already contains a live glfw instance and when `EvenLoop` is dropped, it updates the thread-local to indicate that there's no live glfw instance anymore.
* Some window functions (like making opengl current) can be called on any thread, So, we have `WindowProxy` (`Send`) for off-thread methods, which internally use a mutex to check that `Window` on main-thread is still alive. `Window` on drop will set the alive status to false, so `WindowProxy` will panic upon use after that.

### Lifetimes
* some pointers like `monitor` will live until we get monitor disconnected callback. So, `EventLoop`'s thread-local data keeps track of live monitors via monitor callbacks. All monitor functions will check for liveness before calling the FFI function.
* Most `char*`/arrays will be invalidated when you call a specific glfw function or if there's a configuration change (eg: glfwGetMonitors). So, rust API only returns owned types like `String/Vec` to avoid dealing with complex lifetimes. 

### Error Handling
Glfw's API has two ways to handle errors:
* The `get_error` function which returns the last error on this thread and clears it.
* The `error_callback` which is a callback that gets called when an error occurs.

The glfw-rust API tries to eliminate as many errors as possible at compile time. For example:
* every window maintains a strong reference to glfw (keeping it alive), which ensures that none of the window methods trigger `GLFW_NOT_INITIALIZED`.
* We also use enums, which eliminates `GLFW_INVALID_ENUM` error. and so on.

But there's errors like `GLFW_PLATFORM_ERROR` which don't exactly have a clear reason and glfw recommends reporting them.

So, we approach error-handling with these two simple rules:
* Most functions would just ignore the errors and you can use the error callback (or tracing logs if using default error callback) to check for any errors. Or you can use `get_error` to explicitly check for errors.
* For functions which return values (that usually return null pointer on error), we will use `Option<T>` or `Result<T>` to express the error case.

This works because most of glfw's errors are not fatal.

### Soundness
* glfw's API is not thread-safe. `EventLoop` and `Window` are `!Send` to keep them on main-thread. For functions which need to be called on off-thread, we provide `EventLoopProxy` and `WindowProxy` which internally assert that the main-thread objects are alive, before calling the FFI function.
* Some functions accept values only from a certain range. eg: `glfwSetSizeLimits` requires that min_width >= max_width and both of them be positive integers. or any enums (glfw represents enums as integer constants). We use types to constrain arguments to certain values (eg: using enums or unsigned types like u32 or Option<T> for null cases). And for more complex cases, we prefer just using assert and documenting that it may panic if some condition is not met.
* Some functions require that there's a current "context". eg: `glfwSwapBuffers` for egl windows. We use thread-local to keep track of current window. When a window is made current (or eventloopproxy wants to make no window current), we use the thread-local data to inform the window that it is not current anymore, before we make a new window current. We also check that the window we will be making current (or the window we are destroying) is not current on any other thread. If we don't do these checks, we will have to mark all of these unsafe due to potential UB.
