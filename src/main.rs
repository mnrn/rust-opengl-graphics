mod apps;
mod core;

use crate::apps::texture_mapping::TextureMappingApp;
use crate::core::app::App;
use crate::core::common;
use crate::core::framework::FrameworkBuilder;

fn main() -> Result<(), String> {
    let fw = FrameworkBuilder::new()
        .window(
            "Hello Triangle!",
            common::WINDOW_WIDTH,
            common::WINDOW_HEIGHT,
        )
        .build()?;

    let ctx = fw.context();
    let app = TextureMappingApp::new(ctx);

    fw.run(app)
}
