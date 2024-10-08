// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod app;
mod canvas;
mod consts;
mod figure;

use app::Painting;
use consts::*;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_SIZE.0 as f32, WINDOW_SIZE.1 as f32]),
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
