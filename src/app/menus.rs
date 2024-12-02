use super::{Mode, Painting};

use std::mem::swap;

use crate::color::Color;
use crate::egui::{widgets::color_picker, Ui};
use crate::figure::object::Object;
use crate::figure::vertex::Vertex;
use eframe::egui::color_picker::Alpha;
use rfd::FileDialog;

impl Painting {
    pub fn ui_menus(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("Загрузить объекты", |ui| self.load_obj_nested_menus(ui));
            ui.menu_button("Обозреваемый объект", |ui| self.view_nested_menus(ui));
            ui.menu_button("Выбор цвета", |ui| self.pick_color(ui));
            ui.menu_button("Перемещение источника света", |ui| {
                self.move_light_src_nested_menus(ui)
            });
            // ui.menu_button("Move object", |ui| self.move_obj(ui));
            ui.menu_button("Морфинг", |ui| self.morph(ui));
        });
    }

    fn move_light_src_nested_menus(&mut self, ui: &mut Ui) {
        if ui.button("Вправо").clicked() {
            let delta = Vertex::new(0.1, 0., 0.);
            self.light_direction.mov(delta);
            self.draw_object();
        }
        if ui.button("Влево").clicked() {
            let delta = Vertex::new(-0.1, 0., 0.);
            self.light_direction.mov(delta);
            self.draw_object();
        }
        if ui.button("Вверх").clicked() {
            let delta = Vertex::new(0., -0.1, 0.);
            self.light_direction.mov(delta);
            self.draw_object();
        }
        if ui.button("Вниз").clicked() {
            let delta = Vertex::new(0.0, 0.1, 0.);
            self.light_direction.mov(delta);
            self.draw_object();
        }
    }

    fn pick_color(&mut self, ui: &mut Ui) {
        color_picker::color_picker_color32(ui, &mut self.obj_color, Alpha::Opaque);
    }

    fn view_nested_menus(&mut self, ui: &mut Ui) {
        ui.set_max_width(150.0); // To make sure we wrap long text

        if ui.button(self.button_view_start_obj_label()).clicked() {
            match self.mode {
                Mode::StartObjView => (),
                _ => {
                    self.mode = Mode::StartObjView;
                    self.draw_object();
                }
            }
        }
        if ui.button(self.button_view_result_obj_label()).clicked() {
            match self.mode {
                Mode::ResultObjView => (),
                _ => {
                    self.mode = Mode::ResultObjView;
                    self.draw_object();
                }
            }
        }
    }

    fn load_obj_nested_menus(&mut self, ui: &mut Ui) {
        ui.set_max_width(150.0); // To make sure we wrap long text

        if ui.button(self.button_load_start_label()).clicked() {
            if let Some(filename) = FileDialog::new().add_filter("obj", &["obj"]).pick_file() {
                let loading = Object::load(
                    filename.to_str().unwrap_or(""),
                    Color::new(self.obj_color.to_array()),
                );
                if loading.is_ok() {
                    self.start_obj = Some(loading.unwrap());
                    self.mode = Mode::StartObjView;
                    self.draw_object();
                } else {
                    println!("{:?}", loading.err());
                }
            }
        }
        if ui.button(self.button_load_result_label()).clicked() {
            if let Some(filename) = FileDialog::new().add_filter("obj", &["obj"]).pick_file() {
                let loading = Object::load(
                    filename.to_str().unwrap_or(""),
                    Color::new(self.obj_color.to_array()),
                );
                if loading.is_ok() {
                    self.result_obj = Some(loading.unwrap());
                    self.mode = Mode::ResultObjView;
                    self.draw_object();
                } else {
                    println!("{:?}", loading.err());
                }
            }
        }
        if ui.button("Поменять объекты местами").clicked() {
            swap(&mut self.start_obj, &mut self.result_obj);
            match self.mode {
                Mode::StartObjView => self.mode = Mode::ResultObjView,
                Mode::ResultObjView => self.mode = Mode::StartObjView,
                _ => (),
            }
        }
    }
}
