use std::mem::swap;

use crate::egui::{
    widgets::color_picker, CentralPanel, Color32, ColorImage, Context, Event, Mesh, Pos2, Rect,
    Response, Sense, Shape, Ui,
};
use crate::figure::object::Object;
use crate::figure::projection::Projection;
use crate::figure::vertex::Vertex;
use eframe::egui::color_picker::Alpha;
use eframe::egui::{PointerButton, Vec2};
use rfd::FileDialog;

use crate::canvas::Canvas;
use crate::{BACKGROUND_COLOR, DEFAULT_SCALE, SPHERE_RADIUS, WINDOW_SIZE};

pub struct Painting {
    is_start_obj_viewed: bool,
    start_obj: Option<Object>,
    result_obj: Option<Object>,
    canvas: Canvas,
    obj_color: Color32,
    is_movement_access: bool,
    is_rotating_access: bool,
    light_direction: Vertex,
}

impl Default for Painting {
    fn default() -> Self {
        let is_start_obj_viewed = true;
        let start_obj = None;
        let result_obj = None;
        let canvas = Canvas::new(WINDOW_SIZE.0, WINDOW_SIZE.1, &BACKGROUND_COLOR);
        let obj_color = Color32::WHITE;
        let is_movement_access = false;
        let is_rotating_access = false;
        let light_direction = Vertex::new(0., 0., -1.);
        Self {
            is_start_obj_viewed,
            start_obj,
            result_obj,
            canvas,
            obj_color,
            is_movement_access,
            is_rotating_access,
            light_direction,
        }
    }
}

impl eframe::App for Painting {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.ui_menus(ui);
            self.ui_canvas(ui);
            ui.input(|i| {
                for event in &i.raw.events {
                    if let Event::MouseMoved(pos) = event {
                        if self.is_movement_access {
                            self.move_object(pos);
                        }
                        if self.is_rotating_access {
                            self.rotate_object(pos);
                        }
                    }
                    if let Event::PointerButton {
                        button,
                        pressed,
                        modifiers,
                        ..
                    } = event
                    {
                        match button {
                            PointerButton::Primary => {
                                self.is_movement_access = *pressed && modifiers.ctrl
                            }
                            PointerButton::Secondary => {
                                self.is_rotating_access = *pressed && modifiers.ctrl
                            }
                            _ => {}
                        }
                    }
                }
            })
        });
    }
}

