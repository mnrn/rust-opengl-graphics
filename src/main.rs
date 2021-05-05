use std::mem;
use std::os::raw::c_void;
use std::time::Duration;

use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod vertex;
mod shader;

use vertex::VertexArray;
use shader::Shader;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    let (major, minor) = gl_attr.context_version();
    debug_assert_eq!(gl_attr.context_profile(), sdl2::video::GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (4, 1));
    println!("OK! Init OpenGL Version = {}.{}", major, minor);

    let window = video_subsystem
        .window("App Window", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let shader = Shader::new("res/glsl/triangle.vs.glsl", "res/glsl/triangle.fs.glsl")
        .unwrap_or_else(|e| panic!("{}", e));

    let vao = VertexArray::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        unsafe {
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader.use_program();
            
            vao.draw_arrays(gl::TRIANGLES, 0, 3);
        }
        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
