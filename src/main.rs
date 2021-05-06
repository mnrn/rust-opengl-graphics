mod app;
mod buffer;
mod framework;
mod mesh;
mod shader;
mod vertex;

use app::App;
use buffer::Buffer;
use framework::FrameworkBuilder;
use shader::Shader;
use vertex::VertexArray;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

struct MyApp {
    vao: VertexArray,
    _vbo: Buffer,
    shader: Shader,
}

impl MyApp {
    fn new() -> MyApp {
        let shader = Shader::new("res/glsl/basic.vs.glsl", "res/glsl/basic.fs.glsl").unwrap();
        let vertices = [
            -0.5f32, -0.5f32, 0.0f32, 0.5f32, -0.5f32, 0.0f32, 0.0f32, 0.5f32, 0.0f32,
        ];
        let vao = VertexArray::new();
        let vbo = Buffer::new(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW);
        vao.init(|| unsafe {
            vbo.vertex_attrib_pointer(0, 3, gl::FLOAT, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        });
        MyApp {
            vao: vao,
            _vbo: vbo,
            shader: shader,
        }
    }
}

impl App for MyApp {
    fn render(&self) -> Result<(), String> {
        unsafe {
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.shader.use_program();
            self.vao.draw_arrays(gl::TRIANGLES, 0, 3);
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let fw = FrameworkBuilder::new()
        .window("App Framework", WINDOW_WIDTH, WINDOW_HEIGHT)
        .build()?;

    let app = MyApp::new();

    fw.run(app)
}
