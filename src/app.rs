use std::mem::swap;

use crate::egui::{
    CentralPanel, Color32, ColorImage, Context, Mesh, Pos2, Rect, Response, Sense, Shape, Ui,
};
use rfd::FileDialog;

use crate::canvas::Canvas;
use crate::WINDOW_SIZE;

pub struct Painting {
    is_start_obj_loaded: bool,
    is_result_obj_loaded: bool,
    canvas: Canvas,
}

impl Default for Painting {
    fn default() -> Self {
        let is_start_obj_loaded = false;
        let is_result_obj_loaded = false;
        let canvas = Canvas::new(WINDOW_SIZE.0, WINDOW_SIZE.1, 255);
        Self {
            is_start_obj_loaded,
            is_result_obj_loaded,
            canvas,
        }
    }
}

impl eframe::App for Painting {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.ui_menus(ui);
            self.ui_canvas(ui);
        });
    }
}

impl Painting {
    fn button_load_start_label(&self) -> &'static str {
        if self.is_start_obj_loaded {
            "Start object loaded ✅"
        } else {
            "Load start object..."
        }
    }

    fn button_load_result_label(&self) -> &'static str {
        if self.is_result_obj_loaded {
            "Result object loaded ✅"
        } else {
            "Load result object..."
        }
    }

    fn ui_menus(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("Load objects", |ui| self.load_obj_nested_menus(ui));
        });
    }

    fn load_obj_nested_menus(&mut self, ui: &mut Ui) {
        ui.set_max_width(150.0); // To make sure we wrap long text

        if ui.button(self.button_load_start_label()).clicked() {
            if let Some(object_path) = FileDialog::new().add_filter("obj", &["obj"]).pick_file() {
                self.is_start_obj_loaded = true;
            } else {
                self.is_start_obj_loaded = false;
            }
        }
        if ui.button(self.button_load_result_label()).clicked() {
            if let Some(object_path) = FileDialog::new().add_filter("obj", &["obj"]).pick_file() {
                self.is_result_obj_loaded = true;
            } else {
                self.is_result_obj_loaded = false;
            }
        }
        if ui.button("Swap objects").clicked() {
            swap(
                &mut self.is_start_obj_loaded,
                &mut self.is_result_obj_loaded,
            );
        }
    }

    fn ui_canvas(&self, ui: &mut Ui) -> Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        let image = ColorImage::from_rgba_unmultiplied(
            [WINDOW_SIZE.0 as usize, WINDOW_SIZE.1 as usize],
            self.canvas.frame(),
        );

        let texture = painter
            .ctx()
            .load_texture("Canvas", image, Default::default());

        let mut mesh = Mesh::with_texture(texture.id());
        mesh.add_rect_with_uv(
            painter.clip_rect(),
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
        painter.add(Shape::mesh(mesh));

        response
    }
}
