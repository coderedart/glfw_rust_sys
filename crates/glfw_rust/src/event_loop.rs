use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};

use tracing::error;

use crate::event::Event;
use crate::ffi::*;
use crate::*;
use tinyvec::TinyVec;
/// Configuration for [EventLoop]'s initialization.
/// These affect how glfw behaves until [EventLoop] is dropped.
///
/// Some of these are platform specific and will only have an
/// effect on the relevant platforms. eg: [Self::cocoa_menubar] is only
/// relevant on macos and will be ignored on other platforms.
///
/// Read more about initialization hints at <https://www.glfw.org/docs/latest/intro_guide.html#init_hints>
///
/// For default values, look at the table at <https://www.glfw.org/docs/latest/intro_guide.html#init_hints_values>
#[derive(Debug, Default)]
pub struct EventLoopConfig {
    /// The platform that glfw will use.
    ///
    /// By default, glfw uses `GLFW_ANY_PLATFORM` and you can use this
    /// to force glfw to use a specific platform.
    ///
    /// You can read more at <https://www.glfw.org/docs/latest/intro_guide.html#platform>
    ///
    /// <https://www.glfw.org/docs/latest/intro_guide.html#init_hints_shared>
    ///
    /// For now, this is only relevant in two situations:
    /// 1. Testing: use [Platform::Null] for testing.
    /// 2. Linux: Force use of [Platform::X11] to prefer Xwayland over native wayland windows.
    pub platform: Option<Platform>,
    /// If set, this will be used as the error callback.
    /// Otherwise, the [default_error_callback] will be used. [default_error_callback]
    /// will just log the errors using [tracing::error] macro.
    ///
    /// <https://www.glfw.org/docs/latest/intro_guide.html#error_handling>
    pub error_callback: GLFWerrorfun,

    /// specifies whether to also expose joystick hats as buttons, for compatibility
    /// with earlier versions of GLFW that did not have [EventLoop::get_joystick_hats].
    ///
    /// <https://www.glfw.org/docs/latest/intro_guide.html#GLFW_JOYSTICK_HAT_BUTTONS>
    pub joystick_hat_buttons: Option<bool>,
    /// I have no idea about the rest of the options.
    /// So, I added the links, so you can quickly go read the official docs.
    ///
    /// <https://www.glfw.org/docs/latest/intro_guide.html#init_hints_shared>
    pub angle_platform: Option<AnglePlatform>,
    /// <https://www.glfw.org/docs/latest/intro_guide.html#init_hints_osx>
    pub cocoa_chdir_resources: Option<bool>,
    /// <https://www.glfw.org/docs/latest/intro_guide.html#init_hints_osx>
    pub cocoa_menubar: Option<bool>,
    /// <https://www.glfw.org/docs/latest/intro_guide.html#init_hints_wayland>
    pub wayland_libdecor: Option<bool>,
    /// <https://www.glfw.org/docs/latest/intro_guide.html#init_hints_x11>
    pub x11_xcb_vk_surface: Option<bool>,
}
impl EventLoopConfig {
    /// This sets the window hints and logs any errors before returning the error.
    #[doc(alias = "glfwInitHint")]
    unsafe fn set_hints(self) -> GlfwResult<()> {
        let EventLoopConfig {
            platform,
            error_callback,
            joystick_hat_buttons,
            angle_platform,
            cocoa_chdir_resources,
            cocoa_menubar,
            wayland_libdecor,
            x11_xcb_vk_surface,
        } = self;
        clear_error();
        glfwSetErrorCallback(Some(error_callback.unwrap_or(default_error_callback)));
        get_error().inspect_err(|e| error!("setting error callback failed: {e:?}"))?;
        if let Some(platform) = platform {
            glfwInitHint(GLFW_PLATFORM, platform as _);
            get_error().inspect_err(|e| error!("setting platform init hint failed: {e:?}"))?;
        };
        if let Some(angle_platform) = angle_platform {
            glfwInitHint(GLFW_ANGLE_PLATFORM_TYPE, angle_platform as _);
            get_error()
                .inspect_err(|e| error!("setting angle platform init hint failed: {e:?}"))?;
        }
        if let Some(joystick_hat_buttons) = joystick_hat_buttons {
            glfwInitHint(
                GLFW_JOYSTICK_HAT_BUTTONS,
                bool_to_glfw(joystick_hat_buttons),
            );
            get_error().inspect_err(|e| error!("joystick hat buttons hint failed: {e:?}"))?;
        }
        if let Some(cocoa_chdir) = cocoa_chdir_resources {
            glfwInitHint(GLFW_COCOA_CHDIR_RESOURCES, bool_to_glfw(cocoa_chdir));
            get_error().inspect_err(|e| error!("cocoa chdir hint failed: {e:?}"))?;
        }
        if let Some(cocoa_menubar) = cocoa_menubar {
            glfwInitHint(GLFW_COCOA_MENUBAR, bool_to_glfw(cocoa_menubar));
            get_error().inspect_err(|e| error!("cocoa menubar hint failed: {e:?}"))?;
        }
        if let Some(wayland_libdecor) = wayland_libdecor {
            glfwInitHint(
                GLFW_WAYLAND_LIBDECOR,
                if wayland_libdecor {
                    GLFW_WAYLAND_PREFER_LIBDECOR
                } else {
                    GLFW_WAYLAND_DISABLE_LIBDECOR
                },
            );
            get_error().inspect_err(|e| error!("wayland libdecor hint failed: {e:?}"))?;
        }
        if let Some(x11_xcb_vk_surface) = x11_xcb_vk_surface {
            glfwInitHint(
                GLFW_X11_XCB_VULKAN_SURFACE,
                bool_to_glfw(x11_xcb_vk_surface),
            );
            get_error().inspect_err(|e| error!("x11 xcb vk surface hint failed: {e:?}"))?;
        }
        Ok(())
    }
}
/// This is the default error callback, if user doesn't pass an explicit one in [EventLoopConfig].
///
/// This simply logs the errors with `tracing::error`.
///
/// # Safety
/// The `description` parameter must be null or null-terminated. It should also be valid utf-8.
pub unsafe extern "C" fn default_error_callback(code: i32, description: *const std::ffi::c_char) {
    let description = if description.is_null() {
        ""
    } else {
        std::ffi::CStr::from_ptr(description)
            .to_str()
            .expect("glfw error callback received invalid utf-8")
    };
    error!("code = {}; desc = {}", code, description);
}

