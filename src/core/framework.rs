use std::default::Default;
use std::marker::PhantomData;
use std::sync::mpsc::Receiver;

use glfw::{Action, Key};

use super::app::App;
use super::imgui::ImGui;

pub struct Context {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Context {
    pub fn set_viewport(&self) {
        unsafe {
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
    }

    pub fn clear_buffer(&self, mask: gl::types::GLenum) {
        unsafe {
            gl::Clear(mask);
        }
    }

    pub fn aspect(&self) -> f32 {
        assert!(self.height != 0);
        self.width as f32 / self.height as f32
    }

    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}

pub struct Framework {
    ctx: Context,
}

impl Framework {
    pub fn run<A>(&mut self) -> Result<(), String>
    where
        A: App,
    {
        let mut app = A::new(&self.ctx);

        // Render loop
        while !self.ctx.window.should_close() {
            // Events
            self.ctx.process_events();

            // App update
            app.update(&self.ctx)?;

            // App render
            app.render(&self.ctx)?;

            // glfw: Swap buffers and poll IO events(key pressed/released, mouse moved etc.)
            glfw::Context::swap_buffers(&mut self.ctx.window);
            self.ctx.glfw.poll_events();
        }

        // Destroy Application
        app.destroy(&self.ctx)
    }
}

#[allow(dead_code)]
pub struct Empty;
#[allow(dead_code)]
pub struct Fully;

#[allow(dead_code)]
pub struct FrameworkBuilder<Framework> {
    title: String,
    width: u32,
    height: u32,
    state: PhantomData<Framework>,
}

#[allow(dead_code)]
impl FrameworkBuilder<Empty> {
    pub fn new() -> Self {
        FrameworkBuilder {
            title: Default::default(),
            width: Default::default(),
            height: Default::default(),
            state: PhantomData,
        }
    }
}

impl FrameworkBuilder<Empty> {
    pub fn window<S>(self, title: S, width: u32, height: u32) -> FrameworkBuilder<Fully>
    where
        S: Into<String>,
    {
        FrameworkBuilder {
            title: title.into(),
            width: width,
            height: height,
            state: PhantomData,
        }
    }
}

impl FrameworkBuilder<Fully> {
    pub fn build(self) -> Result<Framework, String> {
        // glfw: initialize and configure
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        #[cfg(debug_assertions)]
        glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

        // glfw window creation
        let (mut window, events) = glfw
            .create_window(
                self.width,
                self.height,
                &self.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");
        glfw::Context::make_current(&mut window);
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Ok(Framework {
            ctx: Context {
                glfw: glfw,
                window: window,
                events: events,
                width: self.width,
                height: self.height,
            },
        })
    }
}
