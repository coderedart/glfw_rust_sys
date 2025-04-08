#[forbid(missing_docs)]
mod cursor;
mod event;
mod event_loop;
mod monitor;
mod native;
mod types;
mod version;
mod window;

use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    ffi::{CStr, CString},
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

pub use cursor::*;
pub use event::*;
pub use event_loop::*;
pub use monitor::*;
pub use types::*;
pub use version::*;
pub use window::*;
pub(crate) mod ffi {
    pub use glfw_rust_sys::*;
}
pub type GlfwResult<T> = Result<T, GlfwError>;

pub(crate) fn bool_to_glfw(b: bool) -> i32 {
    if b {
        ffi::GLFW_TRUE
    } else {
        ffi::GLFW_FALSE
    }
}

thread_local! {
    /// This is main-thread local data for the event loop
    ///
    /// This holds the queued events and other state like live monitors.
    ///
    /// Check `is_alive` field to see if the glfw instance is alive or not.
    pub(crate) static MAIN_THREAD_LOCAL_DATA: ThreadLocalEventLoopData = {
        ThreadLocalEventLoopData {
            is_alive: Cell::new(false),
            events: RefCell::new(Vec::new()),
            monitors: RefCell::new(HashSet::new()),
            el: std::rc::Weak::new().into(),
        }
    };
}
/// This is main-thread local data type for the event loop
///
/// It is primarily used by callbacks to collect events and maintain other
/// state.
pub(crate) struct ThreadLocalEventLoopData {
    /// Whether a glfw instance is live (and not yet terminated).
    ///
    /// Is set to true by [EventLoop::init] and set to false by [EventLoop::drop].
    ///
    /// [EventLoop::init] will assert that this is false, because you must not have two live event loops.
    pub is_alive: Cell<bool>,
    /// Queued events from callbacks.
    ///
    /// These events are drained by [EventLoop::poll_events], [EventLoop::wait_events] and
    /// [EventLoop::wait_events_timeout].
    ///
    /// The first value of the tuple is the time of the event (from [EventLoopProxy::get_time]).
    pub events: RefCell<Vec<(f64, Event)>>,
    /// Monitors that are being tracked for liveness.
    ///
    /// When a monitor disconnected event is received, we will remove the monitor from this set.
    ///
    /// When user queries for monitors using [EventLoop::get_monitors] or
    /// [EventLoop::get_primary_monitor], we will add them to this set to track them.
    ///
    /// Any of the monitor related functions will check (for correctness) the liveness
    /// of a monitor using [EventLoop::is_monitor_alive] (which internally checks this set).
    pub monitors: RefCell<HashSet<*mut ffi::GLFWmonitor>>,
    /// This is a weak reference to event loop. We don't really use this for anything.
    /// But on [EventLoop::init], we check if there's still a strong reference to this
    /// data, just to *really* ensure that there's no bugs.
    pub el: Cell<std::rc::Weak<EventLoop>>,
}
impl ThreadLocalEventLoopData {
    /// Push an event to the queue
    /// It will also add the current time to the event.
    pub fn push_event(&self, ev: event::Event) {
        if !self.is_alive.get() {
            tracing::error!("pushing event to a dead glfw event loop");
            return;
        }
        // safe as event loop is alive
        let time = unsafe { ffi::glfwGetTime() };
        self.events.borrow_mut().push((time, ev));
    }
}
/// A convenience function to call from event callbacks.
///
/// Pushes the event to main-thread local data's queued events.
///
/// The event will be picked up by [EventLoop::poll_events] related methods.
pub(crate) fn push_event_to_thread_local(event: event::Event) {
    MAIN_THREAD_LOCAL_DATA.with(|data| {
        data.push_event(event);
    });
}