/// This represents the entry point of this crate. It must be created, used and destroyed on main-thread.
///
/// All glfw methods that must be called on main-thread are implemented on this struct.
///
/// You just need to create an event loop and start polling events with [EventLoop::poll_events].
///
/// You can create a window anytime you want and the events for that window will start
/// appearing in the event loop.
///
///
/// ```rust
/// # use glfw_rust::*;
/// let el = EventLoop::init(EventLoopConfig::default()).unwrap();
///
/// let window = Window::new(el.clone(),Default::default(),800,600,"Hello World",None,None).unwrap();
/// window.make_current();
/// while window.should_close() {
///     for (event_timestmp, event) in el.wait_events() {
///         // handle events
///     }
///     
///     // do some rendering
///     window.swap_buffers();
/// # break;
/// }
/// ```
///
/// For glfw methods that can be called from *any* (including main) threads, see [EventLoopProxy].
/// [EventLoop] derefs to [EventLoopProxy] for convenience.
///
/// If you want a [EventLoopProxy] to send to other threads, use [EventLoop::new_proxy] method.
///
/// After [EventLoop] is dropped, using any [EventLoopProxy] will panic.
///
///
#[derive(Debug)]
pub struct EventLoop {
    /// The thread on which this struct was initialized.
    init_thread_id: std::thread::ThreadId,
    /// Proxy object that can be used for thread-safe glfw methods.
    proxy: EventLoopProxy,
    /// top stop this from being moved to a different thread.
    _no_sync: std::marker::PhantomData<*const ()>,
}
impl Drop for EventLoop {
    fn drop(&mut self) {
        self.proxy.data.store(false, Ordering::Release);
        // reset thread local data, so that next initialization of glfw can succeed.
        MAIN_THREAD_LOCAL_DATA.with(|data| {
            data.is_alive.set(false);
            data.events.take();
            data.monitors.take();
        });
        // if Arc::weak_count(&self.proxy.data) > 0 {
        //     error!("EventLoop is being dropped with more than one EventloopProxy still being alive. This is a bug.");
        // }
        // pretty much impossible as this is !Send, but just a sanity check.
        if std::thread::current().id() != self.init_thread_id {
            error!("EventLoop is being dropped from a different thread than it was initialized on. This is a bug.");
        }
        
        unsafe {
            clear_error();
            glfwTerminate();
            log_error();
        };
    }
}
impl Deref for EventLoop {
    type Target = EventLoopProxy;
    fn deref(&self) -> &Self::Target {
        &self.proxy
    }
}
impl EventLoop {
    /// <https://www.glfw.org/docs/latest/intro_guide.html#intro_init_init>
    ///
    /// This initializes glfw and starts listening to monitor/joystick events.
    ///
    /// You can call [Self::poll_events] (or related methods) to get all the
    /// queued events.
    ///
    /// If you want to configure any initilazation hints, use [EventLoopConfig].
    ///
    /// If we fail to set any of the hints or fail to init glfw, we will return error.
    ///
    /// You will only ever need a single [EventLoop] instance. You create one at the
    /// beginning of your app, and drop it at the end.
    ///
    /// # Panics
    /// If there's an already existing [EventLoop] instance that was not dropped.
    #[doc(alias = "glfwInit")]
    pub fn init(config: EventLoopConfig) -> GlfwResult<Rc<Self>> {
        MAIN_THREAD_LOCAL_DATA.with(|main_glfw| {
            assert!(
                !main_glfw.is_alive.get(),
                "EventLoop is already initialized AND alive.
                Please drop it first, before initializing again"
            );
        });

        unsafe {
            config
                .set_hints()
                .inspect_err(|e| error!("setting hints failed: {e:?}"))?;
            // previous functions errors should have been cleared.
            assert_no_error();
            if glfwInit() != GLFW_TRUE {
                return match get_error() {
                    Ok(_) => Err(GlfwError::dead_context("glfw init failed with NO errors")),
                    Err(error) => Err(error),
                };
            }
            let data = Arc::new(AtomicBool::new(true));
            let el = Rc::new(Self {
                init_thread_id: std::thread::current().id(),
                proxy: EventLoopProxy { data },
                _no_sync: std::marker::PhantomData,
            });
            MAIN_THREAD_LOCAL_DATA.with(|main_glfw| {
                assert!(
                    !main_glfw.is_alive.get(),
                    "glfw is reinitialized without terminating the previous one"
                );
                main_glfw.is_alive.set(true);
                main_glfw.events.take();
                main_glfw.monitors.take();
                // just to *really* make sure
                let old_el = main_glfw.el.replace(Rc::downgrade(&el));
                if old_el.upgrade().is_some() {
                    error!("old EventLoop is still alive. this could be a bug");
                }
            });
            {
                glfwSetMonitorCallback(Some(monitor_callback));
                get_error().inspect_err(|error| {
                    error!("failed to set monitor callback: {error:?}");
                })?;
                glfwSetJoystickCallback(Some(joystick_callback));
                get_error().inspect_err(|error| {
                    error!("failed to set joystick callback: {error:?}");
                })?;
            }
            Ok(el)
        }
    }
    /// Returns a new [EventLoopProxy] that can be used to call glfw methods from any thread.
    ///
    /// You can also use this to wakeup glfw on main-thread if it is waiting for events.
    ///
    /// You may close as many proxies as you need.
    pub fn new_proxy(&self) -> EventLoopProxy {
        self.proxy.clone()
    }
    /// This function returns whether the library was compiled with support for the specified platform.
    ///
    /// <https://www.glfw.org/docs/latest/intro_guide.html#platform>
    #[doc(alias = "glfwPlatformSupported")]
    pub fn is_platform_supported(platform: Platform) -> bool {
        unsafe { glfwPlatformSupported(platform as _) == GLFW_TRUE }
    }
    /// 1. calls [clear_error].
    /// 2. calls the closure.
    /// 3. calls [get_error]
    ///
    /// If there's error, returns error.
    ///
    /// If no error, returns the return value of closure.
    ///
    /// As most of the functions don't check for errors by default, you can
    /// use this function to simplify checking for errors.
    ///
    /// For example:
    ///
    /// ```rust
    /// use glfw_rust::*;
    /// fn getting_window_pos_on_wayland(win: &Window, el: &EventLoop) -> GlfwResult<(i32, i32)> {
    ///     let (x, y) = win.get_pos(); // returns (0, 0), silently ignoring error.
    ///     // This will return GlfwResult<(i32, i32)>, because on wayland
    ///     // getting position of a window triggers [ErrorCode::FeatureUnavailable]
    ///     el.checked(|| win.get_pos())
    /// }
    /// ```
    pub fn checked<T>(&self, f: impl FnOnce() -> T) -> GlfwResult<T> {
        clear_error();
        let result = f();
        get_error().and(Ok(result))
    }
    /// 1. calls [clear_error].
    /// 2. calls the closure.
    /// 3. calls [get_error].
    ///
    /// If there's error, logs error with caller location.
    ///
    /// returns the return value of closure.
    #[track_caller]
    pub fn logged<T>(&self, f: impl FnOnce() -> T) -> T {
        clear_error();
        let result = f();
        if let Err(error) = get_error() {
            tracing::error!(
                "context = {} code = {}, description = {}",
                std::panic::Location::caller(),
                error.code,
                error.description
            );
        }
        result
    }
    /// 1. calls closure
    /// 2. calls [clear_error] to clean any error status in the current thread.
    ///
    /// returns the value of closure
    pub fn ignored<T>(&self, f: impl FnOnce() -> T) -> T {
        let result = f();
        clear_error();
        result
    }
    /// While some event callbacks are "synchronous" (like window resize), others need to be explicitly
    /// queried.
    ///
    /// This function queries any pending events and returns them.
    /// The first value of the tuple is the time of event (from [EventLoopProxy::get_time]).
    /// The second value is the actual event data.
    ///
    /// This function must be regularly called or the OS will assume that
    /// your app is unresponsive and may forcibly kill it.
    ///
    /// If you are a game-like app, you probably want to pump as many frames as you want, so, you will
    /// use this function to get the events, react to them, draw to screen and present the surface (eg: [WindowProxy::swap_buffers]).
    ///
    /// But if you are a gui-app that only needs to draw in response to events, you are
    /// better off using [Self::wait_events] instead.
    pub fn poll_events(&self) -> Vec<(f64, Event)> {
        unsafe { glfwPollEvents() };
        MAIN_THREAD_LOCAL_DATA.with(|main_glfw| main_glfw.events.take())
    }
    /// This function puts the calling thread to sleep until at least one event is available in the event queue.
    ///
    /// Once one or more events are available, it behaves exactly like [Self::poll_events], i.e.
    /// the events in the queue are processed and the function then returns immediately.
    ///
    /// This is really useful for gui-apps that just want to wait for events and do nothing (i.e. save cpu/battery).
    ///
    /// If you have an off-thread doing some work, you can use
    /// [EventLoopProxy::post_empty_event] to wake up the main thread and force this function to return
    /// even if there wasn't a gui event.
    ///
    /// If you would like to timeout the wait, use [Self::wait_events_timeout].
    pub fn wait_events(&self) -> Vec<(f64, Event)> {
        unsafe { glfwWaitEvents() };
        MAIN_THREAD_LOCAL_DATA.with(|main_glfw| main_glfw.events.take())
    }
    /// This function puts the calling thread to sleep until at least one event is available in the event queue, or until the specified timeout is reached.
    /// If one or more events are available, it behaves exactly like [Self::poll_events], i.e.
    /// the events in the queue are processed and the function then returns immediately
    ///
    /// This is useful when you want to wait for events,
    /// but also want to wake up to do some work after some time. The best example would be
    /// animations, which run at regular intervals.
    ///
    /// Just like [Self::wait_events], you can use [EventLoopProxy::post_empty_event] to
    /// make this function return earlier.
    ///
    ///
    pub fn wait_events_timeout(&self, timeout: f64) -> Vec<(f64, Event)> {
        unsafe { glfwWaitEventsTimeout(timeout) };
        MAIN_THREAD_LOCAL_DATA.with(|main_glfw| main_glfw.events.take())
    }
    /// This function returns whether raw mouse motion is supported on the current
    /// system. This status does not change after GLFW has been initialized
    /// so you only need to check this once. If you attempt to enable raw motion
    /// on a system that does not support it, [PlatformError](ErrorCode::PlatformError)
    /// will be emitted.
    ///
    /// Raw mouse motion is closer to the actual motion of the mouse across a surface.
    /// It is not affected by the scaling and acceleration applied to the motion
    /// of the desktop cursor. That processing is suitable for a cursor while raw motion
    /// is better for controlling for example a 3D camera. Because of this, raw mouse
    /// motion is only provided when the cursor is disabled.
    #[doc(alias = "glfwRawMouseMotionSupported")]
    pub fn is_raw_mouse_motion_supported(&self) -> bool {
        unsafe { glfwRawMouseMotionSupported() == GLFW_TRUE }
    }
    /**
    This function returns the name of the specified printable key,
    encoded as UTF-8. This is typically the character that key would
    produce without any modifier keys, intended for displaying key
    bindings to the user. For dead keys, it is typically the diacritic
    it would add to a character.

    Do not use this function for [text input](Event::Char).
    You will break text input for many languages even if it happens to
    work for yours.

    If the key is None, the scancode is used to identify the key,
    otherwise the scancode is ignored. If you specify a non-printable key,
    or None and a scancode that maps to a non-printable key,
    this function returns None but does not emit an error.

    This behavior allows you to always pass in the arguments in
    the [key event](Event::Key) without modification.

    The printable keys are:

    * GLFW_KEY_APOSTROPHE
    * GLFW_KEY_COMMA
    * GLFW_KEY_MINUS
    * GLFW_KEY_PERIOD
    * GLFW_KEY_SLASH
    * GLFW_KEY_SEMICOLON
    * GLFW_KEY_EQUAL
    * GLFW_KEY_LEFT_BRACKET
    * GLFW_KEY_RIGHT_BRACKET
    * GLFW_KEY_BACKSLASH
    * GLFW_KEY_WORLD_1
    * GLFW_KEY_WORLD_2
    * GLFW_KEY_0 to GLFW_KEY_9
    * GLFW_KEY_A to GLFW_KEY_Z
    * GLFW_KEY_KP_0 to GLFW_KEY_KP_9
    * GLFW_KEY_KP_DECIMAL
    * GLFW_KEY_KP_DIVIDE
    * GLFW_KEY_KP_MULTIPLY
    * GLFW_KEY_KP_SUBTRACT
    * GLFW_KEY_KP_ADD
    * GLFW_KEY_KP_EQUAL

    Names for printable keys depend on keyboard layout, while names
    for non-printable keys are the same across layouts but depend
    on the application language and should be localized along with other
    user interface text.
    */
    pub fn get_key_name(&self, key: Option<Key>, scancode: i32) -> Option<&str> {
        unsafe {
            let name = glfwGetKeyName(key.map_or(GLFW_KEY_UNKNOWN, |k| k as _), scancode);
            if name.is_null() {
                return None;
            }
            CStr::from_ptr(name)
        }
        .to_str()
        .ok()
    }
    /// This function returns whether the specified joystick is present.
    ///
    /// There is no need to call this function before other functions that accept a joystick ID, as they all check for presence before performing any other work.
    #[doc(alias = "glfwJoystickPresent")]
    pub fn is_joystick_present(&self, joystick: Joystick) -> bool {
        unsafe { glfwJoystickPresent(joystick as _) == GLFW_TRUE }
    }
    /// This function returns the values of all axes of the specified joystick.
    /// Each element in the array is a value between -1.0 and 1.0.
    ///
    /// If the specified joystick is not present this function will return None but will not generate an error.
    /// This can be used instead of first calling [Self::is_joystick_present].
    #[doc(alias = "glfwGetJoystickAxes")]
    pub fn get_joystick_axes(&self, joystick: Joystick) -> Option<TinyVec<[f32; 8]>> {
        let mut count = 0;
        let axes = unsafe { glfwGetJoystickAxes(joystick as _, &mut count) };
        if axes.is_null() {
            return None;
        }
        Some(unsafe { std::slice::from_raw_parts(axes, count as _) }.into())
    }
    /**
    This function returns the state of all buttons of the specified joystick.
    if true, button is pressed.

    For backward compatibility with earlier versions that did not have [Self::get_joystick_hats],
    the button array also includes all hats, each represented as four buttons.
    The hats are in the same order as returned by [Self::get_joystick_hats] and
    the buttons are in the order up, right, down and left. To disable these
    extra buttons, set the [EventLoopConfig::joystick_hat_buttons] init hint before initialization.

    If the specified joystick is not present this function will return NULL but will not generate an error. This can be used instead of first calling @ref glfwJoystickPresent.
    */
    #[doc(alias = "glfwGetJoystickButtons")]
    pub fn get_joystick_buttons(&self, joystick: Joystick) -> Option<TinyVec<[bool; 15]>> {
        let mut count = 0;
        let buttons = unsafe { glfwGetJoystickButtons(joystick as _, &mut count) };
        if buttons.is_null() {
            return None;
        }
        Some(TinyVec::from_iter(
            unsafe { std::slice::from_raw_parts(buttons, count as _) }
                .into_iter()
                .map(|c| *c as i32 == GLFW_TRUE),
        ))
    }
    /**
    This function returns the state of all hats of the specified joystick.
    Each element in the array is one of the following values:

    | Name               | Value                       |
    | ------------------ | --------------------------- |
    | GLFW_HAT_CENTERED  | 0                           |
    | GLFW_HAT_UP        | 1                           |
    | GLFW_HAT_RIGHT     | 2                           |
    | GLFW_HAT_DOWN      | 4                           |
    | GLFW_HAT_LEFT      | 8                           |
    | GLFW_HAT_RIGHT_UP  | GLFW_HAT_RIGHT | GLFW_HAT_UP  |
    | GLFW_HAT_RIGHT_DOWN| GLFW_HAT_RIGHT | GLFW_HAT_DOWN|
    | GLFW_HAT_LEFT_UP   | GLFW_HAT_LEFT  | GLFW_HAT_UP  |
    | GLFW_HAT_LEFT_DOWN | GLFW_HAT_LEFT  | GLFW_HAT_DOWN|

    The diagonal directions are bitwise combinations of the
    primary (up, right, down and left) directions and you can test for these
    individually by ANDing it with the corresponding direction.

    In rust, we use bitflags to represent this state. The centered is the empty/default flags.

    see [JoystickHatState] for more details

    If the specified joystick is not present this function will return None
    but will not generate an error. This can be used instead of first calling
    [Self::is_joystick_present].
    */
    #[doc(alias = "glfwGetJoystickHats")]
    pub fn get_joystick_hats(&self, joystick: Joystick) -> Option<TinyVec<[JoystickHatState; 4]>> {
        let mut count = 0;
        let hats = unsafe { glfwGetJoystickHats(joystick as _, &mut count) };
        if hats.is_null() {
            return None;
        }
        Some(TinyVec::from_iter(
            unsafe { std::slice::from_raw_parts(hats, count as _) }
                .into_iter()
                .map(|h| JoystickHatState::from_bits(*h).unwrap()),
        ))
    }
    /**
    This function returns the name, encoded as UTF-8, of the specified joystick.

    If the specified joystick is not present this function will return None but will not
    generate an error. This can be used instead of first calling [Self::is_joystick_present].
    */
    #[doc(alias = "glfwGetJoystickName")]
    pub fn get_joystick_name(&self, joystick: Joystick) -> Option<String> {
        unsafe {
            let p = glfwGetJoystickName(joystick as _);
            if p.is_null() {
                return None;
            }
            Some(CStr::from_ptr(p).to_string_lossy().to_string())
        }
    }
    /**
    This function returns the SDL compatible GUID, as a UTF-8 encoded
    hexadecimal string, of the specified joystick.

    The GUID is what connects a joystick to a gamepad mapping. A connected
    joystick will always have a GUID even if there is no gamepad mapping
    assigned to it.

    If the specified joystick is not present this function will return None but will
    not generate an error. This can be used instead of first calling [Self::is_joystick_present].

    The GUID uses the format introduced in SDL 2.0.5. This GUID tries to uniquely
    identify the make and model of a joystick but does not identify a specific unit,
    e.g. all wired Xbox 360 controllers will have the same GUID on that platform.
    The GUID for a unit may vary between platforms depending on what hardware information
    the platform specific APIs provide.
    */
    #[doc(alias = "glfwGetJoystickGUID")]
    pub fn get_joystick_guid(&self, joystick: Joystick) -> Option<String> {
        unsafe {
            let p = glfwGetJoystickGUID(joystick as _);
            if p.is_null() {
                return None;
            }
            Some(CStr::from_ptr(p).to_string_lossy().to_string())
        }
    }
    /**
    This function returns whether the specified joystick is both present
    and has a gamepad mapping.

    If the specified joystick is present but does not have a gamepad mapping
    this function will return false but will not generate an error.
    Call [Self::is_joystick_present] to check if a joystick is present
    regardless of whether it has a mapping.
    */
    #[doc(alias = "glfwJoystickIsGamepad")]
    pub fn joystick_is_gamepad(&self, joystick: Joystick) -> bool {
        unsafe { glfwJoystickIsGamepad(joystick as _) == GLFW_TRUE }
    }
    /**
    This function parses the specified ASCII encoded string and updates the
    internal list with any gamepad mappings it finds. This string may contain
    either a single gamepad mapping or many mappings separated by newlines. The
    parser supports the full format of the gamecontrollerdb.txt source file
    including empty lines and comments.

    See <https://www.glfw.org/docs/latest/input_guide.html#gamepad_mapping>
    for a description of the format.

    If there is already a gamepad mapping for a given GUID in the internal list,
    it will be replaced by the one passed to this function. If the library
    is terminated and re-initialized the internal list will revert to the
    built-in default.
    */
    #[doc(alias = "glfwUpdateGamepadMappings")]
    pub fn update_gamepad_mappings(&self, mappings: &CStr) -> GlfwResult<()> {
        clear_error();
        let result = self.checked(|| unsafe { glfwUpdateGamepadMappings(mappings.as_ptr()) })?;
        assert!(result == GLFW_TRUE);
        Ok(())
    }
    /**
    This function returns the human-readable name of the gamepad
    from the gamepad mapping assigned to the specified joystick.

    If the specified joystick is not present or does not have a gamepad mapping
    this function will return None but will not generate an error.
    Call [Self::is_joystick_present] to check whether it is present
    regardless of whether it has a mapping.
    */
    #[doc(alias = "glfwGetGamepadName")]
    pub fn get_gamepad_name(&self, joystick: Joystick) -> Option<String> {
        unsafe {
            let p = glfwGetGamepadName(joystick as _);
            if p.is_null() {
                return None;
            }
            Some(CStr::from_ptr(p).to_string_lossy().to_string())
        }
    }
    /**
    This function retrieves the state of the specified joystick
    remapped to an Xbox-like gamepad.

    If the specified joystick is not present or does not have a gamepad
    mapping this function will return None but will not generate an error.
    Call [Self::is_joystick_present] to check whether it is present
    regardless of whether it has a mapping.

    The Guide button may not be available for input as it is often hooked by the
    system or the Steam client.

    Not all devices have all the buttons or axes provided by
    [GamepadState]. Unavailable buttons and axes will always report false
    and 0.0 respectively.
    */
    #[doc(alias = "glfwGetGamepadState")]
    pub fn get_gamepad_state(&self, joystick: Joystick) -> Option<GamepadState> {
        let mut state = GLFWgamepadstate {
            buttons: [0; 15],
            axes: [0.0; 6],
        };
        unsafe {
            if glfwGetGamepadState(joystick as _, &mut state) != GLFW_TRUE {
                return None;
            }
        }
        Some(GamepadState {
            buttons: state.buttons.map(|b| b as i32 == GLFW_TRUE),
            axes: state.axes.map(|a| a),
        })
    }
}
/// This is called when a joystick is connected or disconnected.
///
/// It will also log errors, if the values are out of range.
///
/// It will simply forward the event to [push_event_to_thread_local].
unsafe extern "C" fn joystick_callback(id: i32, event: i32) {
    let Ok(joystick) = id.try_into() else {
        error!("Unknown joystick: {}", id);
        return;
    };
    let connected = match event {
        GLFW_CONNECTED => true,
        GLFW_DISCONNECTED => false,
        _ => {
            error!("Unknown joystick {joystick:?} event: {}", event);
            return;
        }
    };
    push_event_to_thread_local(Event::JoystickConnected {
        joystick,
        connected,
    });
}
/// This is called when a monitor is connected or disconnected.
///
/// It will also add/remove the monitor from the live-set of monitors
/// tracked by thread-local [EventLoop]'s data, which is used by
/// [EventLoop::is_monitor_alive].
///
/// It will also log errors, if the values are out of range.
///
/// It will simply forward the event to [push_event_to_thread_local].
unsafe extern "C" fn monitor_callback(id: *mut GLFWmonitor, event: i32) {
    let Some(monitor) = MonitorId::new(id) else {
        error!("NULL monitor: {:?}", id);
        return;
    };
    let connected = match event {
        GLFW_CONNECTED => true,
        GLFW_DISCONNECTED => false,
        _ => {
            error!("Unknown monitor {monitor:?} event: {}", event);
            return;
        }
    };
    MAIN_THREAD_LOCAL_DATA.with(|main_glfw| {
        if connected {
            main_glfw.monitors.borrow_mut().insert(id);
        } else {
            main_glfw.monitors.borrow_mut().remove(&id);
        }
    });

    push_event_to_thread_local(Event::MonitorConnected { monitor, connected });
}
/// This struct implements all the glfw methods that can be called on any thread (including main).
///
/// All methods of this struct will panic if the [EventLoop] is dropped. Which is a very rare case,
/// but a panic is better than a segfault.
#[derive(Debug, Clone)]
pub struct EventLoopProxy {
    data: std::sync::Arc<AtomicBool>,
}
impl EventLoopProxy {
    /// If the [EventLoop] that this proxy belongs to is still alive (not dropped/terminated).
    ///
    /// All methods of [EventLoopProxy] will assert this and panic if it returns false.
    pub fn is_alive(&self) -> bool {
        self.data.load(Ordering::Acquire)
    }
    /// It just asserts that [Self::is_alive] is true and then, calls the closure.
    pub fn with_proxy_alive<T>(&self, work: impl FnOnce() -> T) -> T {
        assert!(
            self.is_alive(),
            "trying to use EventLoopProxy after main EventLoop has been dropped"
        );
        work()
    }
    pub fn with_alive_checked<T>(&self, work: impl FnOnce() -> T) -> GlfwResult<T> {
        clear_error();
        let result = self.with_proxy_alive(work);
        get_error().and(Ok(result))
    }

