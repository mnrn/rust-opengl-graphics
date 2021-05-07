use super::framework::Context;

#[allow(unused_variables)]
pub trait App {
    fn new(ctx: &Context) -> Self;
    fn update(&self, ctx: &Context) -> Result<(), String> {
        Ok(())
    }
    fn render(&self, ctx: &Context) -> Result<(), String> {
        Ok(())
    }
    fn destroy(&self, ctx: &Context) -> Result<(), String> {
        Ok(())
    }
}
