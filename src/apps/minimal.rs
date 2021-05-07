use crate::core::app::App;
use crate::core::framework::Context;
use crate::core::shader::Shader;
use crate::core::vertex::VertexArray;

pub struct MinimalApp {
    vao: VertexArray,
    shader: Shader,
}

#[allow(dead_code)]
impl App for MinimalApp {
    fn new(_: &Context) -> MinimalApp {
        let shader = Shader::new("res/glsl/triangle.vs.glsl", "res/glsl/triangle.fs.glsl").unwrap();
        let vao = VertexArray::new();

        MinimalApp {
            vao: vao,
            shader: shader,
        }
    }

    fn render(&self, ctx: &Context) -> Result<(), String> {
        unsafe {
            ctx.set_viewport();

            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.shader.use_program();
            self.vao.draw_arrays(gl::TRIANGLES, 0, 3);
        }
        Ok(())
    }
}
