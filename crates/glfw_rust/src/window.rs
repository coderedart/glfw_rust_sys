use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread::ThreadId;

use tracing::error;

use crate::event::Event;
use crate::ffi::*;
use crate::*;
#[derive(Debug, Default)]
pub struct WindowConfig {
    pub resizeable: Option<bool>,
    pub visible: Option<bool>,
    pub decorated: Option<bool>,
    pub focused: Option<bool>,
    pub auto_iconify: Option<bool>,
    pub floating: Option<bool>,
    pub maximized: Option<bool>,
    pub center_cursor: Option<bool>,
    pub transparent_framebuffer: Option<bool>,
    pub focus_on_show: Option<bool>,
    pub scale_to_monitor: Option<bool>,
    pub scale_framebuffer: Option<bool>,
    pub mouse_passthrough: Option<bool>,
    pub position_x: Option<i32>,
    pub position_y: Option<i32>,
    pub red_bits: Option<i32>,
    pub green_bits: Option<i32>,
    pub blue_bits: Option<i32>,
    pub alpha_bits: Option<i32>,
    pub depth_bits: Option<i32>,
    pub stencil_bits: Option<i32>,
    pub accum_red_bits: Option<i32>,
    pub accum_green_bits: Option<i32>,
    pub accum_blue_bits: Option<i32>,
    pub accum_alpha_bits: Option<i32>,
    pub aux_buffers: Option<i32>,
    pub samples: Option<i32>,
    pub refresh_rate: Option<i32>,
    pub stereo: Option<bool>,
    pub srgb_capable: Option<bool>,
    pub doublebuffer: Option<bool>,
    pub client_api: Option<ClientApi>,
    pub context_creation_api: Option<ContextCreationApi>,
    pub context_version_major: Option<i32>,
    pub context_version_minor: Option<i32>,
    pub context_robustness: Option<Robustness>,
    pub context_release_behavior: Option<ContextReleaseBehavior>,
    pub opengl_forward_compat: Option<bool>,
    pub opengl_context_debug: Option<bool>,
    pub opengl_profile: Option<OpenGLProfile>,
    pub win32_keyboard_menu: Option<bool>,
    pub win32_showdefault: Option<bool>,
    pub cocoa_frame_name: Option<String>,
    pub cocoa_graphics_switching: Option<bool>,
    /// must be ascii
    pub wayland_app_id: Option<String>,
    /// must be ascii
    pub x11_class_name: Option<String>,
    /// must be ascii
    pub x11_instance_name: Option<String>,
}
impl WindowConfig {
    /// <https://www.glfw.org/docs/latest/window_guide.html#window_hints>
    ///
    /// This contains a lot of options which are set by `glfwWindowHint`
    /// and `glfwWindowHintString`.
    ///
    /// Some of these are platform specific and will be ignored on other platforms.
    ///
    /// Some of these can be changed later via window methods like "resizeable" or "decorated".
    /// But other options are permanent like client api or transparent framebuffer.
    ///
    /// There's no guarantee that all of these options will be respected.
    /// Some are "hard constraints" like client api (opengl or non-opengl), while others are
    /// "soft constraints", where glfw will *try* to aim for the closest match (eg: opengl version).
    #[doc(alias = "glfwWindowHintString")]
    #[doc(alias = "glfwWindowHint")]
    pub fn set_hints(self, el: &EventLoop) -> Result<(), GlfwError> {
        let WindowConfig {
            resizeable: resizable,
            visible,
            decorated,
            focused,
            auto_iconify,
            floating,
            maximized,
            center_cursor,
            transparent_framebuffer,
            focus_on_show,
            scale_to_monitor,
            scale_framebuffer,
            mouse_passthrough,
            position_x,
            position_y,
            red_bits,
            green_bits,
            blue_bits,
            alpha_bits,
            depth_bits,
            stencil_bits,
            accum_red_bits,
            accum_green_bits,
            accum_blue_bits,
            accum_alpha_bits,
            aux_buffers,
            samples,
            refresh_rate,
            stereo,
            srgb_capable,
            doublebuffer,
            client_api,
            context_creation_api,
            context_version_major,
            context_version_minor,
            context_robustness,
            context_release_behavior,
            opengl_forward_compat,
            opengl_context_debug,
            opengl_profile,
            win32_keyboard_menu,
            win32_showdefault,
            cocoa_frame_name,
            cocoa_graphics_switching,
            wayland_app_id,
            x11_class_name,
            x11_instance_name,
        } = self;
        /// You can use it like this: `set_window_hint!(bool, name, hint)` for individual hints.
        /// For lots of hints, just do `set_window_hint!( (bool, name, hint), (string, name, hint), and so on)`
        ///
        /// It simply sets hint, check for error and logs if an error occurs.
        macro_rules! set_window_hint {
            ($(($hint_type: tt, $name: ident, $hint: ident),)*) => {
                $(
                    set_window_hint!($hint_type, $name, $hint);
                )*
            };
            (bool, $name: ident, $hint: ident) => {
                set_window_hint!($name, $hint, { bool_to_glfw($name)});
            };
            (i32, $name: ident, $hint: ident) => {
                set_window_hint!($name, $hint, { $name });
            };
            (enum, $name: ident, $hint: ident) => {
                set_window_hint!($name, $hint, { $name as _});
            };
            (string, $name: ident, $hint: ident) => {
                if let Some($name) = $name {
                    let $name =
                        CString::new($name).expect(format!("{} window config (hint) contains null byte", stringify!($name)).as_str());
                    if let Err(e) = el.checked(|| {
                        glfwWindowHintString($hint, $name.as_ptr());
                    }) {
                        error!("failed to set window hint {}: {e:?}", stringify!($name));
                    }
                    std::mem::drop($name);
                }
            };
            ($name: ident, $hint: ident, $conv: block) => {
                if let Some($name) = $name {
                    if let Err(e) = el.checked(|| {
                        glfwWindowHint($hint, $conv);
                    }) {
                        error!("failed to set window hint {}: {e:?}", stringify!($name));
                    }
                }
            };
        }
        unsafe {
            glfwDefaultWindowHints();
            clear_error();
            set_window_hint!(
                (bool, resizable, GLFW_RESIZABLE),
                (bool, visible, GLFW_VISIBLE),
                (bool, decorated, GLFW_DECORATED),
                (bool, focused, GLFW_FOCUSED),
                (bool, auto_iconify, GLFW_AUTO_ICONIFY),
                (bool, floating, GLFW_FLOATING),
                (bool, maximized, GLFW_MAXIMIZED),
                (bool, center_cursor, GLFW_CENTER_CURSOR),
                (bool, transparent_framebuffer, GLFW_TRANSPARENT_FRAMEBUFFER),
                (bool, focus_on_show, GLFW_FOCUS_ON_SHOW),
                (bool, scale_to_monitor, GLFW_SCALE_TO_MONITOR),
                (bool, scale_framebuffer, GLFW_SCALE_FRAMEBUFFER),
                (bool, mouse_passthrough, GLFW_MOUSE_PASSTHROUGH),
                (i32, position_x, GLFW_POSITION_X),
                (i32, position_y, GLFW_POSITION_Y),
                (i32, red_bits, GLFW_RED_BITS),
                (i32, green_bits, GLFW_GREEN_BITS),
                (i32, blue_bits, GLFW_BLUE_BITS),
                (i32, alpha_bits, GLFW_ALPHA_BITS),
                (i32, depth_bits, GLFW_DEPTH_BITS),
                (i32, stencil_bits, GLFW_STENCIL_BITS),
                (i32, accum_red_bits, GLFW_ACCUM_RED_BITS),
                (i32, accum_green_bits, GLFW_ACCUM_GREEN_BITS),
                (i32, accum_blue_bits, GLFW_ACCUM_BLUE_BITS),
                (i32, accum_alpha_bits, GLFW_ACCUM_ALPHA_BITS),
                (i32, aux_buffers, GLFW_AUX_BUFFERS),
                (i32, samples, GLFW_SAMPLES),
                (i32, refresh_rate, GLFW_REFRESH_RATE),
                (bool, stereo, GLFW_STEREO),
                (bool, srgb_capable, GLFW_SRGB_CAPABLE),
                (bool, doublebuffer, GLFW_DOUBLEBUFFER),
                (enum, client_api, GLFW_CLIENT_API),
                (enum, context_creation_api, GLFW_CONTEXT_CREATION_API),
                (i32, context_version_major, GLFW_CONTEXT_VERSION_MAJOR),
                (i32, context_version_minor, GLFW_CONTEXT_VERSION_MINOR),
                (enum, context_robustness, GLFW_CONTEXT_ROBUSTNESS),
                (enum, context_release_behavior, GLFW_CONTEXT_RELEASE_BEHAVIOR),
                (bool, opengl_forward_compat, GLFW_OPENGL_FORWARD_COMPAT),
                (bool, opengl_context_debug, GLFW_OPENGL_DEBUG_CONTEXT),
                (enum, opengl_profile, GLFW_OPENGL_PROFILE),
                (bool, win32_keyboard_menu, GLFW_WIN32_KEYBOARD_MENU),
                (bool, win32_showdefault, GLFW_WIN32_SHOWDEFAULT),
                (string, cocoa_frame_name, GLFW_COCOA_FRAME_NAME),
                (bool, cocoa_graphics_switching, GLFW_COCOA_GRAPHICS_SWITCHING),
                (string, wayland_app_id, GLFW_WAYLAND_APP_ID),
                (string, x11_class_name, GLFW_X11_CLASS_NAME),
                (string, x11_instance_name, GLFW_X11_INSTANCE_NAME),
            );
            Ok(())
        }
    }
}

