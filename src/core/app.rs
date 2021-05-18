use super::framework::Context;
use imgui_glfw_rs::imgui;

#[allow(unused_variables)]
pub trait App {
    fn new(ctx: &Context) -> Self;
    fn update(&mut self, ctx: &Context) -> Result<(), String> {
        Ok(())
    }
    fn render(&self, ctx: &Context) -> Result<(), String> {
        Ok(())
    }
    fn ui_overlay<'a>(&self, ui: imgui::Ui<'a>) -> Option<imgui::Ui<'a>> {
        None
    }
    fn destroy(&mut self, ctx: &Context) -> Result<(), String> {
        Ok(())
    }

    fn resized(&mut self, ctx: &Context, width: u32, height: u32) -> Result<(), String> {
        ctx.set_viewport();
        Ok(())
    }
}
