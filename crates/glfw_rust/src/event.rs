use crate::*;
/// <https://www.glfw.org/docs/latest/input_guide.html>
///
///
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Event {
    Error(GlfwError),
    /// when a window is moved, whether by the user, the system or your own code
    ///
    /// the new position, in screen coordinates, of the upper-left corner of the content area when the window is moved.
    ///
    /// see [Window::get_pos], [Window::set_pos], [WindowConfig::position_x] and
    /// [WindowConfig::position_y].
    Pos {
        window: WindowId,
        x: i32,
        y: i32,
    },
    /// the new size, in screen coordinates, of the content area of the window when the window is resized.
    ///
    /// If you want the physical size of surface, see [Event::FramebufferSize]. That is what
    /// you should be using for something like `glViewport`.
    ///
    /// see [Window::get_size] and [Window::set_size]
    Size {
        window: WindowId,
        width: i32,
        height: i32,
    },
    /// When the user attempts to close the window, for example by clicking the
    /// close widget or using a key chord like Alt+F4,
    /// the close flag of the window is set and this event is emitted.
    /// The window is however not actually destroyed
    /// and, unless you watch for this state change, nothing further happens.
    ///
    /// See [WindowProxy::set_should_close] and [WindowProxy::should_close]
    Close {
        window: WindowId,
    },
    /// when the contents of a window is damaged and needs to be refreshed
    ///
    /// > On compositing window systems such as Aero, Compiz or Aqua,
    /// > where the window contents are saved off-screen, this callback
    /// > might only be called when the window or framebuffer is resized.
    Refresh {
        window: WindowId,
    },
    /// when a window gains or loses input focus, whether by the user, system or your own code.
    ///
    /// see [Window::focus], [Window::get_focused].
    Focus {
        window: WindowId,
        focused: bool,
    },
    /// when a window is iconified (minimized) or restored,
    /// whether by the user, system or your own code.
    ///
    /// see [Window::iconify], [Window::restore] and [Window::get_iconified].
    Iconify {
        window: WindowId,
        iconified: bool,
    },
    /// when a window is maximized or restored, whether by the user, system or your own code.
    ///
    /// see [Window::maximize], [Window::restore], [Window::get_maximized] and
    /// [WindowConfig::maximized].
    Maximize {
        window: WindowId,
        maximized: bool,
    },
    /// when the framebuffer of a window is resized, whether by the user or the system.
    ///
    /// The width and height are in *pixels* and you can use these for functions like `glViewport` or
    /// vulkan swapchain creation.
    ///
    /// The size of a framebuffer may change independently of the size of a window,
    /// for example if the window is dragged between a regular monitor and a high-DPI one.
    ///
    /// see [Window::get_framebuffer_size]
    FramebufferSize {
        window: WindowId,
        width: i32,
        height: i32,
    },
    /// when the content scale of a window changes, whether because of a system
    /// setting change or because it was moved to a monitor with a different scale
    ///
    /// see [Window::get_content_scale], as well as [WindowConfig::scale_to_monitor] and
    /// [WindowConfig::scale_framebuffer].
    ContentScale {
        window: WindowId,
        xscale: f32,
        yscale: f32,
    },
    /**
    when a physical key is pressed or released or when it repeats.

    Events with pressed (true) and pressed(false)
    are emitted for every key press. Most keys will also emit events
    with repeat (true) while a key is held down.

    Note that many keyboards have a limit on how many keys being
    simultaneous held down that they can detect.
    This limit is called key rollover.

    Key events with repeat (true) are intended for text input.
    They are emitted at the rate set in the user's keyboard settings.
    At most one key is repeated even if several keys are held down.
    repeat should not be relied on to know which
    keys are being held down or to drive animation. Instead you should
    either save the state of relevant keys based on pressed, or call [Window::get_key], which provides
    basic cached key state.

    The key will be one of the existing key tokens, or None if GLFW lacks
    a token for it, for example E-mail and Play keys.

    The scancode is unique for every key, regardless of whether it has
    a key token. Scancodes are platform-specific but consistent over time,
    so keys will have different scancodes depending on the platform but they
    are safe to save to disk. You can query the scancode for any key token
    supported on the current platform with [EventLoopProxy::get_key_scancode].

    ```rust
    # use glfw_rust::*;
    # fn get_scancode(el: &EventLoopProxy) {
        let scancode = el.get_key_scancode(Key::X);
        // set keybindings or something.
    # }
    ```

    The last reported state for every physical key with a key token
    is also saved in per-window state arrays that can be polled with
    [Window::get_key].

    ```rust
    # use glfw_rust::*;
    fn check_status(window: &Window) {
        let pressed = window.get_key(Key::X);
        if pressed {
            println!("key X pressed");
        } else {
            println!("key X released");
        }
    # }
    ```

    This function only returns cached key event state.
    It does not poll the system for the current state of the physical key.
    It also does not provide any key repeat information.

    Whenever you poll state, you risk missing the state change you are looking for.
    If a pressed key is released again before you poll its state, you will
    have missed the key press. The recommended solution for this is to use
    a key callback, but there is also the [Window::set_sticky_keys].

    When sticky keys mode is enabled, the pollable state of a key will
    remain pressed until the state of that key is polled
    with [Window::get_key]. Once it has been polled, if a key release event
    had been processed in the meantime, the state will reset to pressed (false),
    otherwise it will remain pressed (true).

    If you wish to know what the state of the Caps Lock and Num Lock keys was
    when input events were generated, set the [Window::set_lock_key_mods].

    When this input mode is enabled, any callback that receives modifier bits
    will have the [Modifiers::CAPS_LOCK] bit set if Caps Lock was on when
    the event occurred and the [Modifiers::NUM_LOCK] bit set if Num Lock was on.
    */
    Key {
        window: WindowId,
        key: Option<Key>,
        scancode: i32,
        /// `true` if the key was pressed, `false` if the key was released.
        ///
        /// Ignore this value if `repeat` is true.
        pressed: bool,
        /// repeat if the key is being held down. Ignore `pressed` if this is true.
        repeat: bool,
        mods: Modifiers,
    },
    /**
    text input in the form of a stream of Unicode code points,
    as produced by the operating system text input system.
    Unlike key input, text input is affected by keyboard layouts and
    modifier keys and supports composing characters using dead keys.
    Once received, you can encode the code points into UTF-8 or any other encoding you
    prefer.

    For rust, we convert the `u32` sent by glfw into a `char` for convenience.

    For setting/getting clipboard, see [Window::set_clipboard_string] and
    [Window::get_clipboard_string].
    */
    Char {
        window: WindowId,
        codepoint: char,
    },
    /// when a mouse button is pressed or released,
    ///
    /// The last reported state for every supported mouse button is
    /// also saved in per-window state arrays that can be polled with
    /// [Window::get_mouse_button]. Also, see [Window::set_sticky_mouse_buttons].
    MouseButton {
        window: WindowId,
        button: MouseButton,
        pressed: bool,
        mods: Modifiers,
    },
    /// cursor position, measured in screen coordinates but relative to the
    /// top-left corner of the window content area. On platforms that provide it,
    /// the full sub-pixel cursor position is passed on.
    ///
    /// The cursor position is also saved per-window and can be polled with [Window::get_cursor_pos].
    CursorPos {
        window: WindowId,
        x: f64,
        y: f64,
    },
    /// when the cursor enters or leaves the content area of a window
    ///
    /// You can query whether the cursor is currently inside the content area
    /// of the window with the [Window::get_hovered].
    CursorEnter {
        window: WindowId,
        entered: bool,
    },
    ///  when the user scrolls, whether with a mouse wheel or touchpad gesture.
    ///
    /// A normal mouse wheel, being vertical, provides offsets along the Y-axis.
    Scroll {
        window: WindowId,
        x: f64,
        y: f64,
    },
    /// the paths of files and/or directories dropped on a window
    Drop {
        window: WindowId,
        /// The paths of the dropped files and/or directories.
        paths: Vec<String>,
    },
    /// The joystick functions expose connected joysticks and controllers,
    /// with both referred to as joysticks. It supports up to sixteen joysticks ([Joystick])
    ///
    /// Unlike other mouse/keyboard events, joysticks don't produce any events
    /// except for connected and disconnected events.
    ///
    /// If you want the values (like button presses), you need to use the
    /// [EventLoop::get_joystick_buttons] and similar methods on [EventLoop].
    ///
    /// Also see [EventLoop::get_gamepad_state].
    JoystickConnected {
        joystick: Joystick,
        connected: bool,
    },
    /// This is called when a monitor is connected or disconnected.
    ///
    /// Monitor properties are manually requested with
    /// [EventLoop::get_monitor_name] and related methods on [EventLoop].
    MonitorConnected {
        monitor: MonitorId,
        connected: bool,
    },
}