/// This is data that is shared between [Window] and [WindowProxy]
#[derive(Debug)]
pub(crate) struct WindowData {
    /// the window handle
    pub window: *mut GLFWwindow,
    /// The mutex is used to synchronize all operations related to
    /// making a window current or destroy a window
    ///
    /// The mutex stores the thread id on which the windows was made current most-recently.
    ///
    /// If [Self::is_current] is true, then the window is current on the thread with this particular thread_id.
    pub current_thread: Mutex<ThreadId>,
    /// Whether the window is current *anywhere*.
    /// When the window is made current, the thread_local holds a
    /// strong reference to this data.
    ///
    /// So, anytime the current context is changed (to null or a different window),
    /// this is set to false via that reference.
    ///
    /// And anytime this window is being made current, we check that this is not
    /// already true. If true, then, the current thread id must match the id
    /// in [Self::current_thread]. Otherwise, we must error because it is unsound
    /// to make a window current on a different thread, while it is already current on a thread.
    pub is_current: AtomicBool,
    /// Is set to false when [Window::drop] runs
    /// Usually, you will check for liveness *after* locking the mutex
    pub is_alive: AtomicBool,
    /// The client API with which this window was created.
    /// This is useful to determine if a window is a gl window with [WindowProxy::is_gl_window].
    ///
    /// It never changes after the window is created.
    pub client_api: ClientApi,
    /// Some functions like [WindowProxy::swap_buffers] have additional
    /// restrictions with certain context creation apis like [ContextCreationApi::Egl].
    ///
    /// This also never changes after window is created.
    ///
    /// This is `None` if the window was created with [ClientApi::NoAPI]
    pub context_creation_api: Option<ContextCreationApi>,
}

impl WindowData {
    /// We create a new window data from a window handle
    ///
    /// # Safety
    /// The window handle must be valid
    unsafe fn from_window(window: *mut GLFWwindow, el: &EventLoop) -> Self {
        Self {
            window,
            current_thread: Mutex::new(std::thread::current().id()),
            is_current: AtomicBool::new(false),
            is_alive: AtomicBool::new(true),
            client_api: el
                .checked(|| glfwGetWindowAttrib(window, GLFW_CLIENT_API))
                .expect("failed to query for client api")
                .try_into()
                .expect("invalid client api"),
            context_creation_api: el
                .checked(|| glfwGetWindowAttrib(window, GLFW_CONTEXT_CREATION_API))
                .expect("failed to query for context creation api")
                .try_into()
                .ok(),
        }
    }
}
/// This represents a native Glfw Window. All window-related methods that must be run on main-thread
/// are implemented on this struct.
///
/// We create a window with [Window::new] and we configure a lot of window's properties
/// with [WindowConfig].
///
/// Once we create a window, we can deal with its events via [EventLoop::poll_events] related methods.
///
/// Each Window related event carries a [WindowId], and we can use [WindowProxy::id] function
/// to get the [WindowId] from [Window] and check if the ids match.
///
/// Most apps would just create a single window and use that on the main-thread.
/// So, they rarely need to check the [WindowId].
///
/// If you want to render to the window from a different thread, you can get a [WindowProxy]
/// and send it to another thread. Just remember to read the docs of functions like
/// [WindowProxy::make_current] carefully to avoid crashing (especially if you plan to
/// make the window current on multiple different threads).
///
///
pub struct Window {
    window: *mut GLFWwindow,
    data: Arc<WindowData>,
    weak_window: WindowProxy,
    el: Rc<EventLoop>,
}
impl Drop for Window {
    fn drop(&mut self) {
        clear_error();
        let current_ctx = LOCAL_GL_CONTEXT.with(|ctx| ctx.get_current());
        if current_ctx == Some(self.id()) {
            self.make_uncurrent();
            log_error();
        }
        let guard = if let Ok(current_thread) = self.data.current_thread.try_lock() {
            if self.data.is_current.load(Ordering::Acquire) {
                error!("Window is being destroyed on current thread, but it is still current on an off-thread {current_thread:?}. This is UB.");
            }
            Some(current_thread)
        } else {
            error!("Window is being destroyed on main-thread, but someone else is still using it in off-thread or mutex is poisoned. This could be a bug.");
            None
        };
        self.data.is_alive.store(false, Ordering::Release);
        std::mem::drop(guard);
        log_error();
        unsafe {
            glfwDestroyWindow(self.window);
        }
        log_error();
    }
}

