mod apps;
mod core;

use crate::apps::triangle::TriangleApp;
use crate::core::common;
use crate::core::framework::FrameworkBuilder;

fn main() -> Result<(), String> {
    let fw = FrameworkBuilder::new()
        .window("App Framework", common::WINDOW_WIDTH, common::WINDOW_HEIGHT)
        .build()?;

    let app = TriangleApp::new();

    fw.run(app)
}
