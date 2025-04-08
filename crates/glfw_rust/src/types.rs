use crate::ffi::*;
use crate::*;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(i32)]
pub enum StdCursor {
    #[default]
    Arrow = GLFW_ARROW_CURSOR,
    Ibeam = GLFW_IBEAM_CURSOR,
    Crosshair = GLFW_CROSSHAIR_CURSOR,
    PointingHand = GLFW_POINTING_HAND_CURSOR,
    ResizeEW = GLFW_RESIZE_EW_CURSOR,
    ResizeNS = GLFW_RESIZE_NS_CURSOR,
    ResizeNESW = GLFW_RESIZE_NESW_CURSOR,
    ResizeNWSE = GLFW_RESIZE_NWSE_CURSOR,
    ResizeAll = GLFW_RESIZE_ALL_CURSOR,
    NotAllowed = GLFW_NOT_ALLOWED_CURSOR,
}
/**
The Cursor mode provides several cursor modes for special forms
of mouse motion input. By default, the cursor mode is [Normal](CursorMode::Normal),
meaning the regular arrow cursor (or another cursor set with [Window::set_cursor])
is used and cursor motion is not limited.

If you wish to implement mouse motion based camera controls or other input schemes
require unlimited mouse movement, set the cursor mode to [Disabled](CursorMode::Disabled).

```rust
# use glfw_rust::*;
# fn set_cursor_disabled(window: &Window) {
    window.set_cursor_mode(CursorMode::Disabled);
# }
```

This will hide the cursor and lock it to the specified window.
GLFW will then take care of all the details of cursor re-centering and
offset calculation and providing the application with a virtual cursor position.
This virtual position is provided normally via both the cursor position callback
and through polling.

> **Note**: You should not implement your own version of this functionality
> using other features of GLFW. It is not supported and will not work as robustly
> as [Disabled](CursorMode::Disabled).

If you only wish the cursor to become hidden when it is over a window but
still want it to behave normally, set the cursor mode to [Hidden](CursorMode::Hidden).

```rust
# use glfw_rust::*;
# fn set_cursor_hidden(window: &Window) {
    window.set_cursor_mode(CursorMode::Hidden);
# }
```

This mode puts no limit on the motion of the cursor.

If you wish the cursor to be visible but confined to the content area
of the window, set the cursor mode to [Captured](CursorMode::Captured).

```rust
# use glfw_rust::*;
# fn set_cursor_captured(window: &Window) {
    window.set_cursor_mode(CursorMode::Captured);
# }
```

The cursor will behave normally inside the content area but will
not be able to leave unless the window loses focus.

To exit out of either of these special modes, restore the [Normal](CursorMode::Normal) cursor mode.

```rust
# use glfw_rust::*;
# fn set_cursor_normal(window: &Window) {
    window.set_cursor_mode(CursorMode::Normal);
# }
```

If the cursor was disabled, this will move it back to its last visible position.
*/
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(i32)]
pub enum CursorMode {
    /// makes the cursor visible and behaving normally.
    #[default]
    Normal = GLFW_CURSOR_NORMAL,
    /// makes the cursor invisible when it is over the content area of the window but does not restrict the cursor from leaving.
    Hidden = GLFW_CURSOR_HIDDEN,
    /// hides and grabs the cursor, providing virtual and unlimited cursor movement. This is useful for implementing for example 3D camera controls.
    Disabled = GLFW_CURSOR_DISABLED,
    /// makes the cursor visible and confines it to the content area of the window.
    Captured = GLFW_CURSOR_CAPTURED,
}
impl TryFrom<i32> for CursorMode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            GLFW_CURSOR_NORMAL => Ok(CursorMode::Normal),
            GLFW_CURSOR_HIDDEN => Ok(CursorMode::Hidden),
            GLFW_CURSOR_DISABLED => Ok(CursorMode::Disabled),
            GLFW_CURSOR_CAPTURED => Ok(CursorMode::Captured),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(i32)]
pub enum ClientApi {
    #[default]
    OpenGL = GLFW_OPENGL_API,
    OpenGLES = GLFW_OPENGL_ES_API,
    /// for vulkan/metal/d3d12 like APIs which have
    /// their own surface/swapchain creation methods.
    NoAPI = GLFW_NO_API,
}

