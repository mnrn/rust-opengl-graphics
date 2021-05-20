mod apps;
mod core;

use crate::apps::hello_triangle::HelloTriangleApp;
use crate::apps::imgui::ImGuiApp;
use crate::apps::minimal::MinimalApp;
use crate::apps::texture_mapping::TextureMappingApp;
use crate::core::framework::FrameworkBuilder;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "triangle" => FrameworkBuilder::new()
            .window("Hello Triangle", WINDOW_WIDTH, WINDOW_HEIGHT)
            .build()?
            .run::<HelloTriangleApp>(),
        "minimal" => FrameworkBuilder::new()
            .window("Minimal Application", WINDOW_WIDTH, WINDOW_HEIGHT)
            .build()?
            .run::<MinimalApp>(),
        "texture_mapping" => FrameworkBuilder::new()
            .window("Texture Mapping", WINDOW_WIDTH, WINDOW_HEIGHT)
            .build()?
            .run::<TextureMappingApp>(),
        "imgui" => FrameworkBuilder::new()
            .window("ImGui Demo App", WINDOW_WIDTH, WINDOW_HEIGHT)
            .use_imgui()
            .build()?
            .run::<ImGuiApp>(),
        _ => panic!("Could not support app for {}", args[1]),
    }
}