    /// This function returns the platform that was selected during initialization.
    ///
    /// <https://www.glfw.org/docs/latest/intro_guide.html#platform>
    #[doc(alias = "glfwGetPlatform")]
    pub fn get_platform(&self) -> Platform {
        self.with_proxy_alive(|| match unsafe { glfwGetPlatform() } {
            GLFW_PLATFORM_NULL => Platform::Null,
            GLFW_PLATFORM_WIN32 => Platform::Win32,
            GLFW_PLATFORM_COCOA => Platform::Cocoa,
            GLFW_PLATFORM_X11 => Platform::X11,
            GLFW_PLATFORM_WAYLAND => Platform::Wayland,
            _ => unreachable!("glfwGetPlatform out of bounds platlform enum"),
        })
    }
    /// <https://www.glfw.org/docs/latest/input_guide.html#time>
    ///
    /// There's no synchronization with [Self::set_time].
    /// So, there might be a data race if you [Self::set_time] at the same moment a another thread is getting time.
    #[doc(alias = "glfwGetTime")]
    pub fn get_time(&self) -> f64 {
        self.with_proxy_alive(|| unsafe { glfwGetTime() })
    }
    /// <https://www.glfw.org/docs/latest/input_guide.html#time>
    ///
    /// There's no synchronization with [Self::get_time].
    /// So, there might be a data race if you [Self::get_time] at the same moment a another thread is setting time.
    #[doc(alias = "glfwSetTime")]
    pub fn set_time(&self, time: f64) {
        self.with_proxy_alive(|| unsafe { glfwSetTime(time) });
    }
    /// <https://www.glfw.org/docs/latest/input_guide.html#events>
    ///
    /// wakes up main-thread if it is sleeping while waiting for events
    #[doc(alias = "glfwPostEmptyEvent")]
    pub fn post_empty_event(&self) {
        self.with_proxy_alive(|| unsafe { glfwPostEmptyEvent() })
    }
    /// <https://www.glfw.org/docs/latest/context_guide.html#context_current>
    ///
    /// This calles `glfwMakeContextCurrent` will null pointer, and detaches the current context (if any).
    ///
    ///
    #[doc(alias = "glfwMakeContextCurrent")]
    pub fn make_any_uncurrent(&self) {
        self.with_proxy_alive(|| LOCAL_GL_CONTEXT.with(|ctx| ctx.make_uncurrent(None)))
    }
    #[doc(alias = "glfwGetCurrentContext")]
    pub fn get_any_current(&self) -> Option<WindowId> {
        self.with_proxy_alive(|| LOCAL_GL_CONTEXT.with(|ctx| ctx.get_current()))
            .inspect(|w| {
                assert!(w.get_ptr() == unsafe { glfwGetCurrentContext() });
            })
    }
    /// This function returns whether the Vulkan loader and any minimally functional
    /// ICD have been found.
    ///
    /// The availability of a Vulkan loader and even an ICD does not by
    /// itself guarantee that surface creation or even instance creation is possible.
    /// Call @ref glfwGetRequiredInstanceExtensions to check whether the extensions
    /// necessary for Vulkan surface creation are available and @ref glfwGetPhysicalDevicePresentationSupport
    /// to check whether a queue family of a physical device supports image presentation.
    #[doc(alias = "glfwVulkanSupported")]
    pub fn is_vulkan_supported(&self) -> bool {
        self.with_proxy_alive(|| unsafe { glfwVulkanSupported() == GLFW_TRUE })
    }
    /// This function returns an array of names of Vulkan instance extensions
    /// required by GLFW for creating Vulkan surfaces for GLFW windows.
    /// If successful, the list will always contain VK_KHR_surface, so if
    /// you don't require any additional extensions you can pass this list directly to
    /// the VkInstanceCreateInfo struct.
    ///
    /// If Vulkan is not available on the machine, this function returns
    /// empty Vec and generates a [ErrorCode::ApiUnavailable] error.
    /// Call [Self::is_vulkan_supported] to check whether
    /// Vulkan is at least minimally available.
    ///
    /// If Vulkan is available but no set of extensions allowing window surface
    /// creation was found, this function returns empty Vec.
    /// You may still use Vulkan for off-screen rendering and compute work.
    ///
    /// We return a `Vec<CString>` because it is easier to just
    /// pass it directly to vulkan related APIs.
    #[doc(alias = "glfwGetRequiredInstanceExtensions")]
    pub fn get_required_instance_extensions(&self) -> Vec<CString> {
        assert!(self.is_vulkan_supported());
        let mut count = 0;
        let extensions =
            self.with_proxy_alive(|| unsafe { glfwGetRequiredInstanceExtensions(&mut count) });
        if count == 0 || extensions.is_null() {
            return Vec::new();
        }
        unsafe { std::slice::from_raw_parts(extensions, count as _) }
            .into_iter()
            .map(|s| unsafe {
                assert!(!s.is_null());
                CStr::from_ptr(*s).to_owned()
            })
            .collect()
    }
    /// This function returns the address of the specified Vulkan
    /// instance core or extension function for the specified instance.
    /// If instance is set to NULL it can return any function exported from
    /// the Vulkan loader, including at least the following functions:
    ///
    /// - vkEnumerateInstanceExtensionProperties
    /// - vkEnumerateInstanceLayerProperties
    /// - vkCreateInstance
    /// - vkGetInstanceProcAddr
    ///
    /// If Vulkan is not available on the machine, this function returns
    /// NULL and generates a [ErrorCode::ApiUnavailable]. Call [Self::is_vulkan_supported]
    /// to check whether Vulkan is at least minimally available.
    ///
    /// This function is equivalent to calling vkGetInstanceProcAddr with
    /// a platform-specific query of the Vulkan loader as a fallback.
    #[doc(alias = "glfwGetInstanceProcAddress")]
    pub unsafe fn get_instance_proc_addr_cstr(
        instance: VkInstance,
        proc_name: &CStr,
    ) -> *mut std::ffi::c_void {
        match unsafe { glfwGetInstanceProcAddress(instance, proc_name.as_ptr()) } {
            Some(p) => p as _,
            None => std::ptr::null_mut(),
        }
    }
    /// This function returns whether the specified queue family of the specified
    /// physical device supports presentation to the platform GLFW was built for.
    ///
    /// If Vulkan or the required window surface creation instance extensions are
    /// not available on the machine, or if the specified instance was not created
    /// with the required extensions, this function returns false and generates
    /// a [ErrorCode::ApiUnavailable] error. Call [Self::is_vulkan_supported] to
    /// check whether Vulkan is at least minimally available and
    /// [Self::get_required_instance_extensions] to check what instance extensions
    /// are required.
    #[doc(alias = "glfwGetPhysicalDevicePresentationSupport")]
    pub unsafe fn get_physical_device_presentation_support(
        &self,
        instance: VkInstance,
        device: VkPhysicalDevice,
        queue_family: u32,
    ) -> bool {
        self.with_proxy_alive(|| unsafe {
            glfwGetPhysicalDevicePresentationSupport(instance, device, queue_family) == GLFW_TRUE
        })
    }
    /// This function returns the platform-specific scancode of the specified key.
    ///
    /// If the specified key corresponds to a physical key not supported on
    /// the current platform then this method will return None.
    #[doc(alias = "glfwGetKeyScancode")]
    pub fn get_key_scancode(&self, key: Key) -> Option<i32> {
        let code = unsafe { glfwGetKeyScancode(key as _) };
        (code != -1).then_some(code)
    }
}