impl Painting {
    fn button_load_start_label(&self) -> &'static str {
        if self.start_obj.is_some() {
            "Start object loaded ✅"
        } else {
            "Load start object..."
        }
    }

    fn button_load_result_label(&self) -> &'static str {
        if self.result_obj.is_some() {
            "Result object loaded ✅"
        } else {
            "Load result object..."
        }
    }

    fn button_view_start_obj_label(&self) -> &'static str {
        if self.is_start_obj_viewed {
            "Start object ✅"
        } else {
            "Start object"
        }
    }

    fn button_view_result_obj_label(&self) -> &'static str {
        if self.is_start_obj_viewed {
            "Result object"
        } else {
            "Result object ✅"
        }
    }

    fn ui_menus(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("Load objects", |ui| self.load_obj_nested_menus(ui));
            ui.menu_button("View", |ui| self.view_nested_menus(ui));
            ui.menu_button("Pick color", |ui| self.pick_color(ui));
            ui.menu_button("Move light source", |ui| {
                self.move_light_src_nested_menus(ui)
            });
            // ui.menu_button("Move object", |ui| self.move_obj(ui));
            ui.menu_button("Run morphing", |ui| self.morph(ui));
        });
    }

    fn morph(&mut self, ui: &mut Ui) {
        if self.start_obj.is_none() || self.result_obj.is_none() {
            return;
        }

        let start_proj = Projection::new(self.start_obj.as_ref().unwrap(), SPHERE_RADIUS);
        let result_proj = Projection::new(self.result_obj.as_ref().unwrap(), SPHERE_RADIUS);
        todo!();
    }

    fn move_light_src_nested_menus(&mut self, ui: &mut Ui) {
        if ui.button("Move right").clicked() {
            let delta = Vertex::new(-0.1, 0., 0.);
            self.light_direction.mov(delta);
            self.draw_object();
        }
        if ui.button("Move left").clicked() {
            let delta = Vertex::new(0.1, 0., 0.);
            self.light_direction.mov(delta);
            self.draw_object();
        }
        if ui.button("Move up").clicked() {
            let delta = Vertex::new(0., 0.1, 0.);
            self.light_direction.mov(delta);
            self.draw_object();
        }
        if ui.button("Move down").clicked() {
            let delta = Vertex::new(0.0, -0.1, 0.);
            self.light_direction.mov(delta);
            self.draw_object();
        }
    }

    fn move_object(&mut self, delta: &Vec2) {
        let delta = Vertex::new(
            delta.x as f64 / self.canvas.width() as f64 * DEFAULT_SCALE,
            delta.y as f64 / self.canvas.height() as f64 * DEFAULT_SCALE,
            0.,
        );
        let object = match self.is_start_obj_viewed {
            true => &mut self.start_obj,
            false => &mut self.result_obj,
        };
        if let Some(object) = object {
            object.mov(delta);
            self.draw_object();
        }
    }

    fn rotate_object(&mut self, delta: &Vec2) {
        let delta = Vertex::new(
            -delta.y as f64 / self.canvas.height() as f64,
            delta.x as f64 / self.canvas.width() as f64,
            0.,
        );
        let object = match self.is_start_obj_viewed {
            true => &mut self.start_obj,
            false => &mut self.result_obj,
        };
        if let Some(object) = object {
            object.rotate(delta);
            self.draw_object();
        }
    }

    fn pick_color(&mut self, ui: &mut Ui) {
        color_picker::color_picker_color32(ui, &mut self.obj_color, Alpha::Opaque);
    }

    fn view_nested_menus(&mut self, ui: &mut Ui) {
        ui.set_max_width(150.0); // To make sure we wrap long text

        if ui.button(self.button_view_start_obj_label()).clicked() {
            if !self.is_start_obj_viewed {
                self.is_start_obj_viewed = true;
                self.draw_object();
            }
        }
        if ui.button(self.button_view_result_obj_label()).clicked() {
            if self.is_start_obj_viewed {
                self.is_start_obj_viewed = false;
                self.draw_object();
            }
        }
    }

    fn load_obj_nested_menus(&mut self, ui: &mut Ui) {
        ui.set_max_width(150.0); // To make sure we wrap long text

        if ui.button(self.button_load_start_label()).clicked() {
            if let Some(filename) = FileDialog::new().add_filter("obj", &["obj"]).pick_file() {
                let loading =
                    Object::load(filename.to_str().unwrap_or(""), self.obj_color.to_array());
                if loading.is_ok() {
                    self.start_obj = Some(loading.unwrap());
                    self.is_start_obj_viewed = true;
                    self.draw_object();
                } else {
                    println!("{:?}", loading.err());
                }
            }
        }
        if ui.button(self.button_load_result_label()).clicked() {
            if let Some(filename) = FileDialog::new().add_filter("obj", &["obj"]).pick_file() {
                let loading =
                    Object::load(filename.to_str().unwrap_or(""), self.obj_color.to_array());
                if loading.is_ok() {
                    self.result_obj = Some(loading.unwrap());
                    self.is_start_obj_viewed = false;
                    self.draw_object();
                } else {
                    println!("{:?}", loading.err());
                }
            }
        }
        if ui.button("Swap objects").clicked() {
            swap(&mut self.start_obj, &mut self.result_obj);
            self.is_start_obj_viewed ^= true;
        }
    }

    fn draw_object(&mut self) {
        self.canvas.clear();
        let object = match self.is_start_obj_viewed {
            true => &self.start_obj,
            false => &self.result_obj,
        };
        if let Some(object) = object {
            self.canvas.draw_object(object, self.light_direction);
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
        // painter.add(Shape::LineSegment { points: (), stroke: () })
        painter.add(Shape::mesh(mesh));

        response
    }
}
