pub trait App {
    fn update(&self) -> Result<(), String> {
        Ok(())
    }
    fn render(&self) -> Result<(), String> {
        Ok(())
    }
    fn destroy(&self) -> Result<(), String> {
        Ok(())
    }
}
