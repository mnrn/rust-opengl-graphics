use std::default::Default;
use std::marker::PhantomData;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::app::App;

pub struct Context {
    width: u32,
    height: u32,
    _gl: sdl2::video::GLContext,
    sdl: sdl2::Sdl,
}

impl Context {
    pub unsafe fn set_viewport(&self) {
        gl::Viewport(0, 0, self.width as i32, self.height as i32);
    }

    pub fn aspect(&self) -> f32 {
        assert!(self.height != 0);
        self.width as f32 / self.height as f32
    }
}

pub struct Framework {
    window: sdl2::video::Window,
    ctx: Context,
}

impl Framework {
    pub fn context(&self) -> &Context {
        &self.ctx
    }

    pub fn run<A>(&self, app: A) -> Result<(), String>
    where
        A: App,
    {
        let mut event_pump = self.ctx.sdl.event_pump()?;

        // Event Loop
        'running: loop {
            // Update Application
            app.update(&self.ctx)?;

            // Render Application
            app.render(&self.ctx)?;

            // Swap framebuffer
            self.window.gl_swap_window();

            // Pull Event
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
            ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
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
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        // Init OpenGL
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);
        let (major, minor) = gl_attr.context_version();
        debug_assert_eq!(gl_attr.context_profile(), sdl2::video::GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (4, 1));
        println!("OK! Init OpenGL Version = {}.{}", major, minor);

        // Create Framework
        let window = video_subsystem
            .window(&self.title, self.width, self.height)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        // Load OpenGL
        let gl_context = window.gl_create_context()?;
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        Ok(Framework {
            window: window,
            ctx: Context {
                width: self.width,
                height: self.height,
                _gl: gl_context,
                sdl: sdl_context,
            },
        })
    }
}
