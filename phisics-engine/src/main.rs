use glfw::Context; //semelhante a uma interface
use gl::types::{GLchar, GLenum, GLint, GLuint, GLsizeiptr, GLsizei };
use std::ffi::{CString, c_void};

// Codigo GLSL dos dois shaders. Sao apenas strings: quem compila e o driver
// da GPU, em tempo de execucao. r#"..."# e uma raw string (nao interpreta escapes).
const VERTEX_SRC: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
"#;

const FRAGMENT_SRC: &str = r#"
#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0, 0.5, 0.2, 1.0);
}
"#;

fn main() {

    let mut glfw: glfw::Glfw = glfw::init(glfw::FAIL_ON_ERRORS) //conversa com o OS e retorna um handle de acesso a recursos externos
    .expect("some king of error happend");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core,));
    // devem sempre ser configurados antes do .create_window do contrario nao tem efeito
    // habilita funcoes novas e desativa antigas (lentas e obsoletas)

    let (mut window, events) = glfw
        .create_window(800, 600, "p sim v0.1", glfw::WindowMode::Windowed)
        .expect("Error creating window");


    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);


    gl::load_with(|s| glfw.get_proc_address_raw(s) as *const _);

    unsafe  {
        gl::ClearColor(0.1, 0.2, 0.4, 1.0);
    }

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // inferior esquerdo
         0.5, -0.5, 0.0, // inferior direito
         0.0,  0.5, 0.0, // topo
    ];

    let (programa, vao): (GLuint, GLuint) = unsafe {
        let vs = compile_shaders(gl::VERTEX_SHADER, VERTEX_SRC);
        let fs = compile_shaders(gl::FRAGMENT_SHADER, FRAGMENT_SRC);
        let programa = link_program(vs, fs);
        // ja estao copiados dentro do programa linkado, nao servem mais
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);

        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as GLsizeiptr,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,                                              // location, casa com o shader
            3,                                              // componentes por vértice
            gl::FLOAT,                                      // tipo
            gl::FALSE,                                      // normalizar?
            (3 * std::mem::size_of::<f32>()) as GLsizei,    // stride
            std::ptr::null(),                               // offset
        );
        gl::EnableVertexAttribArray(0);

        // serve de unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        (programa, vao)
    };

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

            gl::UseProgram(programa);            // qual programa
            gl::BindVertexArray(vao);            // quais dados
            gl::DrawArrays(gl::TRIANGLES, 0, 3); // desenha
        }

        window.swap_buffers();
    }
}


fn compile_shaders(t:GLenum, source: &str) -> GLuint {

    unsafe {
        let shader = gl::CreateShader(t);
        let c_fonte = CString::new(source).expect("shader source has null byte");
        gl::ShaderSource(shader, 1, &c_fonte.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut sucess: GLint = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut sucess);

        if sucess == 0 {
            let mut size: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut size);
            let mut buf = vec![0u8; size as usize];
            gl::GetShaderInfoLog(shader, size, std::ptr::null_mut(),
                                 buf.as_mut_ptr() as *mut GLchar);
            // a mensagem do driver e o unico motivo desta funcao existir
            panic!("error compiling shader:\n{}", String::from_utf8_lossy(&buf));
        }
        shader
    }
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {

    unsafe {
        let programa = gl::CreateProgram();
        gl::AttachShader(programa, vs);
        gl::AttachShader(programa, fs);
        gl::LinkProgram(programa);

        let mut sucess: GLint = 0;
        gl::GetProgramiv(programa, gl::LINK_STATUS, &mut sucess);

        if sucess == 0 {
            let mut size: GLint = 0;
            gl::GetProgramiv(programa, gl::INFO_LOG_LENGTH, &mut size);
            let mut buf = vec![0u8; size as usize];
            gl::GetProgramInfoLog(programa, size, std::ptr::null_mut(),
                                  buf.as_mut_ptr() as *mut GLchar);
            panic!("error linking program:\n{}", String::from_utf8_lossy(&buf));
        }
        programa
    }
}
