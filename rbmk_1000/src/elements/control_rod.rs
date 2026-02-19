use crate::elements::neutron::*;
use macroquad::prelude::*;
pub struct ControlRod {
    pub control_rod: Rect,
}
impl ControlRod {
    pub fn new(rod_middle_position_x: f32, core_height: f32, rod_thick: f32) -> ControlRod {
        let rod_x: f32 = rod_middle_position_x - (rod_thick / 2.0);
        let rod_y: f32 = (screen_height() / 2.0) - (core_height / 2.0);
        let control_rod: Rect = Rect::new(rod_x, rod_y, rod_thick, core_height);
        ControlRod { control_rod }
    }
    pub fn try_absorption(&self, neutron: &Neutron) -> bool {
        self.control_rod.contains(neutron.position)
    }
    pub fn draw(&self) {
        draw_rectangle(self.control_rod.x, self.control_rod.y, self.control_rod.w, self.control_rod.h,
        DARKGRAY);
    }
}