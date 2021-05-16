use super::framework::Context;

#[allow(unused_variables)]
pub trait App {
    fn new(ctx: &Context) -> Self;
    fn update(&mut self, ctx: &Context) -> Result<(), String> {
        Ok(())
    }
    fn render(&self, ctx: &Context) -> Result<(), String> {
        Ok(())
    }
    fn ui_overray(&mut self, ctx: &Context) -> Result<(), String> {
        Ok(())
    }
    fn destroy(&mut self, ctx: &Context) -> Result<(), String> {
        Ok(())
    }
    fn event(&mut self, ev: &sdl2::event::Event) -> Result<(), String> {
        Ok(())
    }
    fn resized(&mut self, width: u32, height: u32) {}
}
