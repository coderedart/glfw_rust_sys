use tracing::warn;

use crate::ffi::*;
use crate::*;

/// This is just a monitor "pointer". And it becomes invalid when the monitor is disconnected.
/// You can use [EventLoop::is_monitor_alive] to check if the monitor is still alive.
///
/// If any of the monitor-related methods [EventLoop] find that the monitor is dead, they will
/// exit early with an error. But to avoid mixing that up with other errors, it is better for you
/// to check that the monitor is alive before using it.
///
/// Generally, it is is enough to check that the monitor is alive once at the start of
/// the frame, once after the poll/wait events method.
///
/// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_object>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct MonitorId {
    pub(crate) inner: *mut GLFWmonitor,
}
impl MonitorId {
    pub fn new(inner: *mut GLFWmonitor) -> Option<Self> {
        if inner.is_null() {
            None
        } else {
            Some(MonitorId { inner })
        }
    }
    pub fn get_ptr(self) -> *mut GLFWmonitor {
        self.inner
    }
}

impl EventLoop {
    /// The main-thread local data of the event loop tracks
    /// all monitors returned from [EventLoop::get_monitors] or
    /// [EventLoop::get_primary_monitor].
    ///
    /// Disconnected monitors are removed from the tracked set and
    /// [EventLoop::is_monitor_alive] will return false.
    ///
    /// This function returns true if the monitor is still connected.
    ///
    /// This can be used to avoid using a stale (disconnected) [MonitorId]
    /// in any of the FFI calls. This is also why most monitor-related
    /// methods return a [GlfwResult], as any [MonitorId] you pass as
    /// argument could be dead.
    pub fn is_monitor_alive(&self, monitor: MonitorId) -> bool {
        MAIN_THREAD_LOCAL_DATA.with(|data| data.monitors.borrow().contains(&monitor.inner))
    }
    /// This function returns the primary monitor. This is usually the monitor where elements like the OS task bar are located.
    ///
    /// The monitor could be disconnected at any moment, so, if you plan to use the [MonitorId],
    /// check that it is still connected using [EventLoop::is_monitor_alive].
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_monitors>
    #[doc(alias = "glfwGetPrimaryMonitor")]
    pub fn get_primary_monitor(&self) -> Option<MonitorId> {
        let monitor = unsafe { glfwGetPrimaryMonitor() };
        MAIN_THREAD_LOCAL_DATA.with(|data| {
            data.monitors.borrow_mut().insert(monitor);
        });
        MonitorId::new(monitor)
    }
    /// This function returns an array of handles for all currently connected monitors. The primary monitor is always first in the returned array.
    ///
    /// Any of the monitor handles could become outdated due to monitor being disconnected, so,
    /// if you plan to use the [MonitorId], check that it is still connected using [EventLoop::is_monitor_alive].
    ///
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_monitors>
    #[doc(alias = "glfwGetMonitors")]
    pub fn get_monitors(&self) -> Vec<MonitorId> {
        let mut len = 0;
        let data = self
            .checked(|| unsafe { glfwGetMonitors(&mut len) })
            .expect("failed to call get monitors");

        if len == 0 {
            return Vec::new();
        }
        let mut current_tracked_monitors = HashSet::new();
        let monitors: Vec<MonitorId> =
            unsafe { std::slice::from_raw_parts_mut(data, len.try_into().unwrap()) }
                .iter_mut()
                .filter_map(|monitor| {
                    let Some(monitor) = MonitorId::new(*monitor) else {
                        warn!("found a null pointer for a monitor in get_monitors");
                        return None;
                    };
                    current_tracked_monitors.insert(monitor.inner);
                    Some(monitor)
                })
                .collect();
        let mut dead_monitors_in_old = 0;
        MAIN_THREAD_LOCAL_DATA.with(|data| {
            let old_tracked_monitors = data.monitors.take();
            dead_monitors_in_old = old_tracked_monitors
                .difference(&current_tracked_monitors)
                .count();
            data.monitors.replace(current_tracked_monitors);
        });

        if dead_monitors_in_old > 0 {
            warn!("somehow we have {dead_monitors_in_old} monitors in old monitors, that is not in current monitors");
        }
        monitors
    }
    /// This function returns the position, in screen coordinates, of the upper-left corner of the specified monitor.
    ///
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_pos>
    #[doc(alias = "glfwGetMonitorPos")]
    pub fn get_monitor_pos(&self, monitor: MonitorId) -> GlfwResult<[i32; 2]> {
        if !self.is_monitor_alive(monitor) {
            return Err(GlfwError::dead_monitor(monitor, "get_monitor_pos"));
        }
        let mut x = 0;
        let mut y = 0;
        self.checked(|| {
            unsafe { glfwGetMonitorPos(monitor.inner, &mut x, &mut y) };
        })?;
        Ok([x, y])
    }
    /// This function returns the position, in screen coordinates, of the upper-left corner of the work area of the specified monitor along with the work area size in screen coordinates. The work area is defined as the area of the monitor not occluded by the window system task bar where present. If no task bar exists then the work area is the monitor resolution in screen coordinates.
    ///
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_workarea>
    #[doc(alias = "glfwGetMonitorWorkarea")]
    pub fn get_monitor_work_area(&self, monitor: MonitorId) -> GlfwResult<[i32; 4]> {
        if !self.is_monitor_alive(monitor) {
            return Err(GlfwError::dead_monitor(monitor, "get_monitor_work_area"));
        }
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 0;
        self.checked(|| {
            unsafe {
                glfwGetMonitorWorkarea(monitor.inner, &mut x, &mut y, &mut width, &mut height)
            };
        })?;
        Ok([x, y, width, height])
    }
    /// This function returns the size, in millimetres, of the display area of the specified monitor.
    /// Some platforms do not provide accurate monitor size information, either because the monitor EDID data is incorrect or because the driver does not report it accurately.
    ///
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_size>
    #[doc(alias = "glfwGetMonitorPhysicalSize")]
    pub fn get_monitor_physical_size(&self, monitor: MonitorId) -> GlfwResult<[i32; 2]> {
        if !self.is_monitor_alive(monitor) {
            return Err(GlfwError::dead_monitor(
                monitor,
                "get_monitor_physical_size",
            ));
        }
        let mut width = 0;
        let mut height = 0;
        self.checked(|| {
            unsafe { glfwGetMonitorPhysicalSize(monitor.inner, &mut width, &mut height) };
        })?;
        Ok([width, height])
    }
    /// This function retrieves the content scale for the specified monitor. The content scale is the ratio between the current DPI and the platform's default DPI. This is especially important for text and any UI elements. If the pixel dimensions of your UI scaled by this look appropriate on your machine then it should appear at a reasonable size on other machines regardless of their DPI and scaling settings. This relies on the system DPI and scaling settings being somewhat correct.
    /// The content scale may depend on both the monitor resolution and pixel density and on user settings. It may be very different from the raw DPI calculated from the physical size and current resolution.
    ///
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_scale>
    #[doc(alias = "glfwGetMonitorContentScale")]
    pub fn get_monitor_content_scale(&self, monitor: MonitorId) -> GlfwResult<[f32; 2]> {
        if !self.is_monitor_alive(monitor) {
            return Err(GlfwError::dead_monitor(
                monitor,
                "get_monitor_content_scale",
            ));
        }
        let mut xscale = 0.0;
        let mut yscale = 0.0;
        self.checked(|| {
            unsafe { glfwGetMonitorContentScale(monitor.inner, &mut xscale, &mut yscale) };
        })?;
        Ok([xscale, yscale])
    }
    /// This function returns a human-readable name, encoded as UTF-8, of the specified monitor. The name typically reflects the make and model of the monitor and is not guaranteed to be unique among the connected monitors.
    ///
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_name>
    #[doc(alias = "glfwGetMonitorName")]
    pub fn get_monitor_name(&self, monitor: MonitorId) -> GlfwResult<String> {
        if !self.is_monitor_alive(monitor) {
            return Err(GlfwError::dead_monitor(monitor, "get_monitor_name"));
        }
        let name = self.checked(|| unsafe { glfwGetMonitorName(monitor.inner) })?;
        assert!(!name.is_null());
        Ok(unsafe {
            CStr::from_ptr(name)
                .to_str()
                .expect("monitor name is not utf-8")
                .to_string()
        })
    }
    /// This function returns an array of all video modes supported by the specified monitor. The returned array is sorted in ascending order, first by color bit depth (the sum of all channel depths), then by resolution area (the product of width and height), then resolution width and finally by refresh rate.
    ///
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_modes>
    #[doc(alias = "glfwGetVideoModes")]
    pub fn get_video_modes(&self, monitor: MonitorId) -> GlfwResult<Vec<GLFWvidmode>> {
        if !self.is_monitor_alive(monitor) {
            return Err(GlfwError::dead_monitor(monitor, "get_video_modes"));
        }
        let mut count = 0;
        let data = self.checked(|| unsafe { glfwGetVideoModes(monitor.inner, &mut count) })?;
        unsafe {
            assert!(!data.is_null());
            Ok(std::slice::from_raw_parts(data, count.try_into().unwrap()).to_vec())
        }
    }
    /// This function returns the current video mode of the specified monitor. If you have created a full screen window for that monitor, the return value will depend on whether that window is iconified.
    ///
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_modes>
    #[doc(alias = "glfwGetVideoMode")]
    pub fn get_video_mode(&self, monitor: MonitorId) -> GlfwResult<GLFWvidmode> {
        if !self.is_monitor_alive(monitor) {
            return Err(GlfwError::dead_monitor(monitor, "get_video_mode"));
        }
        let data = self.checked(|| unsafe { glfwGetVideoMode(monitor.inner) })?;
        assert!(!data.is_null());
        Ok(unsafe { *data })
    }
    /// This function generates an appropriately sized gamma ramp from the specified exponent and then calls @ref glfwSetGammaRamp with it. The value must be a finite number greater than zero.
    /// The software controlled gamma ramp is applied in addition to the hardware gamma correction, which today is usually an approximation of sRGB gamma. This means that setting a perfectly linear ramp, or gamma 1.0, will produce the default (usually sRGB-like) behavior.
    ///
    /// # Panics
    /// if gamma is less than zero.
    ///
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_gamma>
    #[doc(alias = "glfwSetGamma")]
    pub fn set_gamma(&self, monitor: MonitorId, gamma: f32) -> GlfwResult<()> {
        assert!(gamma >= 0.0);
        if !self.is_monitor_alive(monitor) {
            return Err(GlfwError::dead_monitor(monitor, "set_gamma"));
        }
        self.checked(|| unsafe { glfwSetGamma(monitor.inner, gamma) })?;
        Ok(())
    }
    /// This function returns the current gamma ramp of the specified monitor.
    ///
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_gamma>
    ///
    /// The return type is a single u16 vector to save allocations.
    /// We copy the red ramp data, then green and finally blue into the vector in that order.
    /// As all of them are same size, so, the total size of vector is `size_of_each_color * 3`.
    /// To get individual color components, use
    /// ```rust
    /// use glfw_rust::*;
    /// fn get_gamma_ramp(el: &EventLoop, monitor: MonitorId) {
    ///     let ramp: Vec<u16> = el.get_gamma_ramp(monitor).unwrap();
    ///     let size_of_each_color = ramp.len() / 3;
    ///     let red = &ramp[0..size_of_each_color];
    ///     let green = &ramp[size_of_each_color..size_of_each_color * 2];
    ///     let blue = &ramp[size_of_each_color * 2..];
    ///     // do whatever you want with those colors.
    /// }
    /// ```
    #[doc(alias = "glfwGetGammaRamp")]
    pub fn get_gamma_ramp(&self, monitor: MonitorId) -> GlfwResult<Vec<u16>> {
        if !self.is_monitor_alive(monitor) {
            return Err(GlfwError::dead_monitor(monitor, "get_gamma_ramp"));
        }
        let data = self.checked(|| unsafe { glfwGetGammaRamp(monitor.inner) })?;
        assert!(!data.is_null());
        unsafe {
            let data = *data;
            let mut ramp = Vec::with_capacity(data.size as usize * 3);
            ramp.extend_from_slice(std::slice::from_raw_parts(data.red, data.size as _));
            ramp.extend_from_slice(std::slice::from_raw_parts(data.green, data.size as _));
            ramp.extend_from_slice(std::slice::from_raw_parts(data.blue, data.size as _));
            Ok(ramp)
        }
    }
    /// This function sets the current gamma ramp for the specified monitor. The original gamma ramp for that monitor is saved by GLFW the first time this function is called and is restored by glfwTerminate.
    /// The software controlled gamma ramp is applied in addition to the hardware gamma correction, which today is usually an approximation of sRGB gamma. This means that setting a perfectly linear ramp, or gamma 1.0, will produce the default (usually sRGB-like) behavior.
    /// For gamma correct rendering with OpenGL or OpenGL ES, see the GLFW_SRGB_CAPABLE hint.
    ///
    /// <https://www.glfw.org/docs/latest/monitor_guide.html#monitor_gamma>
    ///
    /// # Panics
    /// 1. The size of the specified gamma ramp should match the size of the current ramp for that monitor.
    /// 2. On windows, The size of each color component should be 256.
    /// 3. The `ramp.len()` must be a multiple of 3 (as there's 3 colors in it)
    ///
    /// The `ramp` slice is simply red, blue, green colors laid out in that order.
    /// The first 1/3 is red, the second 1/3 is blue and the last 1/3 is green.
    #[doc(alias = "glfwSetGammaRamp")]
    pub fn set_gamma_ramp(&self, monitor: MonitorId, ramp: &[u16]) -> GlfwResult<()> {
        let size_of_each_color = ramp.len() / 3;
        assert_eq!(ramp.len() % 3, 0); // to ensure there's no truncation due to integer division
        #[cfg(windows)]
        assert!(size_of_each_color == 256); // glfw rule: Windows: The gamma ramp size must be 256.
        if !self.is_monitor_alive(monitor) {
            return Err(GlfwError::dead_monitor(monitor, "set gamma ramp"));
        }
        let mut current_size = 0;
        self.checked(|| unsafe {
            let current_ramp = glfwGetGammaRamp(monitor.inner);
            if !current_ramp.is_null() {
                current_size = (*current_ramp).size;
                if current_size != size_of_each_color as _ {
                    return;
                }
            }
            glfwSetGammaRamp(
                monitor.inner,
                &GLFWgammaramp {
                    red: ramp.as_ptr().cast_mut(),
                    green: ramp.as_ptr().add(size_of_each_color).cast_mut(),
                    blue: ramp.as_ptr().add(size_of_each_color * 2).cast_mut(),
                    size: size_of_each_color as _,
                },
            )
        })?;
        if current_size != size_of_each_color as _ {
            panic!("ramp size mismatch in set_gamma_ramp. current_size: {current_size}. size_provided: {size_of_each_color}");
        }
        Ok(())
    }
}