impl Window {
    /// This function creates a window and its associated OpenGL or OpenGL ES context.
    /// Most of the options controlling how the window and its context should be created
    /// are specified with [WindowConfig].
    ///
    /// Successful creation does not change which context is current.
    /// Before you can use the newly created context, you need to make it
    /// current with [WindowProxy::make_current]. For information about the
    /// share parameter, see @ref context_sharing.
    ///
    /// The created window, framebuffer and context may differ from what
    /// you requested, as not all parameters and hints are hard constraints(@ref window_hints_hard).
    /// This includes the size of the window, especially for full screen windows.
    /// To query the actual attributes of the created window, framebuffer and context,
    /// see get_* methods defined on this type.
    ///
    /// To create a full screen window, you need to specify the monitor the window will cover.
    /// If no monitor is specified, the window will be windowed mode. Unless you have a way
    /// for the user to choose a specific monitor, it is recommended that you pick the primary
    /// monitor. For more information on how to query connected monitors, see @ref monitor_monitors.
    ///
    /// For full screen windows, the specified size becomes the resolution of the window's desired
    /// video mode. As long as a full screen window is not iconified, the supported video mode
    /// most closely matching the desired video mode is set for the specified monitor.
    /// For more information about full screen windows, including the creation of so called
    /// windowed full screen or borderless full screen windows, see @ref window_windowed_full_screen.
    ///
    /// Once you have created the window, you can switch it between windowed and full screen mode
    /// with [Self::set_monitor]. This will not affect its OpenGL or OpenGL ES context.
    ///
    /// By default, newly created windows use the placement recommended by the window system.
    /// To create the window at a specific position, set the [WindowConfig::position_x] and
    ///  [WindowConfig::position_y].
    ///
    /// As long as at least one full screen window is not iconified,
    /// the screensaver is prohibited from starting.
    ///
    /// Window systems put limits on window sizes. Very large or very small window dimensions
    /// may be overridden by the window system on creation.
    /// Check the actual size([Window::get_size]) after creation.
    ///
    /// The swap interval is not set during window creation and the initial value
    /// may vary depending on driver settings and defaults.
    #[doc(alias = "glfwCreateWindow")]
    pub fn new(
        el: Rc<EventLoop>,
        config: WindowConfig,
        width: u32,
        height: u32,
        title: &str,
        monitor: Option<MonitorId>,
        parent_window: Option<&Self>,
    ) -> GlfwResult<Self> {
        config.set_hints(&el)?;
        let title = CString::new(title).expect("window title contains null byte");
        if let Some(monitor) = monitor {
            if !el.is_monitor_alive(monitor) {
                return Err(GlfwError::dead_monitor(monitor, "window creation"));
            }
        }
        let window = el.checked(|| unsafe {
            glfwCreateWindow(
                width as _,
                height as _,
                title.as_ptr(),
                monitor.map(|m| m.inner).unwrap_or(std::ptr::null_mut()),
                parent_window
                    .map(|w| w.window)
                    .unwrap_or(std::ptr::null_mut()),
            )
        })?;
        std::mem::drop(title);
        assert!(!window.is_null());
        unsafe { set_window_callbacks(window, el.clone()) };
        let data = Arc::new(unsafe { WindowData::from_window(window, &el) });
        let proxy = el.new_proxy();
        let window = Window {
            window,
            data: data.clone(),
            el,
            weak_window: WindowProxy {
                window,
                data,
                proxy,
            },
        };
        Ok(window)
    }
    /// This function returns the window title, encoded as UTF-8, of the specified window.
    /// This is the title set previously by [Self::new] or [Self::set_title].
    #[doc(alias = "glfwGetWindowTitle")]
    pub fn get_title(&self) -> String {
        let p = self
            .el
            .checked(|| unsafe { glfwGetWindowTitle(self.window) })
            .expect("get window title returned error");
        assert!(!p.is_null());
        unsafe { CStr::from_ptr(p) }.to_string_lossy().to_string()
    }
    /// This function sets the window title, encoded as UTF-8, of the specified window.
    #[doc(alias = "glfwSetWindowTitle")]
    pub fn set_title(&self, title: &str) {
        let title = CString::new(title).expect("window title contains null byte");
        self.el.logged(|| unsafe {
            glfwSetWindowTitle(self.window, title.as_ptr());
        });
        drop(title);
    }
    /*
    #[doc = " @brief Sets the icon for the specified window.\n\n  This function sets the icon of the specified window.  If passed an array of\n  candidate images, those of or closest to the sizes desired by the system are\n  selected.  If no images are specified, the window reverts to its default\n  icon.\n\n  The pixels are 32-bit, little-endian, non-premultiplied RGBA, i.e. eight\n  bits per channel with the red channel first.  They are arranged canonically\n  as packed sequential rows, starting from the top-left corner.\n\n  The desired image sizes varies depending on platform and system settings.\n  The selected images will be rescaled as needed.  Good sizes include 16x16,\n  32x32 and 48x48.\n\n  @param[in] window The window whose icon to set.\n  @param[in] count The number of images in the specified array, or zero to\n  revert to the default window icon.\n  @param[in] images The images to create the icon from.  This is ignored if\n  count is zero.\n\n  @errors Possible errors include @ref GLFW_NOT_INITIALIZED, @ref\n  GLFW_INVALID_VALUE, @ref GLFW_PLATFORM_ERROR and @ref\n  GLFW_FEATURE_UNAVAILABLE (see remarks).\n\n  @pointer_lifetime The specified image data is copied before this function\n  returns.\n\n  @remark @macos Regular windows do not have icons on macOS.  This function\n  will emit @ref GLFW_FEATURE_UNAVAILABLE.  The dock icon will be the same as\n  the application bundle's icon.  For more information on bundles, see the\n  [Bundle Programming Guide][bundle-guide] in the Mac Developer Library.\n\n  [bundle-guide]: https://developer.apple.com/library/mac/documentation/CoreFoundation/Conceptual/CFBundles/\n\n  @remark @wayland There is no existing protocol to change an icon, the\n  window will thus inherit the one defined in the application's desktop file.\n  This function will emit @ref GLFW_FEATURE_UNAVAILABLE.\n\n  @thread_safety This function must only be called from the main thread.\n\n  @sa @ref window_icon\n\n  @since Added in version 3.2.\n\n  @ingroup window"]
    pub fn glfwSetWindowIcon(
        window: *mut GLFWwindow,
        count: ::std::os::raw::c_int,
        images: *const GLFWimage,
    );
    */
    /// This function retrieves the position, in screen coordinates, of the upper-left corner
    /// of the content area of the specified window.
    ///
    /// will error on wayland, as it doesn't support any way to get the window position
    #[doc(alias = "glfwGetWindowPos")]
    pub fn get_pos(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        unsafe { glfwGetWindowPos(self.window, &mut x, &mut y) };
        (x, y)
    }
    /// This function sets the position, in screen coordinates, of the upper-left corner
    /// of the content area of the specified windowed mode window. If the window is a full
    /// screen window, this function does nothing.
    ///
    /// Do not use this function to move an already visible window unless you have very
    /// good reasons for doing so, as it will confuse and annoy the user.
    ///
    /// The window manager may put limits on what positions are allowed.
    /// GLFW cannot and should not override these limits.
    ///
    /// will error on wayland, as it doesn't support any way to set the window position
    #[doc(alias = "glfwSetWindowPos")]
    pub fn set_pos(&self, x: i32, y: i32) {
        unsafe { glfwSetWindowPos(self.window, x, y) };
    }
    /// This function retrieves the size, in screen coordinates, of the content area
    /// of the specified window. If you wish to retrieve the size of the framebuffer
    /// of the window in pixels, see [Self::get_framebuffer_size].
    #[doc(alias = "glfwGetWindowSize")]
    pub fn get_size(&self) -> (i32, i32) {
        let mut width = 0;
        let mut height = 0;
        unsafe { glfwGetWindowSize(self.window, &mut width, &mut height) };

        (width, height)
    }
    /// This function sets the size limits of the content area of the specified window.
    /// If the window is full screen, the size limits only take effect once it is
    /// made windowed. If the window is not resizable, this function does nothing.
    ///
    /// The size limits are applied immediately to a windowed mode window and may
    /// cause it to be resized.
    ///
    /// # Panics
    /// if min_width > max_width || min_height > max_height
    #[doc(alias = "glfwSetWindowSizeLimits")]
    pub fn set_size_limits(
        &self,
        min_width: Option<u32>,
        min_height: Option<u32>,
        max_width: Option<u32>,
        max_height: Option<u32>,
    ) -> GlfwResult<()> {
        assert!(
            min_width <= max_width && min_height <= max_height,
            "min_width: {:?}, min_height: {:?}, max_width: {:?}, max_height: {:?}",
            min_width,
            min_height,
            max_width,
            max_height
        );
        self.el.checked(|| unsafe {
            glfwSetWindowSizeLimits(
                self.window,
                min_width.map(|v| v as _).unwrap_or(GLFW_DONT_CARE),
                min_height.map(|v| v as _).unwrap_or(GLFW_DONT_CARE),
                max_width.map(|v| v as _).unwrap_or(GLFW_DONT_CARE),
                max_height.map(|v| v as _).unwrap_or(GLFW_DONT_CARE),
            )
        })
    }
    /// This function sets the required aspect ratio of the content area of the specified window. If the window is full screen, the aspect ratio only takes effect once it is made windowed. If the window is not resizable, this function does nothing.
    /// The aspect ratio is specified as a numerator and a denominator and both values must be greater than zero. For example, the common 16:9 aspect ratio is specified as 16 and 9, respectively.
    /// If the numerator and denominator is set to None then the aspect ratio limit is disabled.
    /// The aspect ratio is applied immediately to a windowed mode window and may cause it to be resized.
    #[doc(alias = "glfwSetWindowAspectRatio")]
    pub fn set_aspect_ratio(&self, numer: Option<u32>, denom: Option<u32>) -> GlfwResult<()> {
        self.el.checked(|| unsafe {
            glfwSetWindowAspectRatio(
                self.window,
                numer.map(|v| v as _).unwrap_or(GLFW_DONT_CARE),
                denom.map(|v| v as _).unwrap_or(GLFW_DONT_CARE),
            )
        })
    }
    /// This function sets the size, in screen coordinates, of the content area of the specified window.
    /// For full screen windows, this function updates the resolution of its desired video mode and switches to the video mode closest to it, without affecting the window's context. As the context is unaffected, the bit depths of the framebuffer remain unchanged.
    /// If you wish to update the refresh rate of the desired video mode in addition to its resolution, see @ref glfwSetWindowMonitor.
    /// The window manager may put limits on what sizes are allowed. GLFW cannot and should not override these limits.
    #[doc(alias = "glfwSetWindowSize")]
    pub fn set_size(&self, width: u32, height: u32) {
        unsafe { glfwSetWindowSize(self.window, width as _, height as _) }
    }
    /// This function retrieves the size, in pixels, of the framebuffer of the specified window. If you wish to retrieve the size of the window in screen coordinates, see @ref glfwGetWindowSize.
    #[doc(alias = "glfwGetFramebufferSize")]
    pub fn get_framebuffer_size(&self) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;
        unsafe { glfwGetFramebufferSize(self.window, &mut width, &mut height) };
        (width.try_into().unwrap(), height.try_into().unwrap())
    }
    /// This function retrieves the size, in screen coordinates, of each edge of the frame of the specified window. This size includes the title bar, if the window has one. The size of the frame may vary depending on the window-related hints(@ref window_hints_wnd) used to create it.
    /// Because this function retrieves the size of each window frame edge and not the offset along a particular coordinate axis, the retrieved values will always be zero or positive.
    #[doc(alias = "glfwGetWindowFrameSize")]
    pub fn get_frame_size(&self) -> (u32, u32, u32, u32) {
        let mut left = 0;
        let mut top = 0;
        let mut right = 0;
        let mut bottom = 0;
        unsafe {
            glfwGetWindowFrameSize(self.window, &mut left, &mut top, &mut right, &mut bottom)
        };
        (
            left.try_into().unwrap(),
            top.try_into().unwrap(),
            right.try_into().unwrap(),
            bottom.try_into().unwrap(),
        )
    }
    /// This function retrieves the content scale for the specified window. The content scale is the ratio between the current DPI and the platform's default DPI. This is especially important for text and any UI elements. If the pixel dimensions of your UI scaled by this look appropriate on your machine then it should appear at a reasonable size on other machines regardless of their DPI and scaling settings. This relies on the system DPI and scaling settings being somewhat correct.
    /// On platforms where each monitors can have its own content scale, the window content scale will depend on which monitor the system considers the window to be on.
    #[doc(alias = "glfwGetWindowContentScale")]
    pub fn get_content_scale(&self) -> (f32, f32) {
        let mut xscale = 0.0;
        let mut yscale = 0.0;
        unsafe { glfwGetWindowContentScale(self.window, &mut xscale, &mut yscale) };
        (xscale, yscale)
    }
    /// This function returns the opacity of the window, including any decorations.
    /// The opacity (or alpha) value is a positive finite number between zero and one, where zero is fully transparent and one is fully opaque. If the system does not support whole window transparency, this function always returns one.
    /// The initial opacity value for newly created windows is one.
    #[doc(alias = "glfwGetWindowOpacity")]
    pub fn get_opacity(&self) -> f32 {
        unsafe { glfwGetWindowOpacity(self.window) }
    }
    /**
    This function sets the opacity of the window, including any decorations.

    The opacity (or alpha) value is a positive finite number between zero and one, where zero is fully transparent and one is fully opaque.

    The initial opacity value for newly created windows is one.

    A window created with framebuffer transparency may not use whole window transparency. The results of doing this are undefined.

    Not supported on wayland.
    */
    #[doc(alias = "glfwSetWindowOpacity")]
    pub fn set_opacity(&self, opacity: f32) {
        unsafe { glfwSetWindowOpacity(self.window, opacity) }
    }
    /**
    This function iconifies (minimizes) the specified window if it was previously restored. If the window is already iconified, this function does nothing.

    If the specified window is a full screen window, GLFW restores the original video mode of the monitor. The window's desired video mode is set again when the window is restored. */
    #[doc(alias = "glfwIconifyWindow")]
    pub fn iconify(&self) {
        unsafe { glfwIconifyWindow(self.window) }
    }
    /**
    This function restores the specified window if it was previously iconified (minimized) or maximized. If the window is already restored, this function does nothing.

    If the specified window is an iconified full screen window, its desired video mode is set again for its monitor when the window is restored. */
    #[doc(alias = "glfwRestoreWindow")]
    pub fn restore(&self) {
        unsafe { glfwRestoreWindow(self.window) }
    }
    /**
    This function maximizes the specified window if it was previously not maximized. If the window is already maximized, this function does nothing.

    If the specified window is a full screen window, this function does nothing.*/
    #[doc(alias = "glfwMaximizeWindow")]
    pub fn maximize(&self) {
        unsafe { glfwMaximizeWindow(self.window) }
    }
    /**
    This function makes the specified window visible if it was previously hidden. If the window is already visible or is in full screen mode, this function does nothing.

    By default, windowed mode windows are focused when shown Set the [WindowConfig::focus_on_show] window hint to change this behavior for all newly created windows, or change the behavior for an existing window with [Self::set_focus_on_show].

    Because Wayland wants every frame of the desktop to be complete, this function does not immediately make the window visible. Instead it will become visible the next time the window framebuffer is updated after this call.
     */
    #[doc(alias = "glfwShowWindow")]
    pub fn show(&self) {
        unsafe { glfwShowWindow(self.window) }
    }
    /** This function hides the specified window if it was previously visible. If the window is already hidden or is in full screen mode, this function does nothing. */
    #[doc(alias = "glfwHideWindow")]
    pub fn hide(&self) {
        unsafe { glfwHideWindow(self.window) }
    }
    /**
    This function brings the specified window to front and sets input focus. The window should already be visible and not iconified.

    By default, both windowed and full screen mode windows are focused when initially created. Set the GLFW_FOCUSED(@ref GLFW_FOCUSED_hint) to disable this behavior.

    Also by default, windowed mode windows are focused when shown with @ref glfwShowWindow. Set the GLFW_FOCUS_ON_SHOW(@ref GLFW_FOCUS_ON_SHOW_hint) to disable this behavior.

    Do not use this function to steal focus from other applications unless you are certain that is what the user wants. Focus stealing can be extremely disruptive.

    For a less disruptive way of getting the user's attention, see attention requests(@ref window_attention). */
    #[doc(alias = "glfwFocusWindow")]
    pub fn focus(&self) {
        unsafe { glfwFocusWindow(self.window) }
    }
    /**
    This function requests user attention to the specified window. On platforms where this is not supported, attention is requested to the application as a whole.

    Once the user has given attention, usually by focusing the window or application, the system will end the request automatically.*/
    #[doc(alias = "glfwRequestWindowAttention")]
    pub fn request_attention(&self) {
        unsafe { glfwRequestWindowAttention(self.window) }
    }
    /// This function returns the handle of the monitor that the specified window
    /// is in full screen on.
    ///
    /// None if window is in windowed mode
    #[doc(alias = "glfwGetWindowMonitor")]
    pub fn get_monitor(&self) -> Option<MonitorId> {
        let monitor = unsafe { glfwGetWindowMonitor(self.window) };
        if monitor.is_null() {
            return None;
        }
        MAIN_THREAD_LOCAL_DATA.with(|data| {
            data.monitors.borrow_mut().insert(monitor);
        });
        MonitorId::new(monitor)
    }
    /**
    This function sets the monitor that the window uses for full screen mode or,
    if the monitor is None, makes it windowed mode.

    When setting a monitor, this function updates the width, height and
    refresh rate of the desired video mode and switches to the video mode closest
    to it. The window position is ignored when setting a monitor.

    When the monitor is None, the position, width and height are used to place the
    window content area. The refresh rate is ignored when no monitor is specified.

    If you only wish to update the resolution of a full screen window or the
    size of a windowed mode window, see [Self::set_size].

    When a window transitions from full screen to windowed mode,
    this function restores any previous window settings such as whether it
    is decorated, floating, resizable, has size or aspect ratio limits, etc.

    Returns error, if monitor is not connected anymore.
    */
    #[doc(alias = "glfwSetWindowMonitor")]
    pub fn set_monitor(
        &self,
        monitor: Option<MonitorId>,
        xpos: i32,
        ypos: i32,
        width: u32,
        height: u32,
        refresh_rate: Option<u32>,
    ) -> GlfwResult<()> {
        if let Some(monitor) = monitor {
            if !self.el.is_monitor_alive(monitor) {
                return Err(GlfwError::dead_monitor(monitor, "Window::set_monitor"));
            }
        }
        unsafe {
            glfwSetWindowMonitor(
                self.window,
                monitor.map(|m| m.inner).unwrap_or(std::ptr::null_mut()),
                xpos,
                ypos,
                width as _,
                height as _,
                refresh_rate.map(|r| r as _).unwrap_or(GLFW_DONT_CARE),
            )
        }
        Ok(())
    }
    pub fn get_focused(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_FOCUSED) == GLFW_TRUE }
    }
    pub fn get_iconified(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_ICONIFIED) == GLFW_TRUE }
    }
    pub fn get_maximized(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_MAXIMIZED) == GLFW_TRUE }
    }
    pub fn get_hovered(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_HOVERED) == GLFW_TRUE }
    }
    pub fn get_visible(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_VISIBLE) == GLFW_TRUE }
    }
    pub fn get_resizeable(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_RESIZABLE) == GLFW_TRUE }
    }
    pub fn get_decorated(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_DECORATED) == GLFW_TRUE }
    }
    pub fn get_auto_iconify(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_AUTO_ICONIFY) == GLFW_TRUE }
    }
    pub fn get_floating(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_FLOATING) == GLFW_TRUE }
    }
    pub fn get_transparent_framebuffer(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_TRANSPARENT_FRAMEBUFFER) == GLFW_TRUE }
    }
    pub fn get_focus_on_show(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_FOCUS_ON_SHOW) == GLFW_TRUE }
    }
    pub fn get_mouse_passthrough(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_MOUSE_PASSTHROUGH) == GLFW_TRUE }
    }
    pub fn get_client_api(&self) -> ClientApi {
        unsafe {
            glfwGetWindowAttrib(self.window, GLFW_CLIENT_API)
                .try_into()
                .expect("Invalid client api")
        }
    }
    pub fn get_context_creation_api(&self) -> ContextCreationApi {
        unsafe {
            glfwGetWindowAttrib(self.window, GLFW_CONTEXT_CREATION_API)
                .try_into()
                .expect("Invalid context creation api")
        }
    }
    pub fn get_context_version_major(&self) -> i32 {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_CONTEXT_VERSION_MAJOR) }
    }
    pub fn get_context_version_minor(&self) -> i32 {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_CONTEXT_VERSION_MINOR) }
    }
    pub fn get_context_revision(&self) -> i32 {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_CONTEXT_REVISION) }
    }
    pub fn get_opengl_forward_compat(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_OPENGL_FORWARD_COMPAT) == GLFW_TRUE }
    }
    pub fn get_context_debug(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_CONTEXT_DEBUG) == GLFW_TRUE }
    }
    pub fn get_opengl_profile(&self) -> OpenGLProfile {
        unsafe {
            glfwGetWindowAttrib(self.window, GLFW_OPENGL_PROFILE)
                .try_into()
                .unwrap()
        }
    }
    pub fn get_context_release_behavior(&self) -> ContextReleaseBehavior {
        unsafe {
            glfwGetWindowAttrib(self.window, GLFW_CONTEXT_RELEASE_BEHAVIOR)
                .try_into()
                .unwrap()
        }
    }
    pub fn get_context_no_error(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_CONTEXT_NO_ERROR) == GLFW_TRUE }
    }
    pub fn get_context_robustness(&self) -> Robustness {
        unsafe {
            glfwGetWindowAttrib(self.window, GLFW_CONTEXT_ROBUSTNESS)
                .try_into()
                .unwrap()
        }
    }
    pub fn get_doublebuffer(&self) -> bool {
        unsafe { glfwGetWindowAttrib(self.window, GLFW_DOUBLEBUFFER) == GLFW_TRUE }
    }
    pub fn set_resizeable(&self, resizeable: bool) {
        unsafe {
            glfwSetWindowAttrib(self.window, GLFW_RESIZABLE, bool_to_glfw(resizeable));
        }
    }
    pub fn set_decorated(&self, decorated: bool) {
        unsafe {
            glfwSetWindowAttrib(self.window, GLFW_DECORATED, bool_to_glfw(decorated));
        }
    }
    pub fn set_auto_iconify(&self, auto_iconify: bool) {
        unsafe {
            glfwSetWindowAttrib(self.window, GLFW_AUTO_ICONIFY, bool_to_glfw(auto_iconify));
        }
    }
    pub fn set_floating(&self, floating: bool) {
        unsafe {
            glfwSetWindowAttrib(self.window, GLFW_FLOATING, bool_to_glfw(floating));
        }
    }
    pub fn set_focus_on_show(&self, focus_on_show: bool) {
        unsafe {
            glfwSetWindowAttrib(self.window, GLFW_FOCUS_ON_SHOW, bool_to_glfw(focus_on_show));
        }
    }

    pub fn set_mouse_passthrough(&self, mouse_passthrough: bool) {
        unsafe {
            glfwSetWindowAttrib(
                self.window,
                GLFW_MOUSE_PASSTHROUGH,
                bool_to_glfw(mouse_passthrough),
            );
        }
    }
    /// This function sets the cursor image to be used when the cursor
    /// is over the content area of the specified window.
    /// The set cursor will only be visible when the cursor mode(@ref cursor_mode)
    /// of the window is GLFW_CURSOR_NORMAL.
    ///
    /// On some platforms, the set cursor may not be visible unless the window
    /// also has input focus.
    pub fn set_cursor(&self, cursor: Option<&Cursor>) {
        unsafe {
            glfwSetCursor(
                self.window,
                cursor.map_or(std::ptr::null_mut(), |c| c.get_ptr()),
            );
        }
    }
    /// Read [CursorMode] docs
    #[doc(alias = "glfwSetInputMode")]
    pub fn set_cursor_mode(&self, mode: CursorMode) {
        unsafe {
            glfwSetInputMode(self.window, GLFW_CURSOR, mode as i32);
        }
    }
    /// Read [CursorMode] docs
    #[doc(alias = "glfwGetInputMode")]
    pub fn get_cursor_mode(&self) -> CursorMode {
        unsafe { glfwGetInputMode(self.window, GLFW_CURSOR) }
            .try_into()
            .unwrap()
    }

    /// If sticky keys are enabled, a key press will ensure that [get_key](Self::get_key)
    /// returns true. the next time it is called even if the key had been
    /// released before the call. This is useful when you are only interested in
    /// whether keys have been pressed but not when or in which order.
    #[doc(alias = "glfwSetInputMode")]
    pub fn set_sticky_keys(&self, sticky_keys: bool) {
        unsafe {
            glfwSetInputMode(self.window, GLFW_STICKY_KEYS, bool_to_glfw(sticky_keys));
        }
    }
    /// see docs of [set_sticky_keys](Self::set_sticky_keys)
    #[doc(alias = "glfwGetInputMode")]
    pub fn get_sticky_keys(&self) -> bool {
        unsafe { glfwGetInputMode(self.window, GLFW_STICKY_KEYS) == GLFW_TRUE }
    }
    /// If sticky mouse buttons are enabled, a mouse button press will ensure that
    /// [get_mouse_button](Self::get_mouse_button) returns true the next time
    /// it is called even if the mouse button had been released before the call.
    /// This is useful when you are only interested in whether mouse buttons
    /// have been pressed but not when or in which order.
    #[doc(alias = "glfwSetInputMode")]
    pub fn set_sticky_mouse_buttons(&self, sticky_mouse_buttons: bool) {
        unsafe {
            glfwSetInputMode(
                self.window,
                GLFW_STICKY_MOUSE_BUTTONS,
                bool_to_glfw(sticky_mouse_buttons),
            );
        }
    }
    /// see docs of [set_sticky_mouse_buttons](Self::set_sticky_mouse_buttons)
    #[doc(alias = "glfwGetInputMode")]
    pub fn get_sticky_mouse_buttons(&self) -> bool {
        unsafe { glfwGetInputMode(self.window, GLFW_STICKY_MOUSE_BUTTONS) == GLFW_TRUE }
    }
    /// If enabled, callbacks that receive modifier bits will also have
    /// the [CAPS_LOCK](Modifiers::CAPS_LOCK) bit set when the event was
    /// generated with Caps Lock on, and the [NUM_LOCK](Modifiers::NUM_LOCK)
    /// bit when Num Lock was on.
    #[doc(alias = "glfwSetInputMode")]
    pub fn set_lock_key_mods(&self, lock_key_mods: bool) {
        unsafe {
            glfwSetInputMode(self.window, GLFW_LOCK_KEY_MODS, bool_to_glfw(lock_key_mods));
        }
    }
    /// see docs of [set_lock_key_mods](Self::set_lock_key_mods)
    #[doc(alias = "glfwGetInputMode")]
    pub fn get_lock_key_mods(&self) -> bool {
        unsafe { glfwGetInputMode(self.window, GLFW_LOCK_KEY_MODS) == GLFW_TRUE }
    }
    /// set to true to enable raw (unscaled and unaccelerated) mouse motion
    /// when the cursor is disabled, or false to disable it.
    /// If raw motion is not supported, attempting to set this
    /// will emit [ErrorCode::FeatureUnavailable].
    /// See docs of [is_raw_mouse_motion_supported](EventLoop::is_raw_mouse_motion_supported).
    #[doc(alias = "glfwSetInputMode")]
    pub fn set_raw_mouse_motion(&self, raw_mouse_motion: bool) {
        unsafe {
            glfwSetInputMode(
                self.window,
                GLFW_RAW_MOUSE_MOTION,
                bool_to_glfw(raw_mouse_motion),
            );
        }
    }
    /// see docs of [set_raw_mouse_motion](Self::set_raw_mouse_motion)
    #[doc(alias = "glfwGetInputMode")]
    pub fn get_raw_mouse_motion(&self) -> bool {
        unsafe { glfwGetInputMode(self.window, GLFW_RAW_MOUSE_MOTION) == GLFW_TRUE }
    }
    /**
    This function returns the last state reported for the specified key
    to the specified window. The repeat action is only
    reported in [Event::Key::repeat].

    If [Window::set_sticky_keys] is enabled, this function returns true
    the first time you call it for a key that was pressed, even if that key has
    already been released.

    The key functions deal with physical keys, with key tokens([Key])
    named after their use on the standard US keyboard layout. If you want
    to input text, use the [Unicode character event](Event::Char) instead.

    The modifier key bit masks([Modifiers]) are not key tokens and cannot
    be used with this function.

    Do not use this function to implement text input([Event::Char]).

    returns true if pressed. returns false, if released.
    */
    #[doc(alias = "glfwGetKey")]
    pub fn get_key(&self, key: Key) -> bool {
        match unsafe { glfwGetKey(self.window, key as _) } {
            GLFW_PRESS => true,
            GLFW_RELEASE => false,
            _ => unreachable!(),
        }
    }
    /// This function returns the last state reported for the specified
    /// mouse button to the specified window. The returned state true for pressed
    /// and false for released.
    ///
    /// If the [Window::set_sticky_mouse_buttons] is enabled,
    /// this function returns true the first time you call it for a mouse
    /// button that was pressed, even if that mouse button has already been released.
    #[doc(alias = "glfwGetMouseButton")]
    pub fn get_mouse_button(&self, button: MouseButton) -> bool {
        match unsafe { glfwGetMouseButton(self.window, button as _) } {
            GLFW_PRESS => true,
            GLFW_RELEASE => false,
            _ => unreachable!(),
        }
    }
    /**
    This function returns the position of the cursor, in screen coordinates,
    relative to the upper-left corner of the content area of the specified window.

    If the cursor is disabled (with [CursorMode::Disabled]) then the cursor
    position is unbounded and limited only by the minimum and maximum
    values of a double.

    The coordinate can be converted to their integer equivalents with the floor
    function. Casting directly to an integer type works for positive coordinates,
    but fails for negative ones.
    */
    #[doc(alias = "glfwGetCursorPos")]
    pub fn get_cursor_pos(&self) -> (f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        unsafe { glfwGetCursorPos(self.window, &mut x, &mut y) };
        (x, y)
    }
    /**
    This function sets the position, in screen coordinates, of the
    cursor relative to the upper-left corner of the content area of
    the specified window. The window must have input focus. If the window does
    not have input focus when this function is called, it fails silently.

    Do not use this function to implement things like camera controls. GLFW
    already provides the [CursorMode::Disabled] that hides the cursor,
    transparently re-centers it and provides unconstrained cursor motion.
    See [Window::set_cursor_mode] for more information.

    If the cursor mode is [CursorMode::Disabled] then the cursor position
    is unconstrained and limited only by the minimum and maximum values of a double.

    **Wayland**: This function will only work when the cursor mode is
    [CursorMode::Disabled], otherwise it will emit [ErrorCode::FeatureUnavailable].
    */
    #[doc(alias = "glfwSetCursorPos")]
    pub fn set_cursor_pos(&self, x: f64, y: f64) {
        unsafe { glfwSetCursorPos(self.window, x, y) };
    }
    /// This function sets the system clipboard to the specified, UTF-8 encoded string.
    ///
    /// # Panics
    /// If the string has null-byte.
    #[doc(alias = "glfwSetClipboardString")]
    pub fn set_clipboard_string(&self, s: &str) {
        let s = CString::new(s).unwrap();
        unsafe {
            glfwSetClipboardString(self.window, s.as_ptr() as _);
        }
        drop(s);
    }
    /// This function returns the contents of the system clipboard,
    /// if it contains or is convertible to a UTF-8 encoded string.
    /// If the clipboard is empty or if its contents cannot be converted,
    /// Err is returned and a [ErrorCode::FormatUnavailable] error is generated.
    #[doc(alias = "glfwGetClipboardString")]
    pub fn get_clipboard_string(&self) -> GlfwResult<String> {
        self.el.checked(|| unsafe {
            let p = glfwGetClipboardString(self.window);
            assert!(!p.is_null());
            CStr::from_ptr(p).to_string_lossy().to_string()
        })
    }
}
// pub fn should_close(&self) -> bool {
//     let guard = self.data.current_thread.lock().unwrap();
//     let should_close = unsafe { glfwWindowShouldClose(self.window) };
//     drop(guard);
//     assert_no_error();
//     should_close == GLFW_TRUE
// }
// pub fn set_should_close(&self, should_close: bool) {
//     let guard = self.data.current_thread.lock().unwrap();
//     unsafe {
//         glfwSetWindowShouldClose(self.window, bool_to_glfw(should_close));
//     }
//     drop(guard);
//     assert_no_error();
// }

