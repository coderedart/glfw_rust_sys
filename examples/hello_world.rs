use glfw_rust::*;
use glow::HasContext;

fn main() {
    // to watch for any glfw errors
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env()
                .unwrap(),
        )
        .init();
    // create event loop
    let el = EventLoop::init(Default::default()).unwrap();
    let window = Window::new(
        el.clone(),
        Default::default(),
        800,
        600,
        "Hello World",
        None,
        None,
    )
    .unwrap();
    window.make_current();
    let ctx = unsafe { glow::Context::from_loader_function(|s| window.get_proc_addr(s)) };
    unsafe { ctx.clear_color(0.95, 0.32, 0.11, 1.0) };
    // To print fps every second
    let mut fps_counter = 0;
    let mut fps_reset = std::time::Instant::now();
    let mut average_fps = 0;
    while !window.should_close() {
        unsafe {
            ctx.clear(glow::COLOR_BUFFER_BIT);
        }
        let events = el.poll_events();
        for (_, _event) in events {
            dbg!(_event);
        }
        fps_counter += 1;
        if fps_reset.elapsed().as_secs() >= 1 {
            dbg!(average_fps);
            average_fps = fps_counter;
            fps_counter = 0;
            fps_reset = std::time::Instant::now();
        }
        window.swap_buffers();
    }
    // drop will automatically do this, but might as well follow good practice
    window.make_uncurrent();
}
