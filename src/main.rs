// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod consts;
mod app;
mod canvas;

use consts::*;
use app::Painting;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        "Canva",
        options,
        Box::new(|_cc| {
            // // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<Painting>::default())
        }),
    )
}