impl Deref for Window {
    type Target = WindowProxy;
    fn deref(&self) -> &Self::Target {
        &self.weak_window
    }
}
impl AsRef<EventLoop> for Window {
    fn as_ref(&self) -> &EventLoop {
        &self.el
    }
}
/// A weak reference to a window. [Window] also derefs to [WindowProxy], so any function
/// defined on this is also callable on [Window].
///
/// This is useful to send to other threads, where you can make window/gl current and render to it.
/// But the main [Window] must stay on main-thread for the purposes of event loop.
///
/// The simplest use-case is creating a [Window] on main-thread, making it current and just
/// rendering to it. This is the happy path with very little work for anyone.
/// None of the window's methods will panic, because it is alive and always current.
///
/// But if you are going to need multiple windows or have separate rendering threads where
/// you want to make a [WindowProxy] current, you need to be just a little bit careful. Read
/// the rest of the docs to become aware of any potential pitfalls.
///
/// We try to keep our bindings safe, so make sure you enable tracing, as we log
/// any serious errors using `tracing::error!`. It will warn you of any mistakes/UB.
///
/// Pretty much ALL of this struct's methods will panic if the window is not alive.
/// So, you are responsible for ensuring that the window is alive, as long as you are using this.
///
///
/// Window will also panic, if it is current on a thread, but
/// you try to make it current on a different thread.
///
/// A window must also never be destroyed while it is current on a thread. So,
/// try to always make a window uncurrent before destroying it.
///
/// Generally speaking, we only panic in those cases. Most functions will prefer to
/// ignore an error and you are supposed to rely on [EventLoopConfig::error_callback] for
/// any other errors. Or use [get_error] to check for errors explicitly after a call.
///
/// The one exception to the above rule is if window is current on main-thread.
/// In that case, the [Window]'s drop will automatically make it uncurrent. This
/// is just for the happy path convenience.
#[derive(Debug, Clone)]
pub struct WindowProxy {
    window: *mut GLFWwindow,
    data: Arc<WindowData>,
    proxy: EventLoopProxy,
}
impl Deref for WindowProxy {
    type Target = EventLoopProxy;
    fn deref(&self) -> &Self::Target {
        &self.proxy
    }
}
impl WindowProxy {
    /// 1. This locks the mutex of the window
    /// 2. asserts that the window is alive
    /// 3. runs the `work` closure and returns value.
    ///
    /// This ensures that we only ever call glfw FFI functions when the window is alive.
    fn with_checked<T>(&self, work: impl FnOnce() -> T) -> T {
        let guard = self.data.current_thread.lock().unwrap();
        assert!(self.data.is_alive.load(Ordering::Acquire));
        let result = work();
        drop(guard);
        result
    }
    /// 1. asserts that the window is current on the calling thread
    /// 2. calls [WindowProxy::with_checked] to run the `work` closure
    ///
    /// useful for functions like [Self::swap_buffers] which only
    /// work if the window is current.
    fn with_current_checked<T>(&self, work: impl FnOnce() -> T) -> T {
        assert!(self.is_current_on_current_thread());
        self.with_checked(work)
    }
    /// Sets the window's should-close flag.
    pub fn set_should_close(&self, should_close: bool) {
        self.with_checked(|| unsafe {
            glfwSetWindowShouldClose(self.window, bool_to_glfw(should_close));
        })
    }
    /// Returns true if the window's should-close flag is set.
    pub fn should_close(&self) -> bool {
        self.with_checked(|| unsafe { glfwWindowShouldClose(self.window) }) == GLFW_TRUE
    }
    /// The ID of the window. You need this for event-handling,
    /// as events come with the target window id attached.
    ///
    /// This doesn't change during the entire lifetime of the window.
    pub fn id(&self) -> WindowId {
        WindowId(self.window)
    }
    /// Returns true if the window was created with a gl context
    pub fn is_gl_window(&self) -> bool {
        self.data.client_api != ClientApi::NoAPI
    }
    /// Makes the window's opengl context current on the calling thread.
    ///
    /// # Panics
    /// * if the window is already current on a different thread.
    /// * if the window was not created with a gl context
    pub fn make_current(&self) {
        assert!(self.is_gl_window());
        LOCAL_GL_CONTEXT.with(|ctx| ctx.make_current(self.data.clone()))
    }
    /// Makes this window uncurrent IF and ONLY IF it is current on the calling thread.
    /// otherwise, leaves the current context unchanged.
    ///
    /// # Panics
    /// 1. if this is not an opengl window
    pub fn make_uncurrent(&self) {
        assert!(self.is_gl_window());
        // why bother making it uncurrent, when it already isn't current.
        if !self.data.is_current.load(Ordering::Acquire) {
            return;
        }
        LOCAL_GL_CONTEXT.with(|ctx| ctx.make_uncurrent(Some(self.data.clone())))
    }
    /// If the window is current on *any* thread, returns true, else returns false
    pub fn is_current_somewhere(&self) -> bool {
        self.data.is_current.load(Ordering::Acquire)
    }
    /// If the window is current on the calling thread, returns true
    pub fn is_current_on_current_thread(&self) -> bool {
        LOCAL_GL_CONTEXT.with(|ctx| ctx.get_current() == Some(self.id()))
    }
    ///
    /// This function swaps the front and back buffers of the specified window
    /// when rendering with OpenGL or OpenGL ES. If the swap interval is greater
    /// than zero, the GPU driver waits the specified number of screen updates
    /// before swapping the buffers.
    ///
    /// see [Self::set_swap_interval].
    ///
    /// # Panics
    /// * **egl only**: if the window is not current on the calling thread, as egl requires being current for swap buffers to work
    pub fn swap_buffers(&self) {
        if self.data.context_creation_api == Some(ContextCreationApi::Egl) {
            assert!(self.is_current_on_current_thread());
        }
        self.with_checked(|| unsafe { glfwSwapBuffers(self.window) })
    }
    /// Returns opengl function pointer for `proc_name`.
    ///
    /// See [Self::get_proc_addr_cstr] if you want to use a `&CStr` for `proc_name`.
    /// This function just internally calls that function, after creating a `CString` out
    /// of `proc_name`.
    ///
    /// # Panics
    /// * if the window is not current on the calling thread
    /// * if proc-name contains a null-byte (which is not allowed for C strings)
    pub fn get_proc_addr(&self, proc_name: &str) -> *mut std::ffi::c_void {
        let proc_name = CString::new(proc_name).expect("proc_name contains null-byte");
        self.get_proc_addr_cstr(&proc_name)
    }
    /// Returns opengl function pointer for `proc_name`.
    ///
    /// If you have a `&str` for `proc_name`, use [Self::get_proc_addr] instead.
    ///
    /// # Panics
    /// * if the window is not current on the calling thread
    pub fn get_proc_addr_cstr(&self, proc_name: &CStr) -> *mut std::ffi::c_void {
        self.with_current_checked(|| unsafe {
            match glfwGetProcAddress(proc_name.as_ptr()) {
                Some(ptr) => ptr as _,
                None => std::ptr::null_mut(),
            }
        })
    }
    /// This function sets the swap interval for the current OpenGL or OpenGL ES context,
    /// i.e. the number of screen updates to wait from the time
    /// [Self::swap_buffers] was called before swapping the buffers
    /// and returning. This is sometimes called vertical synchronization,
    /// vertical retrace synchronization or just vsync.
    ///
    /// A context that supports either of the WGL_EXT_swap_control_tear and
    /// GLX_EXT_swap_control_tear extensions also accepts negative swap intervals,
    /// which allows the driver to swap immediately even if a frame arrives
    /// a little bit late. You can check for these extensions with
    /// [Self::extension_supported].
    ///
    /// This function does not apply to Vulkan. If you are rendering with Vulkan, see the present mode of your swapchain instead.
    ///
    /// # Panics
    /// * if the window is not current on the calling thread
    pub fn set_swap_interval(&self, interval: i32) {
        self.with_current_checked(|| unsafe { glfwSwapInterval(interval) })
    }
    /// This function returns whether the specified API extension(@ref context_glext) is supported by the current OpenGL or OpenGL ES context. It searches both for client API extension and context creation API extensions.
    ///
    /// A context must be current on the calling thread. Calling this function without a
    ///  current context will cause a [ErrorCode::NoCurrentContext] error.
    ///
    /// As this functions retrieves and searches one or more extension strings each call,
    /// it is recommended that you cache its results if it is going to be used frequently.
    /// The extension strings will not change during the lifetime of a context, so there
    /// is no danger in doing this.
    ///
    /// If you have a `&CStr` for `extension`, use [Self::extension_supported_cstr] instead.
    /// Internally, this calls that function too.
    /// 
    /// # Panics
    /// * if the window is not current on the calling thread
    /// * if `extension` contains a null-byte, which is not allowed for C strings
    #[doc(alias = "glfwExtensionSupported")]
    pub fn extension_supported(&self, extension: &str) -> bool {
        let extension = CString::new(extension).expect("extension contains null-byte");
        self.extension_supported_cstr(&extension)
    }
    /// read [Self::extension_supported] docs
    ///
    /// # Panics
    /// * if the window is not current on the calling thread
    #[doc(alias = "glfwExtensionSupported")]
    pub fn extension_supported_cstr(&self, extension: &CStr) -> bool {
        self.with_current_checked(|| unsafe {
            glfwExtensionSupported(extension.as_ptr()) == GLFW_TRUE
        })
    }

