mod events;
mod labels;
mod menus;
mod ops;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::egui::{
    CentralPanel, Color32, ColorImage, Context, Event, Mesh, Pos2, Rect, Response, Sense, Shape, Ui,
};
use crate::figure::merged_object::MergedObject;
use crate::figure::object::Object;
use crate::figure::vertex::Vertex;
use crate::{BACKGROUND_COLOR, EPS, RATIO_STEP, WINDOW_SIZE};
use egui_notify::Toasts;

enum Mode {
    StartObjView,
    ResultObjView,
    Morphing,
}
pub struct Painting {
    mode: Mode,
    start_obj: Option<Object>,
    result_obj: Option<Object>,
    merged_obj: Option<MergedObject>,
    ratio: f64,
    step: f64,
    canvas: Canvas,
    obj_color: Color32,
    is_movement_access: bool,
    is_rotating_access: bool,
    light_direction: Vertex,
    toasts: Toasts,
}

impl Default for Painting {
    fn default() -> Self {
        let mode = Mode::StartObjView;
        let start_obj = None;
        let result_obj = None;
        let merged_obj = None;
        let ratio = 0.;
        let step = RATIO_STEP;
        let canvas = Canvas::new(WINDOW_SIZE.0, WINDOW_SIZE.1, Color::new(BACKGROUND_COLOR));
        let obj_color = Color32::WHITE;
        let is_movement_access = false;
        let is_rotating_access = false;
        let light_direction = Vertex::new(0., 0., 1.);
        let toasts = Toasts::default();
        Self {
            mode,
            start_obj,
            result_obj,
            merged_obj,
            ratio,
            step,
            canvas,
            obj_color,
            is_movement_access,
            is_rotating_access,
            light_direction,
            toasts,
        }
    }
}

impl eframe::App for Painting {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.ui_menus(ui);
            self.ui_canvas(ui);
            if let Mode::Morphing = self.mode {
                ctx.request_repaint();
            }
            ui.input(|i| {
                for event in &i.raw.events {
                    match event {
                        Event::MouseMoved(pos) => self.mouse_moved(pos),
                        Event::PointerButton {
                            button,
                            pressed,
                            modifiers,
                            ..
                        } => self.pointer_button(*button, *pressed, *modifiers),
                        Event::MouseWheel {
                            modifiers, delta, ..
                        } => self.mouse_wheel(*modifiers, delta),
                        _ => (),
                    }
                }
            });
        });
        self.toasts.show(ctx);
    }
}

impl Painting {
    fn ui_canvas(&mut self, ui: &mut Ui) -> Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        if self.merged_obj.is_some() {
            if self.ratio <= 0. {      
                if self.step < EPS {
                    self.merged_obj = None;
                    self.mode = Mode::ResultObjView;
                } else {
                    self.mode = Mode::StartObjView;
                }
                self.draw_object();
            } else if self.ratio >= 1. {
                self.mode = Mode::ResultObjView;
                self.draw_object();
                if self.step > EPS {
                    self.merged_obj = None;
                }
            } else {
                self.mode = Mode::Morphing;
                self.canvas.clear();
                self.canvas.draw_object(
                    &self.merged_obj.as_ref().unwrap().interpolation(self.ratio),
                    self.light_direction,
                );
            }
            self.ratio += self.step;
        }

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