impl TryFrom<i32> for ClientApi {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            GLFW_OPENGL_API => Ok(ClientApi::OpenGL),
            GLFW_OPENGL_ES_API => Ok(ClientApi::OpenGLES),
            GLFW_NO_API => Ok(ClientApi::NoAPI),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(i32)]
pub enum ContextCreationApi {
    #[default]
    Native = GLFW_NATIVE_CONTEXT_API,
    Egl = GLFW_EGL_CONTEXT_API,
    Osmesa = GLFW_OSMESA_CONTEXT_API,
}
impl TryFrom<i32> for ContextCreationApi {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            GLFW_NATIVE_CONTEXT_API => Ok(ContextCreationApi::Native),
            GLFW_EGL_CONTEXT_API => Ok(ContextCreationApi::Egl),
            GLFW_OSMESA_CONTEXT_API => Ok(ContextCreationApi::Osmesa),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(i32)]
pub enum Robustness {
    #[default]
    No = GLFW_NO_ROBUSTNESS,
    NoResetNotification = GLFW_NO_RESET_NOTIFICATION,
    LoseContextOnReset = GLFW_LOSE_CONTEXT_ON_RESET,
}
impl TryFrom<i32> for Robustness {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            GLFW_NO_ROBUSTNESS => Ok(Robustness::No),
            GLFW_NO_RESET_NOTIFICATION => Ok(Robustness::NoResetNotification),
            GLFW_LOSE_CONTEXT_ON_RESET => Ok(Robustness::LoseContextOnReset),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(i32)]
pub enum ContextReleaseBehavior {
    #[default]
    Any = GLFW_ANY_RELEASE_BEHAVIOR,
    Flush = GLFW_RELEASE_BEHAVIOR_FLUSH,
    None = GLFW_RELEASE_BEHAVIOR_NONE,
}
impl TryFrom<i32> for ContextReleaseBehavior {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            GLFW_ANY_RELEASE_BEHAVIOR => Ok(ContextReleaseBehavior::Any),
            GLFW_RELEASE_BEHAVIOR_FLUSH => Ok(ContextReleaseBehavior::Flush),
            GLFW_RELEASE_BEHAVIOR_NONE => Ok(ContextReleaseBehavior::None),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(i32)]
pub enum OpenGLProfile {
    #[default]
    Any = GLFW_OPENGL_ANY_PROFILE,
    Core = GLFW_OPENGL_CORE_PROFILE,
    Compatibility = GLFW_OPENGL_COMPAT_PROFILE,
}