    /// This function creates a Vulkan surface for the specified window.
    ///
    /// If the Vulkan loader or at least one minimally functional ICD were
    /// not found, this function returns VK_ERROR_INITIALIZATION_FAILED and generates
    /// a [ErrorCode::ApiUnavailable] error. Call [EventLoopProxy::is_vulkan_supported] to check
    /// whether Vulkan is at least minimally available.
    ///
    /// If the required window surface creation instance extensions are not
    /// available or if the specified instance was not created with these extensions
    /// enabled, this function returns VK_ERROR_EXTENSION_NOT_PRESENT and generates
    /// a [ErrorCode::ApiUnavailable] error. Call [EventLoopProxy::get_required_instance_extensions]
    /// to check what instance extensions are required.
    ///
    /// The window surface cannot be shared with another API so the window must have
    /// been created with the client api hint set to [ClientApi::NoAPI] otherwise
    /// it generates a [ErrorCode::InvalidValue] error and returns VK_ERROR_NATIVE_WINDOW_IN_USE_KHR.
    ///
    /// The window surface must be destroyed before the specified Vulkan instance.
    /// It is the responsibility of the caller to destroy the window surface.
    /// GLFW does not destroy it for you. Call vkDestroySurfaceKHR to destroy the surface.
    ///
    /// It returns Ok([VkSurfaceKHR]) on success.
    /// or Err([VkResult]) on failure.
    ///
    /// # Panics
    /// * If the client api is not [ClientApi::NoAPI]
    #[doc(alias = "glfwCreateWindowSurface")]
    pub unsafe fn create_window_surface(
        &self,
        instance: VkInstance,
        allocator: Option<*const VkAllocationCallbacks>,
    ) -> Result<VkSurfaceKHR, VkResult> {
        assert!(self.data.client_api == ClientApi::NoAPI);
        let mut surface: VkSurfaceKHR = std::ptr::null_mut();
        let result = self.with_checked(|| unsafe {
            glfwCreateWindowSurface(
                instance,
                self.window,
                allocator.unwrap_or(std::ptr::null()),
                &mut surface,
            )
        });
        if result == VkResult_VK_SUCCESS {
            Ok(surface)
        } else {
            Err(result)
        }
    }
}

