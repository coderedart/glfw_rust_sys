
# Glfw-Rust
A rust wrapper library for glfw. 

### Tips
* Most of the functions have `#doc[alias = "glfwName"]` attribute, so just search for the C FFI name and you will get the rust function.
* MonitorIds stop being valid after disconnection, so, all monitor-related functions will return an error if monitor is not connected at the time of the call.
* opengl contexts have some quirks, so, our API will try its best to detect any invalid uses and crash if it detects soundness bugs.
* Please check your tracing logs (`default_error_callback` logs errors) to find any errors in your glfw usage. If we returned `Result` in every function for every possible error, it will add lots of `unwrap` noise in downstream code. So, we just silently ignore non-fatal errors. 
* For fatal errors (soundness bugs), we prefer to panic, as it is better to squash them early. eg: using a window's gl context even after window is destroyed. 