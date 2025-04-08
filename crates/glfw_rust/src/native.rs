use crate::*;

impl EventLoopProxy {
    fn with_platform<T>(&self, p: Platform, f: impl FnOnce() -> T) -> GlfwResult<T> {
        self.with_proxy_alive(|| {
            if self.get_platform() == p {
                return Ok(());
            }
            Err(GlfwError {
                code: ErrorCode::PlatformError,
                description: format!("glfw-rust: This is not {p:?} platform"),
            })
        })?;
        self.with_alive_checked(|| f())
    }
}
#[cfg(all(not(target_os = "macos"), unix, feature = "rwh"))]
mod linux {
    use std::ptr::NonNull;

    use crate::ffi::*;
    use crate::*;
    use raw_window_handle::*;
    impl EventLoopProxy {
        fn with_x11<T>(&self, f: impl FnOnce() -> T) -> GlfwResult<T> {
            self.with_platform(Platform::X11, f)
        }
        pub fn get_x11_display(&self) -> GlfwResult<*mut std::ffi::c_void> {
            self.with_x11(|| unsafe { glfwGetX11Display() })
        }
    }
    impl EventLoop {
        pub fn get_x11_adapter(&self, monitor: MonitorId) -> GlfwResult<usize> {
            if !self.is_monitor_alive(monitor) {
                return Err(GlfwError::dead_monitor(monitor, "get_x11_adapter"));
            }
            self.with_x11(|| unsafe { glfwGetX11Adapter(monitor.inner) })
        }
        pub fn get_x11_monitor(&self, monitor: MonitorId) -> GlfwResult<usize> {
            if !self.is_monitor_alive(monitor) {
                return Err(GlfwError::dead_monitor(monitor, "get_x11_monitor"));
            }
            self.with_x11(|| unsafe { glfwGetX11Monitor(monitor.inner) })
        }
    }
    impl Window {
        pub fn get_x11_window(&self) -> GlfwResult<usize> {
            self.with_x11(|| unsafe { glfwGetX11Window(self.id().get_ptr()) })
        }
    }
    impl HasDisplayHandle for Window {
        fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
            match self.get_platform() {
                Platform::Wayland => {
                    let wayland_display = self.get_wayland_display().map_err(|e| {
                        tracing::error!("failed to get display handle: {e:?}");
                        HandleError::Unavailable
                    })?;

                    let Some(wayland_display) = NonNull::new(wayland_display) else {
                        tracing::error!("wayland display is null");
                        return Err(HandleError::Unavailable);
                    };
                    return Ok(unsafe {
                        DisplayHandle::borrow_raw(RawDisplayHandle::Wayland(
                            WaylandDisplayHandle::new(wayland_display),
                        ))
                    });
                }
                Platform::X11 => {
                    let x11_display = self.get_x11_display().map_err(|e| {
                        tracing::error!("failed to get display handle: {e:?}");
                        HandleError::Unavailable
                    })?;

                    return Ok(unsafe {
                        DisplayHandle::borrow_raw(RawDisplayHandle::Xlib(XlibDisplayHandle::new(
                            NonNull::new(x11_display),
                            0,
                        )))
                    });
                }
                _ => {}
            }
            Err(HandleError::Unavailable)
        }
    }
    impl HasWindowHandle for Window {
        fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
            match self.get_platform() {
                Platform::Wayland => {
                    let wayland_window = self.get_wayland_window().map_err(|e| {
                        tracing::error!("failed to get window handle: {e:?}");
                        HandleError::Unavailable
                    })?;
                    let Some(wayland_window) = NonNull::new(wayland_window) else {
                        tracing::error!("wayland window is null");
                        return Err(HandleError::Unavailable);
                    };
                    return Ok(unsafe {
                        WindowHandle::borrow_raw(RawWindowHandle::Wayland(
                            WaylandWindowHandle::new(wayland_window),
                        ))
                    });
                }
                Platform::X11 => {
                    let x11_window = self.get_x11_window().map_err(|e| {
                        tracing::error!("failed to get window handle: {e:?}");
                        HandleError::Unavailable
                    })?;
                    return Ok(unsafe {
                        WindowHandle::borrow_raw(RawWindowHandle::Xlib(XlibWindowHandle::new(
                            x11_window.try_into().unwrap(),
                        )))
                    });
                }
                _ => {}
            }
            Err(HandleError::Unavailable)
        }
    }
    impl EventLoopProxy {
        fn with_wayland<T>(&self, f: impl FnOnce() -> T) -> GlfwResult<T> {
            self.with_platform(Platform::Wayland, f)
        }
        pub fn get_wayland_display(&self) -> GlfwResult<*mut std::ffi::c_void> {
            self.with_wayland(|| unsafe { glfwGetWaylandDisplay().cast_mut() })
        }
    }
    impl EventLoop {
        pub fn get_wayland_monitor(&self, monitor: MonitorId) -> GlfwResult<*mut std::ffi::c_void> {
            if !self.is_monitor_alive(monitor) {
                return Err(GlfwError::dead_monitor(monitor, "get_wayland_monitor"));
            }
            self.with_wayland(|| unsafe { glfwGetWaylandMonitor(monitor.inner).cast_mut() })
        }
    }
    impl Window {
        pub fn get_wayland_window(&self) -> GlfwResult<*mut std::ffi::c_void> {
            self.with_wayland(|| unsafe { glfwGetWaylandWindow(self.id().get_ptr()) })
        }
    }
}
#[cfg(all(target_os = "windows", feature = "rwh"))]
mod win32 {
    use crate::ffi::*;
    use crate::*;
    impl EventLoopProxy {
        fn with_win32<T>(&self, f: impl FnOnce() -> T) -> GlfwResult<T> {
            self.with_platform(Platform::Win32, f)
        }
    }
    impl EventLoop {
        pub fn get_win32_adapter(&self, monitor: MonitorId) -> GlfwResult<*const std::ffi::c_char> {
            if !self.is_monitor_alive(monitor) {
                return Err(GlfwError::dead_monitor(monitor, "get_win32_adapter"));
            }
            self.with_win32(|| unsafe { glfwGetWin32Adapter(monitor.inner) })
        }
        pub fn get_win32_monitor(&self, monitor: MonitorId) -> GlfwResult<*const std::ffi::c_char> {
            if !self.is_monitor_alive(monitor) {
                return Err(GlfwError::dead_monitor(monitor, "get_win32_monitor"));
            }
            self.with_win32(|| unsafe { glfwGetWin32Monitor(monitor.inner) })
        }
    }
    impl Window {
        pub fn get_win32_window(&self) -> GlfwResult<*mut std::ffi::c_void> {
            self.with_win32(|| unsafe { glfwGetWin32Window(self.id().get_ptr()) })
        }
    }
}
#[cfg(all(target_os = "macos", feature = "rwh"))]
mod cocoa {
    use crate::ffi::*;
    use crate::*;
    impl EventLoopProxy {
        fn with_cocoa<T>(&self, f: impl FnOnce() -> T) -> GlfwResult<T> {
            self.with_platform(Platform::Cocoa, f)
        }
    }
    impl EventLoop {
        pub fn get_cocoa_monitor(&self, monitor: MonitorId) -> GlfwResult<u32> {
            if !self.is_monitor_alive(monitor) {
                return Err(GlfwError::dead_monitor(monitor, "get_cocoa_monitor"));
            }
            self.with_cocoa(|| unsafe { glfwGetCocoaMonitor(monitor.inner) })
        }
    }
    impl Window {
        pub fn get_cocoa_window(&self) -> GlfwResult<*mut std::ffi::c_void> {
            self.with_cocoa(|| unsafe { glfwGetCocoaWindow(self.id().get_ptr()) })
        }
        pub fn get_cocoa_view(&self) -> GlfwResult<*mut std::ffi::c_void> {
            self.with_cocoa(|| unsafe { glfwGetCocoaView(self.id().get_ptr()) })
        }
    }
}
