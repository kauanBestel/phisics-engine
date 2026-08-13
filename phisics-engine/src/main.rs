use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(800, 600, "Black Hole (Rust)", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|s| glfw.get_proc_address_raw(s) as *const _);

    unsafe { gl::ClearColor(0.1, 0.2, 0.3, 1.0); }

    while !window.should_close() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}
