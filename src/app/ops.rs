use super::{Mode, Painting};

use crate::egui::Ui;
use crate::figure::merged_object::MergedObject;
use crate::figure::projection::Projection;
use crate::figure::vertex::Vertex;
use crate::{DEFAULT_SCALE, SPHERE_RADIUS};
use eframe::egui::Vec2;

impl Painting {
    pub fn morph(&mut self, ui: &mut Ui) {
        if ui.button("Start").clicked() {
            if self.start_obj.is_none() || self.result_obj.is_none() {
                return;
            }

            let start_proj = Projection::new(self.start_obj.clone().unwrap(), SPHERE_RADIUS);
            let result_proj = Projection::new(self.result_obj.clone().unwrap(), SPHERE_RADIUS);
            self.merged_obj = Some(MergedObject::new(start_proj, result_proj));
        }
    }

    pub fn move_object(&mut self, delta: &Vec2) {
        let delta = Vertex::new(
            delta.x as f64 / self.canvas.width() as f64 * DEFAULT_SCALE,
            delta.y as f64 / self.canvas.height() as f64 * DEFAULT_SCALE,
            0.,
        );
        let object = match self.mode {
            Mode::StartObjView => &mut self.start_obj,
            Mode::ResultObjView => &mut self.result_obj,
            _ => &mut None,
        };
        if let Some(object) = object {
            object.mov(delta);
            self.draw_object();
        }
    }

    pub fn rotate_object(&mut self, delta: &Vec2) {
        let delta = Vertex::new(
            -delta.y as f64 / self.canvas.height() as f64,
            delta.x as f64 / self.canvas.width() as f64,
            0.,
        );
        let object = match self.mode {
            Mode::StartObjView => &mut self.start_obj,
            Mode::ResultObjView => &mut self.result_obj,
            _ => &mut None,
        };
        if let Some(object) = object {
            object.rotate(delta);
            self.draw_object();
        }
    }

    pub fn scale_object(&mut self, delta: f32) {
        let k = (delta + 1.5) as f64;
        let object = match self.mode {
            Mode::StartObjView => &mut self.start_obj,
            Mode::ResultObjView => &mut self.result_obj,
            _ => &mut None,
        };
        if let Some(object) = object {
            object.scale(k);
            self.draw_object();
        }
    }

    pub fn draw_object(&mut self) {
        self.canvas.clear();
        let object = match self.mode {
            Mode::StartObjView => &mut self.start_obj,
            Mode::ResultObjView => &mut self.result_obj,
            _ => &mut None,
        };
        if let Some(object) = object {
            self.canvas.draw_object(object, self.light_direction);
        }
    }
}
