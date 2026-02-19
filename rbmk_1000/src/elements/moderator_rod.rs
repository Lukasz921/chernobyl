use crate::elements::neutron::*;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct ModeratorRod {
    pub moderator_rod: Rect,
}
impl ModeratorRod {
    pub fn new(rod_middle_position_x: f32, core_height: f32, rod_thick: f32) -> ModeratorRod {
        let rod_x: f32 = rod_middle_position_x - (rod_thick / 2.0);
        let rod_y: f32 = (screen_height() / 2.0) - (core_height / 2.0);
        let moderator_rod: Rect = Rect::new(rod_x, rod_y, rod_thick, core_height);
        ModeratorRod { moderator_rod }
    }
    pub fn try_neutron_collision(&self, neutron: &mut Neutron, velocity_left_percent: f32) -> bool {
        if self.moderator_rod.contains(neutron.position) && neutron.is_not_thermal() {
            if neutron.direction.x < 0.0 {
                neutron.direction.x = -neutron.direction.x;
                let path_len: f32 = self.moderator_rod.x + self.moderator_rod.w - neutron.position.x;
                let time: f32 = path_len / neutron.direction.x;
                neutron.position = Vec2::new(neutron.position.x + (neutron.direction.x * time) + 1.0,
                neutron.position.y + (neutron.direction.y * time) + 1.0);
            }
            else {
                neutron.direction.x = -neutron.direction.x;
                let path_len: f32 = neutron.position.x - self.moderator_rod.x;
                let time: f32 = path_len / neutron.direction.x.abs();
                neutron.position = Vec2::new(neutron.position.x + (neutron.direction.x * time) - 1.0,
                neutron.position.y + (neutron.direction.y * time) - 1.0);
            }
            neutron.make_thermal(velocity_left_percent);
            return true;
        }
        false
    }
    pub fn draw(&self) {
        draw_rectangle_lines(self.moderator_rod.x, self.moderator_rod.y, self.moderator_rod.w, self.moderator_rod.h,
        5.0, BLACK);
    }
}