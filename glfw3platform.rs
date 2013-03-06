// modules
extern mod glfw;
mod callbacks;

// pub exports
pub type PlatformType = GlfwData;

// priv uses
use engine::Platform;
use engine::VideoDriver;

// priv data structures
struct GlfwData;

impl Platform for GlfwData {
    #[inline(always)]
    static fn init<D: VideoDriver>() -> GlfwData {
        setup::<D>();
        GlfwData
    }

    fn foo(&self) { }
}

fn setup<D: VideoDriver>() {
    glfw::set_error_callback(callbacks::error_callback);

    do glfw::spawn {

        glfw::window_hint(glfw::RESIZABLE, glfw::TRUE);

        let window = glfw::Window::create(
            800, 600, "ydEngine", glfw::Windowed).unwrap();

        window.set_input_mode(glfw::STICKY_KEYS, 1);

        // Register event callbacks

        callbacks::setup_window(&window);

        window.make_context_current();

        let driver: D = VideoDriver::init();

        let mut done = false;

        while (!done) {
            glfw::poll_events();

            // Check if the window should close
            done = window.should_close();
        }

        window.destroy();
    }
}