impl TryFrom<i32> for OpenGLProfile {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            GLFW_OPENGL_ANY_PROFILE => Ok(OpenGLProfile::Any),
            GLFW_OPENGL_CORE_PROFILE => Ok(OpenGLProfile::Core),
            GLFW_OPENGL_COMPAT_PROFILE => Ok(OpenGLProfile::Compatibility),
            _ => Err(()),
        }
    }
}
/// <https://www.glfw.org/docs/latest/intro_guide.html#error_handling>
///
/// Error codes at <https://www.glfw.org/docs/latest/group__errors.html>
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct GlfwError {
    pub code: ErrorCode,
    pub description: String,
}
/// For more verbose documentation for each error code,
/// open <https://www.glfw.org/docs/latest/group__errors.html> and scroll down.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum ErrorCode {
    /// GLFW has not been initialized.
    NotInitialized = GLFW_NOT_INITIALIZED,

    /// No context is current for this thread.
    NoCurrentContext = GLFW_NO_CURRENT_CONTEXT,

    /// One of the arguments to the function was an invalid enum value.
    InvalidEnum = GLFW_INVALID_ENUM,

    /// One of the arguments to the function was an invalid value.
    InvalidValue = GLFW_INVALID_VALUE,

    /// A memory allocation failed.
    OutOfMemory = GLFW_OUT_OF_MEMORY,

    /// GLFW could not find support for the requested API on the system.
    ApiUnavailable = GLFW_API_UNAVAILABLE,

    /// The requested OpenGL or OpenGL ES version is not available.
    VersionUnavailable = GLFW_VERSION_UNAVAILABLE,

    /// A platform-specific error occurred that does not match any of the more specific categories.
    PlatformError = GLFW_PLATFORM_ERROR,

    /// The requested format is not supported or available.
    FormatUnavailable = GLFW_FORMAT_UNAVAILABLE,

    /// The specified window does not have an OpenGL or OpenGL ES context.
    NoWindowContext = GLFW_NO_WINDOW_CONTEXT,

    /// The specified cursor shape is not available.
    CursorUnavailable = GLFW_CURSOR_UNAVAILABLE,

    /// The requested feature is not provided by the platform.
    FeatureUnavailable = GLFW_FEATURE_UNAVAILABLE,

    /// The requested feature is not implemented for the platform.
    FeatureUnimplemented = GLFW_FEATURE_UNIMPLEMENTED,

    /// Platform unavailable or no matching platform was found.
    PlatformUnavailable = GLFW_PLATFORM_UNAVAILABLE,
    Custom(i32),
}
impl From<i32> for ErrorCode {
    fn from(value: i32) -> Self {
        match value {
            GLFW_NOT_INITIALIZED => Self::NotInitialized,
            GLFW_NO_CURRENT_CONTEXT => Self::NoCurrentContext,
            GLFW_INVALID_ENUM => Self::InvalidEnum,
            GLFW_INVALID_VALUE => Self::InvalidValue,
            GLFW_OUT_OF_MEMORY => Self::OutOfMemory,
            GLFW_API_UNAVAILABLE => Self::ApiUnavailable,
            GLFW_VERSION_UNAVAILABLE => Self::VersionUnavailable,
            GLFW_PLATFORM_ERROR => Self::PlatformError,
            GLFW_FORMAT_UNAVAILABLE => Self::FormatUnavailable,
            GLFW_NO_WINDOW_CONTEXT => Self::NoWindowContext,
            GLFW_CURSOR_UNAVAILABLE => Self::CursorUnavailable,
            GLFW_FEATURE_UNAVAILABLE => Self::FeatureUnavailable,
            GLFW_FEATURE_UNIMPLEMENTED => Self::FeatureUnimplemented,
            GLFW_PLATFORM_UNAVAILABLE => Self::PlatformUnavailable,
            _ => Self::Custom(value),
        }
    }
}
impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl GlfwError {
    pub fn dead_monitor(monitor: MonitorId, context: &str) -> Self {
        Self {
            code: ErrorCode::PlatformError,
            description: format!("At {context}, {monitor:?} monitor is no longer alive"),
        }
    }
    pub fn dead_context(context: &str) -> Self {
        Self {
            code: ErrorCode::NotInitialized,
            description: format!("At {context}, glfw is dead"),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Platform {
    Win32 = GLFW_PLATFORM_WIN32,
    Cocoa = GLFW_PLATFORM_COCOA,
    Wayland = GLFW_PLATFORM_WAYLAND,
    X11 = GLFW_PLATFORM_X11,
    /// A simulated platform
    Null = GLFW_PLATFORM_NULL,
}
impl TryFrom<i32> for Platform {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            GLFW_PLATFORM_WIN32 => Ok(Platform::Win32),
            GLFW_PLATFORM_COCOA => Ok(Platform::Cocoa),
            GLFW_PLATFORM_WAYLAND => Ok(Platform::Wayland),
            GLFW_PLATFORM_X11 => Ok(Platform::X11),
            GLFW_PLATFORM_NULL => Ok(Platform::Null),
            _ => Err(()),
        }
    }
}
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum AnglePlatform {
    None = GLFW_ANGLE_PLATFORM_TYPE_NONE,
    OpenGL = GLFW_ANGLE_PLATFORM_TYPE_OPENGL,
    OpenGLES = GLFW_ANGLE_PLATFORM_TYPE_OPENGLES,
    D3D9 = GLFW_ANGLE_PLATFORM_TYPE_D3D9,
    D3D11 = GLFW_ANGLE_PLATFORM_TYPE_D3D11,
    Vulkan = GLFW_ANGLE_PLATFORM_TYPE_VULKAN,
    Metal = GLFW_ANGLE_PLATFORM_TYPE_METAL,
}
impl TryFrom<i32> for AnglePlatform {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            GLFW_ANGLE_PLATFORM_TYPE_NONE => Ok(AnglePlatform::None),
            GLFW_ANGLE_PLATFORM_TYPE_OPENGL => Ok(AnglePlatform::OpenGL),
            GLFW_ANGLE_PLATFORM_TYPE_OPENGLES => Ok(AnglePlatform::OpenGLES),
            GLFW_ANGLE_PLATFORM_TYPE_D3D9 => Ok(AnglePlatform::D3D9),
            GLFW_ANGLE_PLATFORM_TYPE_D3D11 => Ok(AnglePlatform::D3D11),
            GLFW_ANGLE_PLATFORM_TYPE_VULKAN => Ok(AnglePlatform::Vulkan),
            GLFW_ANGLE_PLATFORM_TYPE_METAL => Ok(AnglePlatform::Metal),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum MouseButton {
    Left = GLFW_MOUSE_BUTTON_LEFT,
    Right = GLFW_MOUSE_BUTTON_RIGHT,
    Middle = GLFW_MOUSE_BUTTON_MIDDLE,
    Button4 = GLFW_MOUSE_BUTTON_4,
    Button5 = GLFW_MOUSE_BUTTON_5,
    Button6 = GLFW_MOUSE_BUTTON_6,
    Button7 = GLFW_MOUSE_BUTTON_7,
    Button8 = GLFW_MOUSE_BUTTON_8,
}
impl TryFrom<i32> for MouseButton {
    type Error = ();
    fn try_from(id: i32) -> Result<MouseButton, ()> {
        match id {
            GLFW_MOUSE_BUTTON_LEFT => Ok(MouseButton::Left),
            GLFW_MOUSE_BUTTON_RIGHT => Ok(MouseButton::Right),
            GLFW_MOUSE_BUTTON_MIDDLE => Ok(MouseButton::Middle),
            GLFW_MOUSE_BUTTON_4 => Ok(MouseButton::Button4),
            GLFW_MOUSE_BUTTON_5 => Ok(MouseButton::Button5),
            GLFW_MOUSE_BUTTON_6 => Ok(MouseButton::Button6),
            GLFW_MOUSE_BUTTON_7 => Ok(MouseButton::Button7),
            GLFW_MOUSE_BUTTON_8 => Ok(MouseButton::Button8),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum Joystick {
    Joystick1 = GLFW_JOYSTICK_1,
    Joystick2 = GLFW_JOYSTICK_2,
    Joystick3 = GLFW_JOYSTICK_3,
    Joystick4 = GLFW_JOYSTICK_4,
    Joystick5 = GLFW_JOYSTICK_5,
    Joystick6 = GLFW_JOYSTICK_6,
    Joystick7 = GLFW_JOYSTICK_7,
    Joystick8 = GLFW_JOYSTICK_8,
    Joystick9 = GLFW_JOYSTICK_9,
    Joystick10 = GLFW_JOYSTICK_10,
    Joystick11 = GLFW_JOYSTICK_11,
    Joystick12 = GLFW_JOYSTICK_12,
    Joystick13 = GLFW_JOYSTICK_13,
    Joystick14 = GLFW_JOYSTICK_14,
    Joystick15 = GLFW_JOYSTICK_15,
    Joystick16 = GLFW_JOYSTICK_16,
}
impl TryFrom<i32> for Joystick {
    type Error = ();
    fn try_from(id: i32) -> Result<Joystick, ()> {
        match id {
            GLFW_JOYSTICK_1 => Ok(Joystick::Joystick1),
            GLFW_JOYSTICK_2 => Ok(Joystick::Joystick2),
            GLFW_JOYSTICK_3 => Ok(Joystick::Joystick3),
            GLFW_JOYSTICK_4 => Ok(Joystick::Joystick4),
            GLFW_JOYSTICK_5 => Ok(Joystick::Joystick5),
            GLFW_JOYSTICK_6 => Ok(Joystick::Joystick6),
            GLFW_JOYSTICK_7 => Ok(Joystick::Joystick7),
            GLFW_JOYSTICK_8 => Ok(Joystick::Joystick8),
            GLFW_JOYSTICK_9 => Ok(Joystick::Joystick9),
            GLFW_JOYSTICK_10 => Ok(Joystick::Joystick10),
            GLFW_JOYSTICK_11 => Ok(Joystick::Joystick11),
            GLFW_JOYSTICK_12 => Ok(Joystick::Joystick12),
            GLFW_JOYSTICK_13 => Ok(Joystick::Joystick13),
            GLFW_JOYSTICK_14 => Ok(Joystick::Joystick14),
            GLFW_JOYSTICK_15 => Ok(Joystick::Joystick15),
            GLFW_JOYSTICK_16 => Ok(Joystick::Joystick16),
            _ => Err(()),
        }
    }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
    pub struct Modifiers: i32 {
        const SHIFT = GLFW_MOD_SHIFT;
        const CONTROL = GLFW_MOD_CONTROL;
        const ALT = GLFW_MOD_ALT;
        const SUPER = GLFW_MOD_SUPER;
        const CAPS_LOCK = GLFW_MOD_CAPS_LOCK;
        const NUM_LOCK = GLFW_MOD_NUM_LOCK;
    }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
    pub struct JoystickHatState: u8 {
        const UP = GLFW_HAT_UP as u8;
        const RIGHT = GLFW_HAT_RIGHT as u8;
        const DOWN = GLFW_HAT_DOWN as u8;
        const LEFT = GLFW_HAT_LEFT as u8;
        const RIGHT_UP = GLFW_HAT_RIGHT_UP as u8;
        const RIGHT_DOWN = GLFW_HAT_RIGHT_DOWN as u8;
        const LEFT_UP = GLFW_HAT_LEFT_UP as u8;
        const LEFT_DOWN = GLFW_HAT_LEFT_DOWN as u8;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GamepadState {
    pub buttons: [bool; 15],
    pub axes: [f32; 6],
}
// #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
// #[repr(i32)]
// pub enum Action {
//     Press = GLFW_PRESS,
//     Release = GLFW_RELEASE,
// }

// impl TryFrom<i32> for Action {
//     type Error = ();
//     fn try_from(action: i32) -> Result<Action, ()> {
//         match action {
//             GLFW_PRESS => Ok(Action::Press),
//             GLFW_RELEASE => Ok(Action::Release),
//             _ => Err(()),
//         }
//     }
// }

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum Key {
    Space = GLFW_KEY_SPACE,
    Apostrophe = GLFW_KEY_APOSTROPHE,
    Comma = GLFW_KEY_COMMA,
    Minus = GLFW_KEY_MINUS,
    Period = GLFW_KEY_PERIOD,
    Slash = GLFW_KEY_SLASH,
    Num0 = GLFW_KEY_0,
    Num1 = GLFW_KEY_1,
    Num2 = GLFW_KEY_2,
    Num3 = GLFW_KEY_3,
    Num4 = GLFW_KEY_4,
    Num5 = GLFW_KEY_5,
    Num6 = GLFW_KEY_6,
    Num7 = GLFW_KEY_7,
    Num8 = GLFW_KEY_8,
    Num9 = GLFW_KEY_9,
    Semicolon = GLFW_KEY_SEMICOLON,
    Equal = GLFW_KEY_EQUAL,
    A = GLFW_KEY_A,
    B = GLFW_KEY_B,
    C = GLFW_KEY_C,
    D = GLFW_KEY_D,
    E = GLFW_KEY_E,
    F = GLFW_KEY_F,
    G = GLFW_KEY_G,
    H = GLFW_KEY_H,
    I = GLFW_KEY_I,
    J = GLFW_KEY_J,
    K = GLFW_KEY_K,
    L = GLFW_KEY_L,
    M = GLFW_KEY_M,
    N = GLFW_KEY_N,
    O = GLFW_KEY_O,
    P = GLFW_KEY_P,
    Q = GLFW_KEY_Q,
    R = GLFW_KEY_R,
    S = GLFW_KEY_S,
    T = GLFW_KEY_T,
    U = GLFW_KEY_U,
    V = GLFW_KEY_V,
    W = GLFW_KEY_W,
    X = GLFW_KEY_X,
    Y = GLFW_KEY_Y,
    Z = GLFW_KEY_Z,
    LeftBracket = GLFW_KEY_LEFT_BRACKET,
    Backslash = GLFW_KEY_BACKSLASH,
    RightBracket = GLFW_KEY_RIGHT_BRACKET,
    GraveAccent = GLFW_KEY_GRAVE_ACCENT,
    World1 = GLFW_KEY_WORLD_1,
    World2 = GLFW_KEY_WORLD_2,
    Escape = GLFW_KEY_ESCAPE,
    Enter = GLFW_KEY_ENTER,
    Tab = GLFW_KEY_TAB,
    Backspace = GLFW_KEY_BACKSPACE,
    Insert = GLFW_KEY_INSERT,
    Delete = GLFW_KEY_DELETE,
    Right = GLFW_KEY_RIGHT,
    Left = GLFW_KEY_LEFT,
    Down = GLFW_KEY_DOWN,
    Up = GLFW_KEY_UP,
    PageUp = GLFW_KEY_PAGE_UP,
    PageDown = GLFW_KEY_PAGE_DOWN,
    Home = GLFW_KEY_HOME,
    End = GLFW_KEY_END,
    CapsLock = GLFW_KEY_CAPS_LOCK,
    ScrollLock = GLFW_KEY_SCROLL_LOCK,
    NumLock = GLFW_KEY_NUM_LOCK,
    PrintScreen = GLFW_KEY_PRINT_SCREEN,
    Pause = GLFW_KEY_PAUSE,
    F1 = GLFW_KEY_F1,
    F2 = GLFW_KEY_F2,
    F3 = GLFW_KEY_F3,
    F4 = GLFW_KEY_F4,
    F5 = GLFW_KEY_F5,
    F6 = GLFW_KEY_F6,
    F7 = GLFW_KEY_F7,
    F8 = GLFW_KEY_F8,
    F9 = GLFW_KEY_F9,
    F10 = GLFW_KEY_F10,
    F11 = GLFW_KEY_F11,
    F12 = GLFW_KEY_F12,
    F13 = GLFW_KEY_F13,
    F14 = GLFW_KEY_F14,
    F15 = GLFW_KEY_F15,
    F16 = GLFW_KEY_F16,
    F17 = GLFW_KEY_F17,
    F18 = GLFW_KEY_F18,
    F19 = GLFW_KEY_F19,
    F20 = GLFW_KEY_F20,
    F21 = GLFW_KEY_F21,
    F22 = GLFW_KEY_F22,
    F23 = GLFW_KEY_F23,
    F24 = GLFW_KEY_F24,
    F25 = GLFW_KEY_F25,
    Kp0 = GLFW_KEY_KP_0,
    Kp1 = GLFW_KEY_KP_1,
    Kp2 = GLFW_KEY_KP_2,
    Kp3 = GLFW_KEY_KP_3,
    Kp4 = GLFW_KEY_KP_4,
    Kp5 = GLFW_KEY_KP_5,
    Kp6 = GLFW_KEY_KP_6,
    Kp7 = GLFW_KEY_KP_7,
    Kp8 = GLFW_KEY_KP_8,
    Kp9 = GLFW_KEY_KP_9,
    KpDecimal = GLFW_KEY_KP_DECIMAL,
    KpDivide = GLFW_KEY_KP_DIVIDE,
    KpMultiply = GLFW_KEY_KP_MULTIPLY,
    KpSubtract = GLFW_KEY_KP_SUBTRACT,
    KpAdd = GLFW_KEY_KP_ADD,
    KpEnter = GLFW_KEY_KP_ENTER,
    KpEqual = GLFW_KEY_KP_EQUAL,
    LeftShift = GLFW_KEY_LEFT_SHIFT,
    LeftControl = GLFW_KEY_LEFT_CONTROL,
    LeftAlt = GLFW_KEY_LEFT_ALT,
    LeftSuper = GLFW_KEY_LEFT_SUPER,
    RightShift = GLFW_KEY_RIGHT_SHIFT,
    RightControl = GLFW_KEY_RIGHT_CONTROL,
    RightAlt = GLFW_KEY_RIGHT_ALT,
    RightSuper = GLFW_KEY_RIGHT_SUPER,
    Menu = GLFW_KEY_MENU,
}
impl TryFrom<i32> for Key {
    type Error = ();
    fn try_from(raw: i32) -> Result<Key, ()> {
        match raw {
            GLFW_KEY_SPACE => Ok(Key::Space),
            GLFW_KEY_APOSTROPHE => Ok(Key::Apostrophe),
            GLFW_KEY_COMMA => Ok(Key::Comma),
            GLFW_KEY_MINUS => Ok(Key::Minus),
            GLFW_KEY_PERIOD => Ok(Key::Period),
            GLFW_KEY_SLASH => Ok(Key::Slash),
            GLFW_KEY_0 => Ok(Key::Num0),
            GLFW_KEY_1 => Ok(Key::Num1),
            GLFW_KEY_2 => Ok(Key::Num2),
            GLFW_KEY_3 => Ok(Key::Num3),
            GLFW_KEY_4 => Ok(Key::Num4),
            GLFW_KEY_5 => Ok(Key::Num5),
            GLFW_KEY_6 => Ok(Key::Num6),
            GLFW_KEY_7 => Ok(Key::Num7),
            GLFW_KEY_8 => Ok(Key::Num8),
            GLFW_KEY_9 => Ok(Key::Num9),
            GLFW_KEY_SEMICOLON => Ok(Key::Semicolon),
            GLFW_KEY_EQUAL => Ok(Key::Equal),
            GLFW_KEY_A => Ok(Key::A),
            GLFW_KEY_B => Ok(Key::B),
            GLFW_KEY_C => Ok(Key::C),
            GLFW_KEY_D => Ok(Key::D),
            GLFW_KEY_E => Ok(Key::E),
            GLFW_KEY_F => Ok(Key::F),
            GLFW_KEY_G => Ok(Key::G),
            GLFW_KEY_H => Ok(Key::H),
            GLFW_KEY_I => Ok(Key::I),
            GLFW_KEY_J => Ok(Key::J),
            GLFW_KEY_K => Ok(Key::K),
            GLFW_KEY_L => Ok(Key::L),
            GLFW_KEY_M => Ok(Key::M),
            GLFW_KEY_N => Ok(Key::N),
            GLFW_KEY_O => Ok(Key::O),
            GLFW_KEY_P => Ok(Key::P),
            GLFW_KEY_Q => Ok(Key::Q),
            GLFW_KEY_R => Ok(Key::R),
            GLFW_KEY_S => Ok(Key::S),
            GLFW_KEY_T => Ok(Key::T),
            GLFW_KEY_U => Ok(Key::U),
            GLFW_KEY_V => Ok(Key::V),
            GLFW_KEY_W => Ok(Key::W),
            GLFW_KEY_X => Ok(Key::X),
            GLFW_KEY_Y => Ok(Key::Y),
            GLFW_KEY_Z => Ok(Key::Z),
            GLFW_KEY_LEFT_BRACKET => Ok(Key::LeftBracket),
            GLFW_KEY_BACKSLASH => Ok(Key::Backslash),
            GLFW_KEY_RIGHT_BRACKET => Ok(Key::RightBracket),
            GLFW_KEY_GRAVE_ACCENT => Ok(Key::GraveAccent),
            GLFW_KEY_WORLD_1 => Ok(Key::World1),
            GLFW_KEY_WORLD_2 => Ok(Key::World2),
            GLFW_KEY_ESCAPE => Ok(Key::Escape),
            GLFW_KEY_ENTER => Ok(Key::Enter),
            GLFW_KEY_TAB => Ok(Key::Tab),
            GLFW_KEY_BACKSPACE => Ok(Key::Backspace),
            GLFW_KEY_INSERT => Ok(Key::Insert),
            GLFW_KEY_DELETE => Ok(Key::Delete),
            GLFW_KEY_RIGHT => Ok(Key::Right),
            GLFW_KEY_LEFT => Ok(Key::Left),
            GLFW_KEY_DOWN => Ok(Key::Down),
            GLFW_KEY_UP => Ok(Key::Up),
            GLFW_KEY_PAGE_UP => Ok(Key::PageUp),
            GLFW_KEY_PAGE_DOWN => Ok(Key::PageDown),
            GLFW_KEY_HOME => Ok(Key::Home),
            GLFW_KEY_END => Ok(Key::End),
            GLFW_KEY_CAPS_LOCK => Ok(Key::CapsLock),
            GLFW_KEY_SCROLL_LOCK => Ok(Key::ScrollLock),
            GLFW_KEY_NUM_LOCK => Ok(Key::NumLock),
            GLFW_KEY_PRINT_SCREEN => Ok(Key::PrintScreen),
            GLFW_KEY_PAUSE => Ok(Key::Pause),
            GLFW_KEY_F1 => Ok(Key::F1),
            GLFW_KEY_F2 => Ok(Key::F2),
            GLFW_KEY_F3 => Ok(Key::F3),
            GLFW_KEY_F4 => Ok(Key::F4),
            GLFW_KEY_F5 => Ok(Key::F5),
            GLFW_KEY_F6 => Ok(Key::F6),
            GLFW_KEY_F7 => Ok(Key::F7),
            GLFW_KEY_F8 => Ok(Key::F8),
            GLFW_KEY_F9 => Ok(Key::F9),
            GLFW_KEY_F10 => Ok(Key::F10),
            GLFW_KEY_F11 => Ok(Key::F11),
            GLFW_KEY_F12 => Ok(Key::F12),
            GLFW_KEY_F13 => Ok(Key::F13),
            GLFW_KEY_F14 => Ok(Key::F14),
            GLFW_KEY_F15 => Ok(Key::F15),
            GLFW_KEY_F16 => Ok(Key::F16),
            GLFW_KEY_F17 => Ok(Key::F17),
            GLFW_KEY_F18 => Ok(Key::F18),
            GLFW_KEY_F19 => Ok(Key::F19),
            GLFW_KEY_F20 => Ok(Key::F20),
            GLFW_KEY_F21 => Ok(Key::F21),
            GLFW_KEY_F22 => Ok(Key::F22),
            GLFW_KEY_F23 => Ok(Key::F23),
            GLFW_KEY_F24 => Ok(Key::F24),
            GLFW_KEY_F25 => Ok(Key::F25),
            GLFW_KEY_KP_0 => Ok(Key::Kp0),
            GLFW_KEY_KP_1 => Ok(Key::Kp1),
            GLFW_KEY_KP_2 => Ok(Key::Kp2),
            GLFW_KEY_KP_3 => Ok(Key::Kp3),
            GLFW_KEY_KP_4 => Ok(Key::Kp4),
            GLFW_KEY_KP_5 => Ok(Key::Kp5),
            GLFW_KEY_KP_6 => Ok(Key::Kp6),
            GLFW_KEY_KP_7 => Ok(Key::Kp7),
            GLFW_KEY_KP_8 => Ok(Key::Kp8),
            GLFW_KEY_KP_9 => Ok(Key::Kp9),
            GLFW_KEY_KP_DECIMAL => Ok(Key::KpDecimal),
            GLFW_KEY_KP_DIVIDE => Ok(Key::KpDivide),
            GLFW_KEY_KP_MULTIPLY => Ok(Key::KpMultiply),
            GLFW_KEY_KP_SUBTRACT => Ok(Key::KpSubtract),
            GLFW_KEY_KP_ADD => Ok(Key::KpAdd),
            GLFW_KEY_KP_ENTER => Ok(Key::KpEnter),
            GLFW_KEY_KP_EQUAL => Ok(Key::KpEqual),
            GLFW_KEY_LEFT_SHIFT => Ok(Key::LeftShift),
            GLFW_KEY_LEFT_CONTROL => Ok(Key::LeftControl),
            GLFW_KEY_LEFT_ALT => Ok(Key::LeftAlt),
            GLFW_KEY_LEFT_SUPER => Ok(Key::LeftSuper),
            GLFW_KEY_RIGHT_SHIFT => Ok(Key::RightShift),
            GLFW_KEY_RIGHT_CONTROL => Ok(Key::RightControl),
            GLFW_KEY_RIGHT_ALT => Ok(Key::RightAlt),
            GLFW_KEY_RIGHT_SUPER => Ok(Key::RightSuper),
            GLFW_KEY_MENU => Ok(Key::Menu),

            _ => Err(()),
        }
    }
}

mod test {
    #[test]
    fn test_last_enums() {
        assert_eq!(glfw_rust_sys::GLFW_KEY_LAST, super::Key::Menu as _);
        assert_eq!(
            glfw_rust_sys::GLFW_MOUSE_BUTTON_LAST,
            super::MouseButton::Button8 as _
        );
        assert_eq!(
            glfw_rust_sys::GLFW_JOYSTICK_LAST,
            super::Joystick::Joystick16 as _
        );
    }
}
