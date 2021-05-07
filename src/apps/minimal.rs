use crate::core::app::App;
use crate::core::common;
use crate::core::shader::Shader;
use crate::core::vertex::VertexArray;

pub struct MinimalApp {
    vao: VertexArray,
    shader: Shader,
}

#[allow(dead_code)]
impl MinimalApp {
    pub fn new() -> MinimalApp {
        let shader = Shader::new("res/glsl/triangle.vs.glsl", "res/glsl/triangle.fs.glsl").unwrap();
        let vao = VertexArray::new();

        MinimalApp {
            vao: vao,
            shader: shader,
        }
    }
}

impl App for MinimalApp {
    fn render(&self) -> Result<(), String> {
        unsafe {
            gl::Viewport(
                0,
                0,
                common::WINDOW_WIDTH as i32,
                common::WINDOW_HEIGHT as i32,
            );

            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.shader.use_program();
            self.vao.draw_arrays(gl::TRIANGLES, 0, 3);
        }
        Ok(())
    }
}
