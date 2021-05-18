mod apps;
mod core;

use crate::apps::imgui::ImGuiApp;
use crate::core::framework::FrameworkBuilder;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

fn main() -> Result<(), String> {
    FrameworkBuilder::new()
        .window("ImGui Demo Window", WINDOW_WIDTH, WINDOW_HEIGHT)
        .use_imgui()
        .build()?
        .run::<ImGuiApp>()
}
