use glfw::Context; //semelhante a uma interface

fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS) //conversa com o OS e retorna um handle de acesso a recursos externos
        .expect("falha ao inicializar o GLFW");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    )); // devem sempre ser configurados antes do .create_window do contrario nao tem efeito
        // habilita funcoes novas e desativa antigas (lentas e obsoletas)

    let (mut window, events) = glfw
        .create_window(800, 600, "p sim v0.1", glfw::WindowMode::Windowed)
        .expect("falha ao criar a janela");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|s| glfw.get_proc_address_raw(s) as *const _);

    unsafe  {
        gl::ClearColor(0.1, 0.2, 0.3, 1.0);
    }

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(
                glfw::Key::Escape, _, glfw::Action::Press, _
                ) = event {
                    window.set_should_close(true);
                }

            if let glfw::WindowEvent::FramebufferSize(w, h) = event {
                unsafe { gl::Viewport(0, 0, w, h); }
                }
        }
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        window.swap_buffers();
    }
}
