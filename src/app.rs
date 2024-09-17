use crate::egui::*;

use crate::canvas::Canvas;

pub struct Painting {
    canvas: Canvas,
}

impl Default for Painting {
    fn default() -> Self {
        let canvas = Canvas::new(500, 500, 255);
        Self { canvas }
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
    fn ui_menus(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("Click for menu", |ui| self.nested_menus(ui));
        });
    }

    fn nested_menus(&mut self, ui: &mut Ui) {
        ui.set_max_width(200.0); // To make sure we wrap long text

        if ui.button("Change canvas color").clicked() {
            self.canvas = Canvas::new(500, 500, 100);
        }
        ui.menu_button("SubMenu", |ui| {
            ui.menu_button("SubMenu", |ui| {
                if ui.button("Open…").clicked() {
                    ui.close_menu();
                }
                let _ = ui.button("Item");
            });
            ui.menu_button("SubMenu", |ui| {
                if ui.button("Open…").clicked() {
                    ui.close_menu();
                }
                let _ = ui.button("Item");
            });
            let _ = ui.button("Item");
            if ui.button("Open…").clicked() {
                ui.close_menu();
            }
        });
        ui.menu_button("SubMenu", |ui| {
            let _ = ui.button("Item1");
            let _ = ui.button("Item2");
            let _ = ui.button("Item3");
            let _ = ui.button("Item4");
            if ui.button("Open…").clicked() {
                ui.close_menu();
            }
        });
        let _ = ui.button("Very long text for this item that should be wrapped");
    }

    fn ui_canvas(&self, ui: &mut Ui) -> Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        let image = ColorImage::from_rgba_unmultiplied([500, 500], self.canvas.frame());

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
