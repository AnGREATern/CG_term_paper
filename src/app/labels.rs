use super::{Mode, Painting};

impl Painting {
    pub fn button_load_start_label(&self) -> &'static str {
        if self.start_obj.is_some() {
            "Стартовый объект загружен ✅"
        } else {
            "Загрузить стартовый объект..."
        }
    }

    pub fn button_load_result_label(&self) -> &'static str {
        if self.result_obj.is_some() {
            "Итоговый объект загружен ✅"
        } else {
            "Загрузить итоговый объект..."
        }
    }

    pub fn button_view_start_obj_label(&self) -> &'static str {
        match self.mode {
            Mode::StartObjView => "Стартовый объект ✅",
            _ => "Стартовый объект",
        }
    }

    pub fn button_view_result_obj_label(&self) -> &'static str {
        match self.mode {
            Mode::ResultObjView => "Итоговый объект ✅",
            _ => "Итоговый объект",
        }
    }
}