/// useful to just log any existing errors and clear them.
///
/// We mainly use logging in [Drop::drop] or extern "C" functions, as
/// panicking or returning errors is usually not recommended in those contexts.
///
/// While the [default_error_callback] logs all errors globally by default, this is useful
/// for more targeted logging, as it tracks caller info and logs the location along with the error.
#[track_caller]
pub(crate) fn log_error() {
    if let Err(error) = get_error() {
        tracing::error!(
            "context = {} code = {}, description = {}",
            std::panic::Location::caller(),
            error.code,
            error.description
        );
    }
}
/// Simply calls [get_error] and ignores the result.
///
/// When you make a glfw FFI call and call [get_error], you never know if
/// the error came from this call or a random previous call in the current thread.
///
/// As most of the functions don't check for errors by default, you can have stale
/// errors just resident in the thread. This function will clear that error, so that,
/// you can check for any new errors after making an ffi call.
pub fn clear_error() {
    let _ = get_error();
}
/// Asserts that there is no error.
///
/// simply calls [get_error] and crashes if result is not Ok.
///
/// # Panics
/// If there is an error.
pub fn assert_no_error() {
    get_error().expect("found glfw error");
}
/// <https://www.glfw.org/docs/latest/intro_guide.html#error_handling>
///
/// This function returns the last error that occurred on **this** thread.
///
/// When a glfw function fails, it calls the [EventLoopConfig::error_callback] and
/// sets the error data in a thread-local variable. You can then query that error
/// in the current thread by using this function.
///
/// Remember that the error just stays there forever, until
///
/// * you call this function or
/// * another FFI call replaces it with a new error or
/// * the thread exits.
#[doc(alias = "glfwGetError")]
pub fn get_error() -> GlfwResult<()> {
    let mut description: *const std::ffi::c_char = std::ptr::null();
    let code = unsafe { ffi::glfwGetError(&mut description) };
    if code == ffi::GLFW_NO_ERROR {
        return Ok(());
    }
    Err(GlfwError {
        code: code.into(),
        description: description
            .is_null()
            .then_some(String::new())
            .unwrap_or_else(|| {
                unsafe { std::ffi::CStr::from_ptr(description) }
                    .to_string_lossy()
                    .to_string()
            }),
    })
}
thread_local! {
    /// This is the thread-local data for the current opengl context
    ///
    /// This is usually initialized when someone tries to make a window current.
    ///
    /// And destroyed when the thread exits.
    pub(crate) static LOCAL_GL_CONTEXT: ThreadLocalContext = {
        ThreadLocalContext::new_uncurrent()
    }
}
/// The type to store in thread-local storage to track the current window in this thread.
///
/// The field [Self::is_any_current] is true if any window is current in this thread.
struct ThreadLocalContext {
    /// When a window is made current, its [WindowProxy::data] is stored here.
    ///
    /// The main-reason to store it here, is so that when the window is made uncurrent by
    /// that window or any other window, we can set the [WindowData::is_current] field to false
    ///
    /// That will allow the window to be made current again on any thread.
    ///
    /// We also used this, to check *which* window is current. See [Self::get_current]
    data: RefCell<Arc<WindowData>>,
    /// is true if any window is current
    ///
    /// if false, then no window is current and [Self::data] must not be used/modified.
    is_any_current: Cell<bool>,
}
impl ThreadLocalContext {
    /// Returns true if any context is current on this thread
    pub fn is_any_current(&self) -> bool {
        self.is_any_current.get()
    }
    /// Returns the current context on this thread (none if there is no current context)
    ///
    /// You can use this to check if a certain window is current or not.
    pub fn get_current(&self) -> Option<WindowId> {
        if self.is_any_current() {
            WindowId::new(self.data.borrow().window)
        } else {
            None
        }
    }
    /// returns a new uncurrent default object.
    /// only useful for initializing [`LOCAL_GL_CONTEXT`]
    pub fn new_uncurrent() -> Self {
        ThreadLocalContext {
            data: Arc::new(WindowData {
                window: std::ptr::null_mut(),
                current_thread: Mutex::new(std::thread::current().id()),
                is_current: AtomicBool::new(false),
                is_alive: AtomicBool::new(false),
                client_api: ClientApi::NoAPI,
                context_creation_api: None,
            })
            .into(),
            is_any_current: Cell::new(false),
        }
    }
    /// Make the provided window current.
    ///
    /// * This also makes any *already* current context non-current
    /// * does nothing if the provided window is already current
    ///
    /// # Panics
    /// * if the window is not alive
    /// * if the window is current on a different thread
    pub fn make_current(&self, new_data: Arc<WindowData>) {
        let is_current = self.is_any_current.get();
        // if the context is already current, early return.
        if is_current && Arc::ptr_eq(&new_data, &self.data.borrow()) {
            return;
        }
        // now, we know that data is not current or there's a different current context.
        let mut guard = new_data.current_thread.lock().unwrap();
        // check if the window is still alive
        assert!(new_data.is_alive.load(Ordering::Acquire));
        // if data is already current on a different thread, then this is UB
        assert!(!new_data.is_current.load(Ordering::Acquire));
        unsafe {
            ffi::glfwMakeContextCurrent(new_data.window);
        }
        // now, data is current.
        if is_current {
            // if another context was current before, tell it that it is not current anymore
            self.data
                .borrow()
                .is_current
                .store(false, Ordering::Release);
        }
        // tell our data that it is current
        new_data.is_current.store(true, Ordering::Release);
        // update the current thread
        *guard = std::thread::current().id();
        // now, any other reference knows that this window is current in a thread, so we can drop the guard
        drop(guard);
        // don't forget to set the thread local's data, so it knows who is current for future calls
        self.data.replace(new_data);
        self.is_any_current.set(true);
    }
    /// Make the provided window non-current.
    /// If no window is provided, then any current context on this thread is made non-current.
    ///
    /// * does nothing if no context is current on this thread
    /// * does nothing if a window is provided and it is not current on this thread
    ///
    /// # Panics
    /// * if a window is provided and it is not alive
    pub fn make_uncurrent(&self, which: Option<Arc<WindowData>>) {
        let is_current = self.is_any_current.get();
        // if no context is current, return early.
        if !is_current {
            return;
        }
        // if the particular data is not current, return early.
        if let Some(data) = which {
            // if this particular data is not current, return early
            if !Arc::ptr_eq(&data, &self.data.borrow()) {
                return;
            }
            assert!(
                data.is_alive.load(Ordering::Acquire),
                "Window {:?} is dead, but it was current on a thread.",
                data.window
            );
            // else, this data is current, so, we make it non-current
        }
        // else no data is provided, so any current context must be made non-current
        let data = self.data.borrow();
        let guard = data.current_thread.lock().unwrap();
        unsafe {
            ffi::glfwMakeContextCurrent(std::ptr::null_mut());
        }
        data.is_current.store(false, Ordering::Release);
        drop(guard);
        self.is_any_current.set(false);
    }
}
