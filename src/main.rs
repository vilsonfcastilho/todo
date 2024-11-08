mod todoapp;

mod prelude {
    pub use eframe::{egui, App, Frame};
    pub use egui::*;

    pub use crate::todoapp::*;
}

use prelude::*;

fn main() -> eframe::Result<()> {
    let native_options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size(egui::Vec2::new(260., 360.)),
        ..Default::default()
    };

    eframe::run_native(
        "todo",
        native_options,
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(TodoApp::new(cc)))),
    )
}
