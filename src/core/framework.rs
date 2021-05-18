use std::default::Default;
use std::marker::PhantomData;
use std::sync::mpsc::Receiver;

use super::app::App;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;

pub struct ImGui {
    ctx: imgui::Context,
    glfw: ImguiGLFW,
}

pub struct Context {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    width: u32,
    height: u32,
    imgui: Option<ImGui>,
}

#[allow(dead_code)]
impl Context {
    pub fn set_viewport(&self) {
        unsafe {
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
    }

    pub fn change_view(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.set_viewport();
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

    pub fn ui_overlay<A>(&mut self, app: &A)
    where
        A: App,
    {
        if let Some(ref mut imgui) = self.imgui {
            let ui = app.ui_overlay(imgui.glfw.frame(&mut self.window, &mut imgui.ctx));
            imgui.glfw.draw(ui, &mut self.window);
        }
    }

    fn process_events<A>(&mut self, app: &mut A) -> Result<(), String>
    where
        A: App,
    {
        for (_, event) in glfw::flush_messages(&self.events) {
            // Imgui Event Hangle
            if let Some(ref mut imgui) = self.imgui {
                imgui.glfw.handle_event(&mut imgui.ctx, &event);
            }
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    self.width = width as u32;
                    self.height = height as u32;
                    app.resized(self, width as u32, height as u32)?
                }
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn run<A>(&mut self) -> Result<(), String>
    where
        A: App,
    {
        let mut app = A::new(&self);

        // Render loop
        while !self.window.should_close() {
            // App update
            app.update(&self)?;

            // App render
            app.render(&self)?;

            // App UI Overlay
            self.ui_overlay(&app);

            // glfw: Swap buffers and poll IO events(key pressed/released, mouse moved etc.)
            glfw::Context::swap_buffers(&mut self.window);
            self.glfw.poll_events();
            self.process_events(&mut app)?;
        }

        // Destroy Application
        app.destroy(&self)
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
    use_imgui: bool,
    state: PhantomData<Framework>,
}

#[allow(dead_code)]
impl FrameworkBuilder<Empty> {
    pub fn new() -> Self {
        FrameworkBuilder {
            title: Default::default(),
            width: Default::default(),
            height: Default::default(),
            use_imgui: false,
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
            use_imgui: self.use_imgui,
            state: PhantomData,
        }
    }
}

impl FrameworkBuilder<Fully> {
    #[allow(dead_code)]
    pub fn use_imgui(self) -> FrameworkBuilder<Fully> {
        FrameworkBuilder {
            title: self.title,
            width: self.width,
            height: self.height,
            use_imgui: true,
            state: PhantomData,
        }
    }

    pub fn build(self) -> Result<Context, String> {
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
        window.set_all_polling(true);

        // Polling of events can be turned on and off by the specific event type
        // window.set_pos_polling(true);
        // window.set_all_polling(true);
        // window.set_size_polling(true);
        // window.set_close_polling(true);
        // window.set_refresh_polling(true);
        // window.set_focus_polling(true);
        // window.set_iconify_polling(true);
        // window.set_framebuffer_size_polling(true);
        // window.set_key_polling(true);
        // window.set_char_polling(true);
        // window.set_char_mods_polling(true);
        // window.set_mouse_button_polling(true);
        // window.set_cursor_pos_polling(true);
        // window.set_cursor_enter_polling(true);
        // window.set_scroll_polling(true);
        // window.set_maximize_polling(true);
        // window.set_content_scale_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // Imgui creation
        let imgui = if self.use_imgui {
            let mut imgui = imgui::Context::create();
            imgui.set_ini_filename(None);
            let imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);
            Some(ImGui {
                ctx: imgui,
                glfw: imgui_glfw,
            })
        } else {
            None
        };

        Ok(Context {
            glfw: glfw,
            window: window,
            events: events,
            width: self.width,
            height: self.height,
            imgui: imgui,
        })
    }
}
