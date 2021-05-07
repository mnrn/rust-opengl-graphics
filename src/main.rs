mod apps;
mod core;

use crate::apps::minimal::MinimalApp;
use crate::core::common;
use crate::core::framework::FrameworkBuilder;

fn main() -> Result<(), String> {
    let fw = FrameworkBuilder::new()
        .window("App Framework", common::WINDOW_WIDTH, common::WINDOW_HEIGHT)
        .build()?;

    let app = MinimalApp::new();

    fw.run(app)
}
