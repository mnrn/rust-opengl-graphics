mod apps;
mod core;

use crate::apps::texture_mapping::TextureMappingApp;
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

    fw.run::<TextureMappingApp>()
}
