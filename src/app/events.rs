use eframe::egui::{Modifiers, PointerButton, Vec2};

use super::Painting;

impl Painting {
    pub fn mouse_moved(&mut self, pos: &Vec2) {
        if self.is_movement_access {
            self.move_object(pos);
        }
        if self.is_rotating_access {
            self.rotate_object(pos);
        }
    }

    pub fn pointer_button(&mut self, button: PointerButton, pressed: bool, modifiers: Modifiers) {
        match button {
            PointerButton::Primary => self.is_movement_access = pressed && modifiers.ctrl,
            PointerButton::Secondary => self.is_rotating_access = pressed && modifiers.ctrl,
            _ => {}
        }
    }

    pub fn mouse_wheel(&mut self, modifiers: Modifiers, delta: &Vec2) {
        if modifiers.ctrl {
            self.scale_object(delta.y);
        }
    }
}