/// Id of a [Window].
///
/// This is just a pointer (`*mut Glfwwindow`) and you can get the pointer
/// by calling [Self::get_ptr].
///
/// This is mainly used to identify which window a [Event] is targeting and
/// also to find out which window is current on a thread via [EventLoopProxy::get_any_current].
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct WindowId(*mut GLFWwindow);
impl WindowId {
    /// Creates a new [WindowId] from a pointer to a `GLFWwindow`.
    ///
    /// Returns `None` if the pointer is null.
    pub fn new(window: *mut GLFWwindow) -> Option<Self> {
        if window.is_null() {
            return None;
        }
        Some(Self(window))
    }
    /// provides the pointer stored inside.
    pub fn get_ptr(self) -> *mut GLFWwindow {
        self.0
    }
}
/// This sets the window callbacks for a [Window]
unsafe fn set_window_callbacks(window: *mut GLFWwindow, el: Rc<EventLoop>) {
    /// A macro to avoid writing the same callback setting code for each callback
    ///
    /// The syntax is simply the variable name of `*mut GLFWwindow` and a list of tuples:
    ///
    /// Each tuple contains:
    /// 1. the name of the function that sets the callback
    /// 2. the name of actual callback function.
    ///
    /// It will try to set the callback, but if there's an error, it will log that error
    /// using the human-readable name.
    ///
    /// ```text
    /// // assuming `window` is a variable with type `*mut GLFWwindow
    /// // and pos_cb + size_cb are extern "C" functions
    /// // that match the callback signatures.
    /// set_window_callbacks! {
    ///     window,
    ///     (glfwSetWindowPosCallback, pos_cb),
    ///     (glfwSetWindowSizeCallback, size_cb),
    /// }
    ///
    /// ```
    macro_rules! set_window_callbacks {
        ($window: ident, $(($setter: ident, $cb: ident),)*) => {
            $(
                if let Err(e) = el.checked(|| {
                    $setter($window, Some($cb));
                }) {
                    tracing::error!("failed to set window callback {name}: {e:?}", name = stringify!($cb));
                }
            )*
        };
    }
    // implement all the callbacks
    set_window_callbacks!(
        window,
        (glfwSetWindowPosCallback, pos_cb),
        (glfwSetWindowSizeCallback, size_cb),
        (glfwSetWindowCloseCallback, close_cb),
        (glfwSetWindowRefreshCallback, refresh_cb),
        (glfwSetWindowFocusCallback, focus_cb),
        (glfwSetWindowIconifyCallback, iconify_cb),
        (glfwSetWindowMaximizeCallback, maximize_cb),
        (glfwSetFramebufferSizeCallback, fb_size_cb),
        (glfwSetWindowContentScaleCallback, content_scale_cb),
        (glfwSetCursorPosCallback, cursor_pos_cb),
        (glfwSetCursorEnterCallback, cursor_enter_cb),
        (glfwSetScrollCallback, scroll_cb),
        (glfwSetDropCallback, drop_cb),
        (glfwSetKeyCallback, key_cb),
        (glfwSetCharCallback, char_cb),
        (glfwSetMouseButtonCallback, mouse_button_cb),
    );
}
/// pushes [Event::Pos] event to the thread-local event queue
unsafe extern "C" fn pos_cb(window: *mut GLFWwindow, x: i32, y: i32) {
    push_event_to_thread_local(Event::Pos {
        window: WindowId(window),
        x,
        y,
    })
}
/// pushes [Event::Size] event to the thread-local event queue
unsafe extern "C" fn size_cb(window: *mut GLFWwindow, width: i32, height: i32) {
    push_event_to_thread_local(Event::Size {
        window: WindowId(window),
        width,
        height,
    })
}
/// pushes [Event::Close] event to the thread-local event queue
unsafe extern "C" fn close_cb(window: *mut GLFWwindow) {
    push_event_to_thread_local(Event::Close {
        window: WindowId(window),
    });
}
/// pushes [Event::Refresh] event to the thread-local event queue
unsafe extern "C" fn refresh_cb(window: *mut GLFWwindow) {
    push_event_to_thread_local(Event::Refresh {
        window: WindowId(window),
    });
}
/// pushes [Event::Focus] event to the thread-local event queue
unsafe extern "C" fn focus_cb(window: *mut GLFWwindow, focused: i32) {
    push_event_to_thread_local(Event::Focus {
        window: WindowId(window),
        focused: focused == GLFW_TRUE,
    });
}
/// pushes [Event::Iconify] event to the thread-local event queue
unsafe extern "C" fn iconify_cb(window: *mut GLFWwindow, iconified: i32) {
    push_event_to_thread_local(Event::Iconify {
        window: WindowId(window),
        iconified: iconified == GLFW_TRUE,
    });
}
/// pushes [Event::Maximize] event to the thread-local event queue
unsafe extern "C" fn maximize_cb(window: *mut GLFWwindow, maximized: i32) {
    push_event_to_thread_local(Event::Maximize {
        window: WindowId(window),
        maximized: maximized == GLFW_TRUE,
    });
}
/// pushes [Event::FramebufferSize] event to the thread-local event queue
unsafe extern "C" fn fb_size_cb(window: *mut GLFWwindow, width: i32, height: i32) {
    push_event_to_thread_local(Event::FramebufferSize {
        window: WindowId(window),
        width,
        height,
    });
}
/// pushes [Event::ContentScale] event to the thread-local event queue
unsafe extern "C" fn content_scale_cb(window: *mut GLFWwindow, xscale: f32, yscale: f32) {
    push_event_to_thread_local(Event::ContentScale {
        window: WindowId(window),
        xscale,
        yscale,
    });
}
/// pushes [Event::Key] event to the thread-local event queue
unsafe extern "C" fn key_cb(
    window: *mut GLFWwindow,
    key: i32,
    scancode: i32,
    action: i32,
    mods: i32,
) {
    let key = if key == GLFW_KEY_UNKNOWN {
        None
    } else {
        let Ok(key) = key.try_into() else {
            error!("Unknown key: {}", key);
            return;
        };
        Some(key)
    };
    let repeat = action == GLFW_REPEAT;
    let pressed = action == GLFW_PRESS;
    if action != GLFW_REPEAT && action != GLFW_PRESS && action != GLFW_RELEASE {
        error!("Unknown action: {}", action);
        return;
    }
    let Some(mods) = Modifiers::from_bits(mods) else {
        error!("Unknown mods: {}", mods);
        return;
    };
    push_event_to_thread_local(Event::Key {
        window: WindowId(window),
        key,
        scancode,
        pressed,
        repeat,
        mods,
    });
}
/// pushes [Event::Char] event to the thread-local event queue
unsafe extern "C" fn char_cb(window: *mut GLFWwindow, codepoint: u32) {
    push_event_to_thread_local(Event::Char {
        window: WindowId(window),
        codepoint: char::from_u32(codepoint).unwrap(),
    });
}
/// pushes [Event::MouseButton] event to the thread-local event queue
unsafe extern "C" fn mouse_button_cb(window: *mut GLFWwindow, button: i32, action: i32, mods: i32) {
    let Ok(button) = button.try_into() else {
        error!("Unknown button: {}", button);
        return;
    };
    let pressed = action == GLFW_PRESS;
    if action != GLFW_PRESS && action != GLFW_RELEASE {
        error!("Unknown action: {}", action);
        return;
    }
    let Some(mods) = Modifiers::from_bits(mods) else {
        error!("Unknown mods: {}", mods);
        return;
    };
    push_event_to_thread_local(Event::MouseButton {
        window: WindowId(window),
        button,
        pressed,
        mods,
    });
}
/// pushes [Event::CursorPos] event to the thread-local event queue
unsafe extern "C" fn cursor_pos_cb(window: *mut GLFWwindow, x: f64, y: f64) {
    push_event_to_thread_local(Event::CursorPos {
        window: WindowId(window),
        x,
        y,
    });
}
/// pushes [Event::CursorEnter] event to the thread-local event queue
unsafe extern "C" fn cursor_enter_cb(window: *mut GLFWwindow, entered: i32) {
    push_event_to_thread_local(Event::CursorEnter {
        window: WindowId(window),
        entered: entered == GLFW_TRUE,
    });
}
/// pushes [Event::Scroll] event to the thread-local event queue
unsafe extern "C" fn scroll_cb(window: *mut GLFWwindow, xoffset: f64, yoffset: f64) {
    push_event_to_thread_local(Event::Scroll {
        window: WindowId(window),
        x: xoffset,
        y: yoffset,
    });
}
/// pushes [Event::Drop] event to the thread-local event queue
unsafe extern "C" fn drop_cb(
    window: *mut GLFWwindow,
    count: i32,
    paths: *mut *const std::ffi::c_char,
) {
    let paths = std::slice::from_raw_parts(paths, count as usize);
    let paths = paths
        .iter()
        .map(|&s| {
            let cstr = CStr::from_ptr(s);
            match cstr.to_str() {
                Ok(s) => s.to_string(),
                Err(e) => {
                    error!(
                        "path is not utf-8: {}. using lossy conversion: {:?}",
                        e, cstr
                    );
                    cstr.to_string_lossy().into_owned()
                }
            }
        })
        .collect();
    push_event_to_thread_local(Event::Drop {
        window: WindowId(window),
        paths,
    });
}
