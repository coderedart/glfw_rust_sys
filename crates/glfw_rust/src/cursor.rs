use super::ffi::*;
use super::*;

/// A custom cursor to use for your window.
/// 
/// You can use one of the provided [StdCursor]s or create one from pixels
/// using [Cursor::new_from_pixels].
/// 
/// Once you have the cursor, you just need to set it with [Window::set_cursor].
#[derive(Debug)]
pub struct Cursor {
    /// glfwTerminate destroys cursors, so lets keep it alive.
    _el: Rc<EventLoop>,
    ptr: *mut GLFWcursor,
}
impl Cursor {
    /// Creates a new custom cursor image that can be set for a window
    /// with [Window::set_cursor].
    ///
    /// The pixels are 32-bit, little-endian, non-premultiplied RGBA,
    /// i.e. eight bits per channel with the red channel first.
    /// They are arranged canonically as packed sequential rows, starting from
    /// the top-left corner.
    ///
    /// The cursor hotspot is specified in pixels,
    /// relative to the upper-left corner of the cursor image.
    /// Like all other coordinate systems in GLFW, the X-axis points to
    /// the right and the Y-axis points down.
    pub fn new_from_pixels(
        el: Rc<EventLoop>,
        width: u32,
        height: u32,
        pixels: &[u8],
        x_hot: i32,
        y_hot: i32,
    ) -> Option<Self> {
        assert!(width as usize * height as usize * 4 == pixels.len());
        let image = GLFWimage {
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
            pixels: pixels.as_ptr().cast_mut(),
        };
        let cursor = unsafe { glfwCreateCursor(&image, x_hot, y_hot) };
        if cursor.is_null() {
            None
        } else {
            Some(Cursor {
                ptr: cursor,
                _el: el,
            })
        }
    }
    /**
    Returns a cursor with a standard shape, that can be set for a window
    with [Window::set_cursor]. The images for these cursors come from
    the system cursor theme and their exact appearance will vary between platforms.

    Most of these shapes are guaranteed to exist on every supported platform
    but a few may not be present. See the table below for details.

    |Cursor shape	           | Windows |  macOS  |  X11  |  Wayland  |
    | -----------------------  | ------- | ------- | ----- | --------- |
    | [StdCursor::Arrow]       | Yes     | Yes     | Yes   | Yes       |
    | [StdCursor::Ibeam]       | Yes     | Yes     | Yes   | Yes       |
    | [StdCursor::Crosshair]   | Yes     | Yes     | Yes   | Yes       |
    | [StdCursor::PointingHand]| Yes     | Yes     | Yes   | Yes       |
    | [StdCursor::ResizeEW]    | Yes     | Yes     | Yes   | Yes       |
    | [StdCursor::ResizeNS]    | Yes     | Yes     | Yes   | Yes       |
    | [StdCursor::ResizeNWSE]  | Yes     | Yes1    | Maybe2| Maybe2    |
    | [StdCursor::ResizeNESW]  | Yes     | Yes1    | Maybe2| Maybe2    |
    | [StdCursor::ResizeAll]   | Yes     | Yes     | Yes   | Yes       |
    | [StdCursor::NotAllowed]  | Yes     | Yes     | Maybe2| Maybe2    |

    1. This uses a private system API and may fail in the future.
    2. This uses a newer standard that not all cursor themes support.

    If the requested shape is not available, this function emits a
    [ErrorCode::CursorUnavailable] error and returns None.
    */
    pub fn new_std_cursor(el: Rc<EventLoop>, cursor: StdCursor) -> Option<Self> {
        let cursor = unsafe { glfwCreateStandardCursor(cursor as _) };
        if cursor.is_null() {
            None
        } else {
            Some(Cursor {
                ptr: cursor,
                _el: el,
            })
        }
    }
    /// Just provides the inner pointer. 
    pub fn get_ptr(&self) -> *mut GLFWcursor {
        self.ptr
    }
}
impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe {
            glfwDestroyCursor(self.ptr);
        }
    }
}
