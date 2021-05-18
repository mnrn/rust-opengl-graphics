use imgui_glfw_rs::imgui;

use crate::core::app::App;
use crate::core::framework::Context;
pub struct ImGuiApp {}

#[allow(dead_code)]
impl App for ImGuiApp {
    fn new(ctx: &Context) -> ImGuiApp {
        ctx.set_viewport();
        ctx.set_clear_color(1.0, 1.0, 1.0, 1.0);

        ImGuiApp {}
    }

    fn render(&self, ctx: &Context) -> Result<(), String> {
        ctx.clear_buffer(gl::COLOR_BUFFER_BIT);
        Ok(())
    }

    fn ui_overlay<'a>(&self, ui: imgui::Ui<'a>) -> imgui::Ui<'a> {
        ui.show_demo_window(&mut true);
        ui
    }
}
