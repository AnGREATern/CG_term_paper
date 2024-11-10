use super::{Mode, Painting};

impl Painting {
    pub fn button_load_start_label(&self) -> &'static str {
        if self.start_obj.is_some() {
            "Start object loaded ✅"
        } else {
            "Load start object..."
        }
    }

    pub fn button_load_result_label(&self) -> &'static str {
        if self.result_obj.is_some() {
            "Result object loaded ✅"
        } else {
            "Load result object..."
        }
    }

    pub fn button_view_start_obj_label(&self) -> &'static str {
        match self.mode {
            Mode::StartObjView => "Start object ✅",
            _ => "Start object",
        }
    }

    pub fn button_view_result_obj_label(&self) -> &'static str {
        match self.mode {
            Mode::ResultObjView => "Result object ✅",
            _ => "Result object",
        }
    }
}
