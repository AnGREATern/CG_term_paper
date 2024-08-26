use crate::egui::*;

use crate::canvas::Canvas;

pub struct Painting {
    // /// in 0-1 normalized coordinates
    // lines: Vec<Vec<Pos2>>,
    // stroke: Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            // lines: Default::default(),
            // stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
        }
    }
}

impl eframe::App for Painting {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
        //     ui.horizontal(|ui| {
                // ui.label("Base window");
        //         ui.add(&mut self.stroke);
        //         ui.separator();
        //         if ui.button("Clear Painting").clicked() {
        //             self.lines.clear();
        //         }
        //     })
        //     .response;

        // self.ui_control(ui);
        // ui.label("Paint with your mouse/touch!");
        // Frame::canvas(ui.style()).show(ui, |ui| {
        //     self.ui_content(ui);
        // });

        self.ui_canvas(ui);

        // if ctx.input(|i| i.key_pressed(Key::Space)) {
        //     let mut open = true;
        //     self.a_handler(ctx, &mut open);
        // }
        });
    }
}

impl Painting {
    // fn ui(&mut self, ui: &mut Ui) {
    //     self.ui_control(ui);
    //     ui.label("Paint with your mouse/touch!");
    //     Frame::canvas(ui.style()).show(ui, |ui| {
    //         self.ui_content(ui);
    //     });
    // }

    // fn a_handler(&mut self, ctx: &Context, open: &mut bool) {
    //     Window::new("Wow")
    //         .open(open)
    //         .default_size(vec2(512.0, 512.0))
    //         .vscroll(false)
    //         .show(ctx, |ui| self.ui(ui));
    // }

    fn ui_canvas(&self, ui: &mut Ui) -> Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        let canvas = Canvas::new(500, 500);
        let image = ColorImage::from_rgba_unmultiplied(
            [500, 500],
            canvas.frame(),
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

    // pub fn ui_control(&mut self, ui: &mut Ui) -> Response {
    //     ui.horizontal(|ui| {
    //         ui.label("Stroke:");
    //         ui.add(&mut self.stroke);
    //         ui.separator();
    //         if ui.button("Clear Painting").clicked() {
    //             self.lines.clear();
    //         }
    //     })
    //     .response
    // }

    // pub fn ui_content(&mut self, ui: &mut Ui) -> Response {
    //     let (mut response, painter) =
    //         ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

    //     let to_screen = emath::RectTransform::from_to(
    //         Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
    //         response.rect,
    //     );
    //     let from_screen = to_screen.inverse();

    //     if self.lines.is_empty() {
    //         self.lines.push(vec![]);
    //     }

    //     let current_line = self.lines.last_mut().unwrap();

    //     if let Some(pointer_pos) = response.interact_pointer_pos() {
    //         let canvas_pos = from_screen * pointer_pos;
    //         if current_line.last() != Some(&canvas_pos) {
    //             current_line.push(canvas_pos);
    //             response.mark_changed();
    //         }
    //     } else if !current_line.is_empty() {
    //         self.lines.push(vec![]);
    //         response.mark_changed();
    //     }

    //     let shapes = self
    //         .lines
    //         .iter()
    //         .filter(|line| line.len() >= 2)
    //         .map(|line| {
    //             let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
    //             Shape::line(points, self.stroke)
    //         });

    //     painter.extend(shapes);

    //     response
    // }

    // pub fn ui_try(&mut self, ui: &mut Ui) -> Response {
    //     let size = ui.available_size_before_wrap();
    //     let (mut response, painter) =
    //         ui.allocate_painter(size, Sense::drag());
    //     let rect = response.rect;
    //     let c = rect.center();
    //     let r = rect.width() / 2.0 - 1.0;
    //     let color = Color32::from_gray(128);
    //     let stroke = Stroke::new(1.0, color);

    //     painter.add(shape)

    //     response
    // }
}
